// Detailed documentation for this code can be found at https://bitcoindevkit.github.io/book-of-bdk/cookbook/starter-example/

use bdk_esplora::esplora_client;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::EsploraExt;
use bdk_wallet::bitcoin::{Address, Amount, FeeRate, Network, Psbt};
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResponse};
use bdk_wallet::rusqlite::Connection;
#[allow(deprecated)]
use bdk_wallet::SignOptions;
use bdk_wallet::Wallet;
use bdk_wallet::{AddressInfo, KeychainKind};
use std::process::exit;
use std::str::FromStr;

const STOP_GAP: usize = 20;
const PARALLEL_REQUESTS: usize = 1;
const DB_PATH: &str = "starter.sqlite3";

// The codeblocks in https://bookofbdk.com pull their code from these examples. Since we do not want an indent on the 
// codeblocks on the website, we also remove the indents here.

#[allow(deprecated)]
fn main() {
    println!("\nWelcome to the Book of BDK Starter Example Wallet!");
// --8<-- [start:descriptors]
let descriptor: &str = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m";
let change_descriptor: &str = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr";
// --8<-- [end:descriptors]

// --8<-- [start:wallet]
// Initiate the connection to the database
let mut conn = Connection::open(DB_PATH).expect("Can't open database");

// Create the wallet
let wallet_opt = Wallet::load()
    .descriptor(KeychainKind::External, Some(descriptor))
    .descriptor(KeychainKind::Internal, Some(change_descriptor))
    // .extract_keys() // uncomment this line when using private descriptors
    .check_network(Network::Signet)
    .load_wallet(&mut conn)
    .unwrap();

let mut wallet = if let Some(loaded_wallet) = wallet_opt {
    loaded_wallet
} else {
    Wallet::create(descriptor, change_descriptor)
        .network(Network::Signet)
        .create_wallet(&mut conn)
        .unwrap()
};
// --8<-- [end:wallet]

// --8<-- [start:client]
// Sync the wallet
let client: esplora_client::BlockingClient =
    Builder::new("https://blockstream.info/signet/api/").build_blocking();

println!("Syncing wallet...");
let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
let update: FullScanResponse<KeychainKind> = client
    .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
    .unwrap();

// Apply the update from the full scan to the wallet
wallet.apply_update(update).unwrap();

let balance = wallet.balance();
println!("Wallet balance: {} sat", balance.total().to_sat());
// --8<-- [end:client]

// --8<-- [start:address]
if balance.total().to_sat() < 5000 {
    println!("Your wallet does not have sufficient balance for the following steps!");
    // Reveal a new address from your external keychain
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Send Signet coins to {} (address generated at index {})",
        address.address, address.index
    );
    wallet.persist(&mut conn).expect("Cannot persist");
    exit(0)
}
// --8<-- [end:address]

// --8<-- [start:recipient]
// Use a faucet return address
let faucet_address =
    Address::from_str("tb1p4tp4l6glyr2gs94neqcpr5gha7344nfyznfkc8szkreflscsdkgqsdent4")
        .unwrap()
        .require_network(Network::Signet)
        .unwrap();

let send_amount: Amount = Amount::from_sat(1000);
// --8<-- [end:recipient]

// --8<-- [start:transaction]
let mut builder = wallet.build_tx();
builder
    .fee_rate(FeeRate::from_sat_per_vb(4).unwrap())
    .add_recipient(faucet_address.script_pubkey(), send_amount);

let mut psbt: Psbt = builder.finish().unwrap();

let finalized = wallet.sign(&mut psbt, SignOptions::default()).unwrap();
assert!(finalized);

let tx = psbt.extract_tx().unwrap();
client.broadcast(&tx).unwrap();
println!("Transaction broadcast! Txid: {}", tx.compute_txid());
// --8<-- [end:transaction]
}
