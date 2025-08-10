use std::sync::Arc;

use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client, RpcApi};
use bdk_bitcoind_rpc::{Emitter, NO_EXPECTED_MEMPOOL_TXS};
use bdk_wallet::bitcoin::{Network, Transaction};
use bdk_wallet::chain::local_chain::CheckPoint;
use bdk_wallet::{AddressInfo, Balance, KeychainKind, Wallet};

const COOKIE_FILE_PATH: &str = "<path_to_your_regtest_bitcoin_core_data_dir>/.cookie";
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Regtest)
        .create_wallet_no_persist()
        .unwrap();

    let balance: Balance = wallet.balance();
    println!("Wallet balance before syncing: {}", balance.total());

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Generated address {} at index {}",
        address.address, address.index
    );

    let rpc_client: Client = Client::new(
        "http://127.0.0.1:18443",
        // Auth::UserPass("__cookie__".to_string(), "cookievalue".to_string())
        Auth::CookieFile(COOKIE_FILE_PATH.into()),
    )
    .unwrap();

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

    let mut emitter = Emitter::new(
        &rpc_client,
        wallet_tip.clone(),
        wallet_tip.height(),
        NO_EXPECTED_MEMPOOL_TXS,
    );

    println!("Syncing blocks...");
    while let Some(block) = emitter.next_block().unwrap() {
        print!("{} ", block.block_height());
        wallet
            .apply_block_connected_to(&block.block, block.block_height(), block.connected_to())
            .unwrap();
    }
    println!();

    println!("Syncing mempool...");
    let mempool_emissions: Vec<(Arc<Transaction>, u64)> = emitter.mempool().unwrap().update;
    wallet.apply_unconfirmed_txs(mempool_emissions);

    let balance: Balance = wallet.balance();
    println!("Wallet balance after syncing: {}", balance.total());
}
