const DB_MAGIC: &str = "bdk_wallet_esplora_example";
const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;

use std::collections::BTreeMap;
use std::io::Write;
use std::str::FromStr;

use bdk::wallet::{AddressIndex, AddressInfo, ChangeSet, Update};
use bdk::{KeychainKind, SignOptions};
use bdk::bitcoin::{Address, Network, Transaction};
use bdk::bitcoin::address::NetworkChecked;
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::chain::{ConfirmationTimeHeightAnchor, TxGraph};
use bdk::Wallet;
use bdk::chain::local_chain::CheckPoint;
use bdk_esplora::{esplora_client, EsploraExt};
use bdk_file_store::Store;

fn main() -> () {
    let db_path = std::env::current_dir().unwrap().join("esploraexample.db");
    let db = Store::<bdk::wallet::ChangeSet>::open_or_create_new(DB_MAGIC.as_bytes(), db_path).unwrap();
    let external_descriptor = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly";

    let mut wallet: Wallet<Store<ChangeSet>> = Wallet::new_or_load(
        external_descriptor,
        None,
        db,
        Network::Testnet,
    ).unwrap();

    let address: AddressInfo = wallet.try_get_address(AddressIndex::New).unwrap();
    
    println!("Generated address {} at index {}", address.address, address.index);
    // Generated address tb1p5nja3w87mc6xl5w3yy85evlg0qpyq2j4wzytazt4437nr37j2ajswm3ptl at index 0

    let client = esplora_client::Builder::new("https://esplora.testnet.kuutamo.cloud")
        .build_blocking()
        .unwrap();

    let balance = wallet.get_balance();
    println!("Wallet balance before syncing: {} sats", balance.total());

    let prev_tip: CheckPoint = wallet.latest_checkpoint();

    let keychain_spks = wallet
        .all_unbounded_spk_iters()
        .into_iter()
        .map(|(k, k_spks)| {
            let mut once = Some(());
            let mut stdout = std::io::stdout();
            let k_spks = k_spks
                .inspect(move |(spk_i, _)| match once.take() {
                    Some(_) => print!("\nScanning keychain [{:?}]", k),
                    None => print!(" {:<3}", spk_i),
                })
                .inspect(move |_| stdout.flush().expect("must flush"));
            (k, k_spks)
        })
        .collect();

    let scan_result = client.full_scan(keychain_spks, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    let (graph_update, keychain_update): (TxGraph<ConfirmationTimeHeightAnchor>, BTreeMap<KeychainKind, u32>) = scan_result;

    let missing_heights = graph_update.missing_heights(wallet.local_chain());
    let chain_update = client.update_local_chain(prev_tip, missing_heights).unwrap();

    let wallet_update = Update {
        last_active_indices: keychain_update,
        graph: graph_update,
        chain: Some(chain_update),
    };

    wallet.apply_update(wallet_update).unwrap();
    wallet.commit().unwrap();

    let balance = wallet.get_balance();
    println!("\nWallet balance after syncing: {} sats", balance.total());

    if balance.total() < 5000 {
        println!(
            "Please send at least 5000 sats to receiving address {}",
            address.address
        );
        std::process::exit(0);
    }

    let faucet_address: Address<NetworkChecked> = Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt").unwrap()
        .require_network(Network::Testnet).unwrap();

    let mut tx_builder = wallet.build_tx();
    tx_builder
        .add_recipient(faucet_address.script_pubkey(), 5000)
        .enable_rbf();

    let mut psbt: PartiallySignedTransaction = tx_builder.finish().unwrap();
    let psbt_was_finalized = wallet.sign(&mut psbt, SignOptions::default()).unwrap();
    assert!(psbt_was_finalized);

    let tx: Transaction = psbt.extract_tx();

    // Uncomment this line to broadcast the transaction
    // client.transaction_broadcast(&tx).unwrap();
    
    println!("Tx broadcasted! txid: {}", tx.txid());
}
