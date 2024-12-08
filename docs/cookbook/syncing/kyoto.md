# Sync a wallet with Kyoto

!!! tip 
    This page is up-to-date with version `1.0.0-beta.5` of bdk.

[BIP157](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki) and [BIP158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki) define a protocol for light clients to sync with the Bitcoin network without downloading the entire set of blocks in the chain of most work. These proposals define _compact block filters_, which allow a client to download a small commitment
for the scripts contained in each block. These commitments, or filters, may be checked for inclusion of scripts owned by a user. In the event of a match, the light client may download and verify a block indeed contains a relevant transaction. Syncing via compact block filters offers privacy advantages over other chain sources, as the nodes serving the blocks to the client are only aware that the client is interested in an entire block, which may contain thousands of transactions.

One such implementation of this protocol is [Kyoto](https://github.com/rustaceanrob/kyoto), which is a node and client for compact block filter based syncing. The [`bdk_kyoto`](https://github.com/bitcoindevkit/bdk-kyoto) crate supports an integration between Kyoto and `bdk_wallet`, so developers using `bdk_wallet` have a simple option to provide privacy-preserving and memory-conservative wallet syncing for their users.

The following example uses the `bdk_kyoto` crate to recover and update a `bdk_wallet` using compact block filters.

### Add required bdk dependencies to your `Cargo.toml` file

```toml
bdk_kyoto = { version = "0.4.0", default-features = false, features = ["rusqlite", "wallet", "callbacks"] }
bdk_wallet = { version = "1.0.0-beta.5" }
tokio = { version = "1.37", features = ["full"], default-features = false }
```

### Create and sync the wallet

```rust
use bdk_kyoto::builder::LightClientBuilder;
use bdk_kyoto::logger::PrintLogger;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::{KeychainKind, Wallet};

const RECEIVE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/0/*)";
const CHANGE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/1/*)";
const RECOVERY_HEIGHT: u32 = 190_000;
const RECOVERY_LOOKAHEAD: u32 = 50;

#[tokio::main]
async fn main() {
    // Apply the recovery lookahead to the wallet
    let mut wallet = Wallet::create(RECEIVE, CHANGE)
        .network(Network::Signet)
        .lookahead(RECOVERY_LOOKAHEAD)
        .create_wallet_no_persist()
        .unwrap();

    // Build a node that will find and connect to peers, gather relevant blocks, and broadcast transacitons.
    // In addition, receive a client that allows for communication with a running node to receive wallet
    // updates, relay transactions to the node, and get updates on the node's actions.
    let (node, mut client) = LightClientBuilder::new(&wallet)
        .scan_after(RECOVERY_HEIGHT)
        .build()
        .unwrap();

    // Run the node on a separate task. The node will run continuously until instructed by the client
    // to stop. The node will attempt to stay in sync with its peers by listening for messages as long
    // as the application is running.
    tokio::task::spawn(async move { node.run().await });

    // Print logs to the console.
    let logger = PrintLogger::new();

    // Sync and apply updates to the wallet. We can do this a continual loop while the application is running.
    // Often this would occur on a separate thread than the underlying application user interface.
    loop {
        // Wait for an update from the client, if there is one. Intermediate logs and warnings
        // are handled by the `PrintLogger`. Note that `PrintLogger` implements `NodeEventHandler`.
        // A production application would likely implement custom behavior by implementing 
        // a novel `NodeEventHandler`.
        if let Some(update) = client.update(&logger).await {
            wallet.apply_update(update).unwrap();
            println!("Tx count: {}", wallet.transactions().count());
            println!("Balance: {}", wallet.balance().total().to_sat());
            let last_revealed = wallet.derivation_index(KeychainKind::External).unwrap();
            println!("Last revealed External: {}", last_revealed);
            println!(
                "Last revealed Internal: {}",
                wallet.derivation_index(KeychainKind::Internal).unwrap()
            );
            println!("Local chain tip: {}", wallet.local_chain().tip().height());
            let next = wallet.peek_address(KeychainKind::External, last_revealed + 1);
            println!("Next receiving address: {next}");
            client.add_script(next.address).await.unwrap();
            break;
        }
    }
}
```

### A note on recoveries, sync and full-scan

The entire set of scripts is checked against each block filter as new blocks are gossiped to the Kyoto node. Because the scripts are not checked iteratively, there is not a semantic difference between "sync" and "full scan". Rather, Kyoto is made aware of the `lookahead` number of scripts ahead of the last revealed index for each keychain in the wallet when the node is built. To recover a wallet, the `lookahead` should be set to a number greater than or equal to the number of scripts revealed by the wallet. Developers can and should add scripts to check for filter inclusions by calling `add_script` when transactions are built or addresses are revealed.