# Sync a Wallet with Bitcoin Core RPC

!!! tip
    This page is up-to-date with version `1.0.0-beta.1` of bdk.

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

# Send to address
bitcoin-cli --chain=regtest sendtoaddress <address> <amount>
```

### 2. Create a new Rust project
```shell
cargo init rpcexample
cd rpcexample
```

### 3. Add required bdk dependencies to your `Cargo.toml` file
```toml
[package]
name = "rpcexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk = { version = "=1.0.0-beta.1" }
bdk_bitcoind_rpc = {  version = "0.13.0" }
```

### 4. Create your descriptors
Refer to the [Working with Descriptors](../keys-descriptors/descriptors.md) page for information on how to generate descriptors. This page will assume you are working on Regtest with the following BIP86 descriptors:
```txt
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";
```

### 5. Create and sync wallet

```rust
use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client, RpcApi};
use bdk_bitcoind_rpc::Emitter;
use bdk_wallet::bitcoin::{Network, Transaction};
use bdk_wallet::chain::local_chain::CheckPoint;
use bdk_wallet::{AddressInfo, Balance, KeychainKind, Wallet};

const COOKIE_FILE_PATH: &str = "<path_to_your_regtest_bitcoin_core_data_dir>/.cookie";
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> () {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Regtest)
        .create_wallet_no_persist()
        .unwrap();

    let balance: Balance = wallet.balance();
    println!("Wallet balance before syncing: {}", balance.total());

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
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
    while let Some(block) = emitter.next_block().unwrap() {
        // the `block` variable is of type `BlockEvent<Block>`
        print!("{} ", block.block_height());

        wallet
            .apply_block_connected_to(&block.block, block.block_height(), block.connected_to())
            .unwrap();
    }
    println!();

    println!("Syncing mempool...");
    let mempool_emissions: Vec<(Transaction, u64)> = emitter.mempool().unwrap();

    wallet.apply_unconfirmed_txs(mempool_emissions.iter().map(|(tx, time)| (tx, *time)));

    let balance: Balance = wallet.balance();
    println!("Wallet balance after syncing: {}", balance.total());
}
```

Once you have synced the wallet once, mine a few new blocks using the bitcoin-cli and send coins to the address provided by the wallet and printed in the console. Upon running the example code again, your wallet will sync up the latest blocks and update its balance.
