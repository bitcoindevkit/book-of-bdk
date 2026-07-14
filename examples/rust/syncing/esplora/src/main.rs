use anyhow::{Context, Error};
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::{esplora_client, EsploraExt};
use bdk_wallet::bitcoin::Network;
use bdk_wallet::chain::spk_client::{
    FullScanRequestBuilder, FullScanResponse, SyncRequestBuilder, SyncResponse,
};
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::Wallet;

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;
const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/84'/1'/0'/0/*)#5cskkptg";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/84'/1'/0'/1/*)#9v4ht5ms";

fn main() -> Result<(), Error> {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Regtest)
        .create_wallet_no_persist()
        .context("failed to create wallet")?;

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Generated address {} at index {}",
        address.address, address.index
    );

    //--8<-- [start:client]
    let esplora_url =
        std::env::var("ESPLORA_URL").unwrap_or_else(|_| "http://127.0.0.1:3002".to_owned());
    let client: esplora_client::BlockingClient = Builder::new(&esplora_url).build_blocking();
    //--8<-- [end:client]

    //--8<-- [start:scan]
    // Full scan the wallet
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let full_scan_response: FullScanResponse<KeychainKind> =
        client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)?;

    // Apply the full scan response to the wallet
    wallet.apply_update(full_scan_response)?;
    //--8<-- [end:scan]

    //--8<-- [start:sync]
    // Sync the wallet
    let sync_request: SyncRequestBuilder<(KeychainKind, u32)> =
        wallet.start_sync_with_revealed_spks();
    let sync_response: SyncResponse = client.sync(sync_request, PARALLEL_REQUESTS)?;

    // Apply the sync response to the wallet
    wallet.apply_update(sync_response)?;
    //--8<-- [end:sync]

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
