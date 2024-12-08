use bdk_kyoto::builder::LightClientBuilder;
use bdk_kyoto::logger::PrintLogger;
use bdk_kyoto::LightClient;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::{KeychainKind, Wallet};

const RECEIVE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/0/*)";
const CHANGE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/1/*)";
const RECOVERY_HEIGHT: u32 = 190_000;
const RECOVERY_LOOKAHEAD: u32 = 50;
const NUM_CONNECTIONS: u8 = 1;

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
    let LightClient {
        sender,
        mut receiver,
        node,
    } = LightClientBuilder::new()
        .scan_after(RECOVERY_HEIGHT)
        .connections(NUM_CONNECTIONS)
        .build(&wallet)
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
        if let Some(update) = receiver.update(&logger).await {
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
            sender.add_script(next.address).await.unwrap();
            break;
        }
    }
}
