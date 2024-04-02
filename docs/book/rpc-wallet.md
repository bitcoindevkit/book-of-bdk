# Wallet with Bitcoin Core RPC

!!! tip
    This page is up-to-date with version `1.0.0-alpha.8` of bdk.

### 1. Start a regtest bitcoin daemon
For this example you'll need to run a bitcoin core daemon locally in regtest mode. Here are some of the commands you'll need:
```shell
# In a shell dedicated to the bitcoin daemon 
bitcoind --chain=regtest

# In a new shell dedicated to the bitcoin-cli
bitcoin-cli --chain=regtest getblockchaininfo

bitcoin-cli --chain=regtest createwallet mywallet
bitcoin-cli --chain=regtest loadwallet mywallet
bitcoin-cli --chain=regtest getnewaddress

# Mine 101 blocks
bitcoin-cli --chain=regtest generatetoaddress 101 <address>
```

### 2. Create a new Rust project
```shell
cargo init rpcexample
cd rpcexample
```

### 3. Add required bdk dependencies to your Cargo.toml file
```toml
[package]
name = "rpcexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk = { version = "=1.0.0-alpha.8" }
bdk_file_store = { version = "=0.8.0" }
bdk_bitcoind_rpc = {  version = "0.7.0" }
```

### 4. Create your wallet
Refer to the [Working with Descriptors](./descriptors.md) page for information on how to generate descriptors. This page will assume you are working on testnet with the following BIP86 descriptor:
```txt
tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly
```

A wallet is generic in its `Store`. For example, you can create a `Wallet<()>` which will have no persistence or a `Wallet<bdk_file_store::store::Store>` which will store to a flat file. The example below uses this flat file storage system.

```rs title="Part 1: Wallet"
const DB_MAGIC: &str = "bdk_wallet_rpc_example";
const COOKIE_FILE_PATH: &str = "<path_to_your_regtest_bitcoin_core_cookie_file>/.cookie";

use bdk::wallet::{AddressIndex, AddressInfo, Balance, ChangeSet};
use bdk::bitcoin::{Block, Network, Transaction};
use bdk::{Wallet};
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
}
```

### 5. Sync the wallet

Syncing using a bitcoin core RPC client is called the block-by-block approach of syncing in BDK. You do this by creating an `Emitter` object and calling the `next_block()` method on it, which returns an `Option<BlockEvent<Block>>`. If this option is `None`, there are no new blocks to sync; if it's `Some`, you can apply this new block to your wallet by using the `wallet.apply_block_connected_to()` method. A similar workflow works for the mempool (see code below).

To get the most of this example, work with a regtest that has at least 10 to 100 blocks in it.

```rs title="Part 2: Sync"
fn main() -> () {
    
    // --- Snippet from part 1 above ---
    
    let rpc_client: Client = Client::new(
        "http://127.0.0.1:18443",
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
```

Once you have synced the wallet once, mine a few new blocks using the bitcoin-cli and send coins to the address provided by the wallet and printed in the console. Upon running the example code again, your wallet will sync up the latest blocks and update its balance.
