// detailed documentation for this code can be found at https://bitcoindevkit.github.io/book-of-bdk/cookbook/quickstart/
// --8<-- [start:file]
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult};

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;

fn main() -> () {

    // --8<-- [start:descriptors]
    let descriptor: &str = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m";
    let change_descriptor: &str = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr";
    // --8<-- [end:descriptors]

    // Create the wallet
    let mut wallet: Wallet = Wallet::create(descriptor, change_descriptor)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    // Reveal a new address from your external keychain
        // doing this just to show it is an HD wallet 
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Sync the wallet
    // --8<-- [start:client]
    let client: esplora_client::BlockingClient = Builder::new("http://signet.bitcoindevkit.net").build_blocking();
    // --8<-- [end:client]
    
    // --8<-- [start:scan]
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
    // --8<-- [end:scan]
}
// --8<-- [end:file]