use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequest, FullScanResult};

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> () {
    print_page_link("starter/");

    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Sync the wallet
    let client: esplora_client::BlockingClient = Builder::new("http://signet.bitcoindevkit.net").build_blocking();
    let full_scan_request: FullScanRequest<KeychainKind> = wallet.start_full_scan();
    let mut update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let _ = update.graph_update.update_last_seen_unconfirmed(now);

    // Apply the update from the sync to the wallet
    wallet.apply_update(update).unwrap();

    // Query the wallet balance again
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}

fn print_page_link(link: &str) -> () {
    println!();
    println!("+----------------------------------------------------------------------------------+");
    println!("|                                                                                  |");
    println!("| Companion code for https://bitcoindevkit.github.io/book-of-bdk/cookbook/{} |", link);
    println!("|                                                                                  |");
    println!("+----------------------------------------------------------------------------------+");
    println!();
}
