#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(unused_variables)]

const DB_MAGIC: &str = "bdk_wallet_rpc_example";
const COOKIE_FILE_PATH: &str = "<path_to_your_regtest_bitcoin_core_cookie_file>/.cookie";

use bdk::wallet::{AddressIndex, AddressInfo, Balance, ChangeSet};
use bdk::bitcoin::{Block, Network, Transaction};
use bdk::Wallet;
use bdk::chain::local_chain::CheckPoint;
use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client, RpcApi};
use bdk_bitcoind_rpc::{BlockEvent, Emitter};
use bdk_file_store::Store;

fn main() -> () {
    print_page_link("rpc-wallet/");

    let db_path = std::env::current_dir().unwrap().join("rpcexample.db");
    let db = Store::<bdk::wallet::ChangeSet>::open_or_create_new(DB_MAGIC.as_bytes(), db_path).unwrap();
    let external_descriptor = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly";

    let mut wallet: Wallet<Store<ChangeSet>> = Wallet::new_or_load(
        external_descriptor,
        None,
        db,
        Network::Regtest,
    ).unwrap();

    let balance: Balance = wallet.get_balance();
    println!("Wallet balance before syncing: {} sats", balance.total());

    let address: AddressInfo = wallet.try_get_address(AddressIndex::New).unwrap();
    println!("Generated address {} at index {}", address.address, address.index);

    let rpc_client: Client = Client::new(
        "http://127.0.0.1:18443",
        // Auth::UserPass("__cookie__".to_string(), "cookievalue".to_string())
        Auth::CookieFile(COOKIE_FILE_PATH.into())
    ).unwrap();

    let blockchain_info = rpc_client.get_blockchain_info().unwrap();
    println!(
        "\nConnected to Bitcoin Core RPC.\nChain: {}\nLatest block: {} at height {}\n",
        blockchain_info.chain, blockchain_info.best_block_hash, blockchain_info.blocks,
    );

    let wallet_tip: CheckPoint = wallet.latest_checkpoint();
    println!(
        "Current wallet tip is: {} at height {}",
        &wallet_tip.hash(),
        &wallet_tip.height()
    );

    let mut emitter = Emitter::new(&rpc_client, wallet_tip.clone(), wallet_tip.height());

    println!("Syncing blocks...");
    loop {
        let block_event: Option<BlockEvent<Block>> = emitter.next_block().unwrap();
        let block = if block_event.is_none() {
            break;
        } else {
            block_event.unwrap()
        };
        print!("{} ", block.block_height());

        wallet.apply_block_connected_to(&block.block, block.block_height(), block.connected_to()).unwrap();
        wallet.commit().unwrap();
    }
    println!();

    println!("Syncing mempool...");
    let mempool_emissions: Vec<(Transaction, u64)> = emitter.mempool().unwrap();

    wallet.apply_unconfirmed_txs(mempool_emissions.iter().map(|(tx, time)| (tx, *time)));
    wallet.commit().unwrap();

    let balance = wallet.get_balance();
    println!("Wallet balance after syncing: {} sats", balance.total());
}

fn print_page_link(link: &str) -> () {
    println!();
    println!("-------------------------------------------------------------------------------");
    println!("Companion code for https://bitcoindevkit.github.io/book-of-bdk/book/{}", link);
    println!("-------------------------------------------------------------------------------");
    println!();
}
