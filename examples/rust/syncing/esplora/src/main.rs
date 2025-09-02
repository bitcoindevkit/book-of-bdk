use anyhow::Error;
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
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> Result<(), Error> {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Signet)
        .create_wallet_no_persist()?;

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Generated address {} at index {}",
        address.address, address.index
    );

    //--8<-- [start:client]
    // Create the Esplora client
    let client: esplora_client::BlockingClient =
        Builder::new("https://blockstream.info/signet/api/").build_blocking();
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

    Ok(())
}
