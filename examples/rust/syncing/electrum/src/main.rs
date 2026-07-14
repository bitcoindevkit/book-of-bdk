use anyhow::Context;
use bdk_electrum::electrum_client::Client;
use bdk_electrum::{electrum_client, BdkElectrumClient};
use bdk_wallet::bitcoin::Network;
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::Wallet;

const STOP_GAP: usize = 50;
const BATCH_SIZE: usize = 5;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> Result<(), anyhow::Error> {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Regtest)
        .create_wallet_no_persist()
        .context("failed to create wallet")?;

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Generated address {} at index {}",
        address.address, address.index
    );

    let electrum_url =
        std::env::var("ELECTRUM_URL").unwrap_or_else(|_| "tcp://127.0.0.1:60401".to_owned());
    let client: BdkElectrumClient<Client> = BdkElectrumClient::new(
        electrum_client::Client::new(&electrum_url)
            .context("failed to create Electrum client; is the Regtest environment running?")?,
    );

    // Perform the initial full scan on the wallet
    let full_scan_request = wallet.start_full_scan();
    let update = client.full_scan(full_scan_request, STOP_GAP, BATCH_SIZE, true)?;

    wallet.apply_update(update)?;
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());

    if let Ok(expected_address) = std::env::var("EXPECTED_RECEIVE_ADDRESS") {
        assert_eq!(
            address.address.to_string(),
            expected_address,
            "derived address {} does not match expected {}",
            address.address,
            expected_address,
        );
        println!("Address matches expected value");
    }

    if let Ok(expected_sats) = std::env::var("EXPECTED_BALANCE_SATS") {
        let expected: u64 = expected_sats
            .parse()
            .context("invalid EXPECTED_BALANCE_SATS, expected a number")?;
        let actual = balance.total().to_sat();
        assert_eq!(
            actual, expected,
            "expected balance of {} sats but wallet has {} sats",
            expected, actual,
        );
        println!("Balance matches expected value");
    }

    Ok(())
}
