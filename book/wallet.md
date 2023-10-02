# Wallet with Async Explora

1. Create a new Rust project:
```shell
cargo init my_bdk_app
cd my_bdk_app
```
   
2. Add `bdk` to your `Cargo.toml` file. ~~Find the latest `BDK@1.0.0` release on [`crates.io`](https://crates.io/crates/bdk/versions)~~ (use pre-released branch for now):
```shell
cargo add bdk --git "https://github.com/notmandatory/bdk.git" --branch "test/esplora_tests"
```

3. Add other required dependencies:
```shell
cargo add bdk_esplora --git "https://github.com/notmandatory/bdk.git" --branch "test/esplora_tests"
cargo add bdk_file_store --git "https://github.com/notmandatory/bdk.git" --branch "test/esplora_tests"
cargo add tokio@1 --features "rt,rt-multi-thread,macros"
```

4. Edit `src/main.rs`, replace with below code to load or create and save new descriptors:
```rust
use std::fs::File;
use std::io::Read;
use std::string::ToString;
use std::{io::Write, str::FromStr};

use bdk::bitcoin::bip32;
use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::bitcoin::secp256k1::{rand, rand::RngCore, Secp256k1};

use bdk::{bitcoin::Network, descriptor};

use bdk::descriptor::IntoWalletDescriptor;
use bdk::keys::IntoDescriptorKey;

const CONFIG_FILE: &str = "config.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// Create and load or save new descriptors

    let secp = Secp256k1::new();
    let network = Network::Signet;

    // get descriptors from config.txt file, if file is missing create a new ones
    let descriptors = match File::open(CONFIG_FILE) {
        // load descriptors from file
        Ok(mut file) => {
            let mut config = String::new();
            file.read_to_string(&mut config)?;
            let descriptor_strings: [_; 2] = config
                .split("|")
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let external_descriptor = descriptor_strings[0]
                .into_wallet_descriptor(&secp, network)
                .unwrap();
            let internal_descriptor = descriptor_strings[1]
                .into_wallet_descriptor(&secp, network)
                .unwrap();
            (external_descriptor, internal_descriptor)
        }
        Err(_) => {
            // create new descriptors and save them to the file
            let mut seed = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut seed);
            let xprv = ExtendedPrivKey::new_master(network, &seed).unwrap();
            let bip86_external = bip32::DerivationPath::from_str("m/86'/1'/0'/0/0").unwrap();
            let bip86_internal = bip32::DerivationPath::from_str("m/86'/1'/0'/0/1").unwrap();
            let external_key = (xprv, bip86_external).into_descriptor_key().unwrap();
            let internal_key = (xprv, bip86_internal).into_descriptor_key().unwrap();
            let external_descriptor = descriptor!(tr(external_key))
                .unwrap()
                .into_wallet_descriptor(&secp, network)
                .unwrap();
            let internal_descriptor = descriptor!(tr(internal_key))
                .unwrap()
                .into_wallet_descriptor(&secp, network)
                .unwrap();
            // save descriptor strings to file
            let mut file = File::create(CONFIG_FILE).unwrap();
            println!("Created new descriptor config file: config.txt");
            let config = format!(
                "{}|{}",
                &external_descriptor
                    .0
                    .to_string_with_secret(&external_descriptor.1),
                &internal_descriptor
                    .0
                    .to_string_with_secret(&internal_descriptor.1)
            );
            file.write(config.as_bytes()).unwrap();
            (external_descriptor, internal_descriptor)
        }
    };

    let external_descriptor = descriptors.0;
    let internal_descriptor = descriptors.1;
    println!(
        "External descriptor: {}",
        &external_descriptor
            .0
            .to_string_with_secret(&external_descriptor.1)
    );
    println!(
        "Internal descriptor: {}\n",
        &internal_descriptor
            .0
            .to_string_with_secret(&internal_descriptor.1)
    );

    Ok(())
}
```

5. Add code to create a wallet and get a new address and current wallet balance:
```rust
use bdk::{bitcoin::Network, descriptor, Wallet};
use bdk::wallet::AddressIndex;
use bdk_file_store::Store;

const CHAIN_DATA_FILE: &str = "chain.dat";
const DB_MAGIC: &[u8] = "TABCONF24".as_bytes();
```

```rust
// Create a wallet and get a new address and current wallet balance

let db = Store::<bdk::wallet::ChangeSet>::new_from_path(DB_MAGIC, CHAIN_DATA_FILE)?;

// Create a new wallet
let mut wallet = Wallet::new(external_descriptor, Some(internal_descriptor), db, network)?;

// Get a new wallet address
let address = wallet.get_address(AddressIndex::New);
println!("Generated Address: {:?}", address);

// Get the wallet balance before syncing
let balance = wallet.get_balance();
println!("Wallet balance before syncing: confirmed {} sats, trusted_pending {} sats, untrusted pending {} sats", balance.confirmed, balance.trusted_pending, balance.untrusted_pending);
```

6. Add code to create an async esplora client:
```rust
use bdk_esplora::esplora_client;
```
   
```rust
// Create an async esplora client

let client = esplora_client::Builder::new("http://signet.bitcoindevkit.net").build_async()?;
let prev_tip = wallet.latest_checkpoint();
```

7. Add code to scans keychain SPKs for transaction histories, stopping after `stop_gap` is reached:
```rust
use std::collections::BTreeMap;
use std::{io, io::Write, str::FromStr};
use bdk::chain::keychain::WalletUpdate;
use bdk::{bitcoin::Network, descriptor, KeychainKind, Wallet};
use bdk_esplora::{esplora_client, EsploraAsyncExt};

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 5;
```
   
```rust
// Prepare the `IndexedTxGraph` update based on whether we are scanning or syncing.

// Scanning: We are iterating through spks of all keychains and scanning for transactions for
//   each spk. We start with the lowest derivation index spk and stop scanning after `stop_gap`
//   number of consecutive spks have no transaction history. A Scan is done in situations of
//   wallet restoration. It is a special case. Applications should use "sync" style updates
//   after an initial scan.
if prompt("Scan wallet") {
    let keychain_spks = wallet
        .spks_of_all_keychains()
        .into_iter()
        // This `map` is purely for logging.
        .map(|(keychain, iter)| {
            let mut first = true;
            let spk_iter = iter.inspect(move |(i, _)| {
                if first {
                    // TODO impl Display for Keychain
                    eprint!(
                        "\nscanning {}: ",
                        match keychain {
                            KeychainKind::External => "External",
                            KeychainKind::Internal => "Internal",
                        }
                    );
                    first = false;
                }
                eprint!("{} ", i);
                // Flush early to ensure we print at every iteration.
                let _ = io::stderr().flush();
            });
            (keychain, spk_iter)
        })
        .collect::<BTreeMap<_, _>>();

    // The client scans keychain spks for transaction histories, stopping after `stop_gap`
    // is reached. It returns a `TxGraph` update (`graph_update`) and a structure that
    // represents the last active spk derivation indices of keychains
    // (`keychain_indices_update`).
    let (graph_update, last_active_indices) = client
        .update_tx_graph(
            keychain_spks,
            core::iter::empty(),
            core::iter::empty(),
            STOP_GAP,
            PARALLEL_REQUESTS,
        )
        .await?;

    println!();
    let missing_heights = wallet.tx_graph().missing_heights(wallet.local_chain());
    let chain_update = client
        .update_local_chain(prev_tip.clone(), missing_heights)
        .await?;

    let update = WalletUpdate {
        last_active_indices,
        graph: graph_update,
        chain: chain_update,
    };
    wallet.apply_update(update)?;
    wallet.commit()?;
    println!("Scan completed.");

    let balance = wallet.get_balance();
    println!("Wallet balance after scanning: confirmed {} sats, trusted_pending {} sats, untrusted pending {} sats",
                balance.confirmed, balance.trusted_pending, balance.untrusted_pending);
}
```

8. Add code to sync wallet by checking for history on all derived SPKs:
```rust
use bdk::bitcoin::{bip32, Address, OutPoint, ScriptBuf, Txid};
```
   
```rust
// Syncing: We only check for specified spks, utxos and txids to update their confirmation
//   status or fetch missing transactions.
else {
    // Spks, outpoints and txids we want updates on will be accumulated here.
    let mut spks: Box<Vec<ScriptBuf>> = Box::new(Vec::new());
    let mut outpoints: Box<dyn Iterator<Item = OutPoint> + Send> =
        Box::new(core::iter::empty());
    let mut txids: Box<dyn Iterator<Item = Txid> + Send> = Box::new(core::iter::empty());

    // Sync all SPKs
    if prompt("Sync all SPKs") {
        // TODO add Wallet::all_spks() function, gives all tracked spks
        let all_spks: Vec<ScriptBuf> = wallet
            .spk_index()
            .all_spks()
            .into_iter()
            .map(|((keychain, index), script)| {
                eprintln!(
                    "Checking if keychain: {}, index: {}, address: {} has been used",
                    match keychain {
                        KeychainKind::External => "External",
                        KeychainKind::Internal => "Internal",
                    },
                    index,
                    Address::from_script(script.as_script(), network).unwrap(),
                );
                // Flush early to ensure we print at every iteration.
                let _ = io::stderr().flush();
                (*script).clone()
            })
            .collect();
        spks = Box::new(all_spks);
    }

    let graph_update = client
        .update_tx_graph_without_keychain(spks.into_iter(), txids, outpoints, PARALLEL_REQUESTS)
        .await?;

    let missing_heights = wallet.tx_graph().missing_heights(wallet.local_chain());
    let chain_update = client.update_local_chain(prev_tip, missing_heights).await?;

    let update = WalletUpdate {
        // no update to active indices
        last_active_indices: BTreeMap::new(),
        graph: graph_update,
        chain: chain_update,
    };
    wallet.apply_update(update)?;
    wallet.commit()?;
    println!("Sync completed.");

    let balance = wallet.get_balance();
    println!("Wallet balance after syncing: confirmed {} sats, trusted_pending {} sats, untrusted pending {} sats",
                balance.confirmed, balance.trusted_pending, balance.untrusted_pending);
}
```

9. Add code to sync wallet by checking for history on only unused SPKs:
```rust
    // Sync only unused SPKs
    else if prompt("Sync only unused SPKs") {
        // TODO add Wallet::unused_spks() function, gives all unused tracked spks
        let unused_spks: Vec<ScriptBuf> = wallet
            .spk_index()
            .unused_spks(..)
            .into_iter()
            .map(|((keychain, index), script)| {
                eprintln!(
                    "Checking if keychain: {}, index: {}, address: {} has been used",
                    match keychain {
                        KeychainKind::External => "External",
                        KeychainKind::Internal => "Internal",
                    },
                    index,
                    Address::from_script(script, network).unwrap(),
                );
                // Flush early to ensure we print at every iteration.
                let _ = io::stderr().flush();
                ScriptBuf::from(script)
            })
            .collect();
        spks = Box::new(unused_spks);
    }
```

10. Add code to sync wallet UTXOs to see if any have been spent:
```rust
    // Sync UTXOs
    if prompt("Sync UTXOs") {
        // We want to search for whether the UTXO is spent, and spent by which
        // transaction. We provide the outpoint of the UTXO to
        // `EsploraExt::update_tx_graph_without_keychain`.
        let utxo_outpoints = wallet
            .list_unspent()
            .inspect(|utxo| {
                eprintln!(
                    "Checking if outpoint {} (value: {}) has been spent",
                    utxo.outpoint, utxo.txout.value
                );
                // Flush early to ensure we print at every iteration.
                let _ = io::stderr().flush();
            })
            .map(|utxo| utxo.outpoint);
        outpoints = Box::new(utxo_outpoints);
    };
```

11. Add code to sync wallet unconfirmed TXs:
```rust
    // Sync unconfirmed TX
    if prompt("Sync unconfirmed TX") {
        // We want to search for whether the unconfirmed transaction is now confirmed.
        // We provide the unconfirmed txids to
        // `EsploraExt::update_tx_graph_without_keychain`.
        let unconfirmed_txids = wallet
            .transactions()
            .filter(|canonical_tx| !canonical_tx.chain_position.is_confirmed())
            .map(|canonical_tx| canonical_tx.tx_node.txid)
            .inspect(|txid| {
                eprintln!("Checking if {} is confirmed yet", txid);
                // Flush early to ensure we print at every iteration.
                let _ = io::stderr().flush();
            });
        txids = Box::new(unconfirmed_txids);
    }
```
12. Add code to check the new wallet balance and request a deposit if required:
```rust
const SEND_AMOUNT: u64 = 5000;
```
   
```rust
// Check balance and request deposit if required
if balance.total() < SEND_AMOUNT {
    println!(
        "Please send at least {} sats to {} using: https://signetfaucet.com/",
        SEND_AMOUNT, address.address
    );
    std::process::exit(0);
}
```

13. Add code to create a TX to return sats to the [signet faucet](https://signetfaucet.com/):
```rust
// Create TX to return sats to signet faucet https://signetfaucet.com/
let faucet_address = Address::from_str("tb1qg3lau83hm9e9tdvzr5k7aqtw3uv0dwkfct4xdn")?
    .require_network(network)?;

let mut tx_builder = wallet.build_tx();
tx_builder
    .add_recipient(faucet_address.script_pubkey(), SEND_AMOUNT)
    // .drain_to(faucet_address.script_pubkey())
    // .drain_wallet()
    .fee_rate(FeeRate::from_sat_per_vb(2.1))
    .enable_rbf();

let mut psbt = tx_builder.finish()?;
let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
assert!(finalized);

let tx = psbt.extract_tx();
let (sent, received) = wallet.sent_and_received(&tx);
let fee = wallet.calculate_fee(&tx).expect("fee");
let fee_rate = wallet
    .calculate_fee_rate(&tx)
    .expect("fee rate")
    .as_sat_per_vb();
println!(
    "Created tx sending {} sats to {}",
    sent - received - fee,
    faucet_address
);
println!(
    "Fee is {} sats, fee rate is {:.2} sats/vbyte",
    fee, fee_rate
);

if prompt("Broadcast") {
    client.broadcast(&tx).await?;
    println!(
        "Tx broadcast! https://mempool.space/signet/tx/{}",
        tx.txid()
    );
}
```
