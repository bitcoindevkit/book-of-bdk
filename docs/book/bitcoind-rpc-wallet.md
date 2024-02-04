# Wallet with Bitcoin Core RPC

!!! tip
    This page is up-to-date with version `1.0.0-alpha.5` of bdk.

### Create a new Rust project
```shell
cargo new rpcexample
cd rpcexample
```

### Add required bdk dependencies to your Cargo.toml file
```toml
[package]
name = "rpcexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk = { version = "=1.0.0-alpha.5", features = ["all-keys"] }
bdk_bitcoind_rpc = { version = "0.4" }
```

### Setup bitcoind
<!-- Note: we currently expect the user to have bitcoind installed. Consider utilizing bdk_testenv for fetching a prebuilt binary -->
Before we begin coding, be sure you have setup `bitcoind` to run locally on Regtest by setting the following configuration in `bitcoin.conf`. To install the Bitcoin software, look for a recent version of Bitcoin Core on [bitcoincore.org](https://bitcoincore.org/en/download/), or build Bitcoin from [source](https://github.com/bitcoin/bitcoin). 

When you're ready to test the example code, remember to start the daemon with `bitcoind -regtest`. Note also it's recommended to start each run with a clean data directory.

```sh
# bitcoin.conf
server=1
daemon=1
regtest=1
rpcuser=user
rpcpassword=password
fallbackfee=0.0001
```

### Create a Wallet
For this example the descriptor is already provided. Refer to the [Working with Descriptors](./descriptors.md) page for information on how we generated a descriptor. There are a few ways to persist wallet data, but for simplicity we'll use an in-memory database using `Wallet::new_no_persist`.

```rs title="Part 1: Wallet"
use bdk_bitcoind_rpc::bitcoincore_rpc;
use bdk_bitcoind_rpc::bitcoincore_rpc::Auth;
use bdk_bitcoind_rpc::bitcoincore_rpc::Client;
use bdk_bitcoind_rpc::bitcoincore_rpc::RpcApi;
use bdk_bitcoind_rpc::Emitter;

use bdk::bitcoin::Amount;
use bdk::bitcoin::Network;

use bdk::wallet::AddressIndex;
use bdk::SignOptions;
use bdk::Wallet;

// Amount to send.
const AMOUNT: u64 = 2_000_000;

// `Result` boilerplate.
type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Create Wallet
    let external_descriptor = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly";

    let mut wallet = Wallet::new_no_persist(
        external_descriptor,
        None,
        Network::Regtest,
    )?;

    let balance = wallet.get_balance().total();
    println!("Wallet balance: {} sat", balance);

    let bdk_addr = wallet.get_address(AddressIndex::New);
    println!("Wallet new address: {}", bdk_addr);

    Ok(())
}
```

### Configure RPC client and mine blocks
Next configure the `bitcoincore_rpc::Client` using the same credentials set in `bitcoin.conf`. Then create Bitcoin Core's wallet, get a new address, and generate enough blocks for the balance to be spendable.

```rs title="Part 2: bitcoind"
fn main() -> Result<()> {
    
    // --- snippet from part 1 above ---

    // Setup Core
    let client = bitcoincore_rpc::Client::new(
        "http://127.0.0.1:18443/wallet/test",
        Auth::UserPass("user".to_string(), "password".to_string()),
    )?;

    let height = match client.get_block_count() {
        Ok(h) => h,
        Err(e) => {
            println!("unable to connect to bitcoind; caused by {e}");
            return Ok(());
        }
    };
    
    println!(
        "Connected to Bitcoin Core at block {}: {:?}",
        height,
        client.get_best_block_hash()?,
    );
    println!();

    client.create_wallet(
        "test", /*disable_private_keys: */ None, /*blank: */ None,
        /*passphrase: */ None, /*avoid_reuse: */ None,
    )?;

    let core_addr = client
        .get_new_address(/*label: */ None, /*address_type: */ None)?
        .assume_checked();

    println!("Generating blocks...");
    client.generate_to_address(101, &core_addr)?;

    let core_balance = client
        .get_balance(/*minconf: */ None, /*include_watch_only: */ None)?
        .to_btc();
    println!("Core balance: {} BTC", core_balance);

    Ok(())
}
```

### Sync Wallet
We're using `bitcoind` as the source of blockchain data. In order to sync BDK's wallet, we introduce a new type `Emitter` who will serve two primary roles: 1) polling `bitcoind` for new blocks, and 2) querying the node's mempool for unconfirmed transactions. `Emitter` will send all the transaction data it finds to the wallet which allows BDK to update its local chain and discover transactions that influence the wallet's balance. A newly constructed `Emitter` will hold a reference to the RPC client, the latest local chain checkpoint, and a block height from which to start syncing.

```rs title="Part 3: Emitter"
fn main() -> Result<()> {
    
    // --- snippet from part 1 above ---
    
    // --- snippet from part 2 above ---
    
    // Configure chain source emitter
    let start_height = 0u32;
    let mut emitter = Emitter::new(&client, wallet.latest_checkpoint(), start_height);

    Ok(())
}
```

<!-- detour to implement sync helper -->
Because we plan to sync more than once, let's put the syncing logic in its own helper function `sync`, which will take a mutable reference to `Wallet` and a mutable reference to `Emitter`, and process all emitted block events. Note, this new function can be placed anywhere outside of `main`.

```rs title="Part 4: Sync"
// Calls `Emitter::next_block`, applying transactions we care about to the wallet's
// transaction graph until all events have been consumed.
fn sync(wallet: &mut Wallet, emitter: &mut Emitter<Client>) -> Result<()> {
    print!("Syncing... ");
    while let Some(event) = emitter.next_block()? {
        let block = &event.block;
        let height = event.block_height();
        wallet.apply_block_connected_to(block, height, event.connected_to())?;
    }
    wallet.commit()?;

    println!("Ok");
    Ok(())
}
```

### Transaction building
Now we can conveniently call `sync` when we need to. In the remaining steps we fund BDK's wallet by sending a transaction from Bitcoin Core to BDK. Then we use BDK's `TxBuilder` to build a new transaction sending half of the first transaction amount back to Bitcoin Core, such that both wallets have some balance that can be seen in the final output. Feel free to add or remove `println!` statements at various points of execution.

```rs title="Part 5: Transactions"
fn main() -> Result<()> {
    
    // --- snippet from part 1 above ---
    
    // --- snippet from part 2 above ---
    
    // --- snippet from part 3 above ---
    
    // Sync (#1)
    sync(&mut wallet, &mut emitter)?;
    println!();
    
    println!("Sending 2M sat Core -> Bdk");
    client.send_to_address(
        &bdk_addr,
        Amount::from_sat(AMOUNT),
        /*comment: */ None,
        /*comment_to: */ None,
        /*subtract_fee: */ None,
        /*replaceable: */ None,
        /*confirmation_target: */ None,
        /*estimate_mode: */ None,
    )?;

    let unconfirmed = emitter.mempool()?;
    wallet.apply_unconfirmed_txs(unconfirmed.iter().map(|(tx, time)| (tx, *time)));
    wallet.commit()?;

    let balance = wallet.get_balance().untrusted_pending;
    println!("Wallet pending: {} sat", balance);

    client.generate_to_address(1, &core_addr)?;

    // Sync (#2)
    sync(&mut wallet, &mut emitter)?;

    let balance = wallet.get_balance().confirmed;
    assert_eq!(balance, AMOUNT);
    println!("Confirmed! {} sat", balance);
    println!();

    println!("Sending 1M sat Bdk -> Core");
    let mut tx_builder = wallet.build_tx();
    let amount = AMOUNT / 2;
    tx_builder.add_recipient(core_addr.script_pubkey(), amount);

    let mut psbt = tx_builder.finish()?;
    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized);

    let tx = psbt.extract_tx();
    let fee = wallet.calculate_fee(&tx)?;
    let txid = client.send_raw_transaction(&tx)?;
    println!("Txid: {}", txid);

    client.generate_to_address(1, &core_addr)?;

    // Sync (#3)
    sync(&mut wallet, &mut emitter)?;

    let core_balance = client.get_balance(None, None)?.to_btc();
    let bdk_balance = wallet.get_balance().total();
    assert!(bdk_balance < amount);
    println!("Core balance: {} BTC", core_balance);
    println!("Wallet balance: {} sat ", bdk_balance);
    println!("Network fee: {} sat", fee);

    Ok(())
}
```
