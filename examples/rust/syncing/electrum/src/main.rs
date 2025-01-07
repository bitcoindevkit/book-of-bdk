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

fn main() -> () {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Generated address {} at index {}",
        address.address, address.index
    );

    // Create the Electrum client
    let client: BdkElectrumClient<Client> =
        BdkElectrumClient::new(electrum_client::Client::new("ssl://mempool.space:60602").unwrap());

    // Perform the initial full scan on the wallet
    let full_scan_request = wallet.start_full_scan();
    let update = client
        .full_scan(full_scan_request, STOP_GAP, BATCH_SIZE, true)
        .unwrap();

    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}
