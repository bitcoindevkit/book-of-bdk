use bdk_kyoto::builder::LightClientBuilder;
use bdk_kyoto::{LightClient, LogSubscriber, WarningSubscriber};
use bdk_wallet::bitcoin::Network;
use bdk_wallet::{KeychainKind, Wallet};
use tokio::select;

const RECEIVE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/0/*)";
const CHANGE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/1/*)";
const RECOVERY_HEIGHT: u32 = 190_000;
const RECOVERY_LOOKAHEAD: u32 = 50;
const NUM_CONNECTIONS: u8 = 1;

/// Implement a custom logger that prints log messages to the console.
async fn trace_logs(mut log_subscriber: LogSubscriber, mut warning_subscriber: WarningSubscriber) {
    loop {
        select! {
            log = log_subscriber.next_log() => {
                tracing::info!("{log}")
            }
            warn = warning_subscriber.next_warning() => {
                tracing::warn!("{warn}")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Apply the recovery lookahead to the wallet
    let mut wallet = Wallet::create(RECEIVE, CHANGE)
        .network(Network::Signet)
        .lookahead(RECOVERY_LOOKAHEAD)
        .create_wallet_no_persist()
        .unwrap();

    // Build a node that will find and connect to peers, gather relevant blocks, and broadcast transactions.
    // In addition, receive a client that allows for communication with a running node to receive wallet
    // updates, relay transactions to the node, and get updates on the node's actions.
    let LightClient {
        requester,
        log_subscriber,
        warning_subscriber,
        mut update_subscriber,
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

    // Trace the logs with a custom function.
    tokio::task::spawn(async move { trace_logs(log_subscriber, warning_subscriber).await });

    // Sync and apply updates to the wallet. We can do this a continual loop while the application is running.
    // Often this would occur on a separate thread than the underlying application user interface.
    loop {
        // Wait for an update from the client, if there is one.
        if let Some(update) = update_subscriber.update().await {
            wallet.apply_update(update).unwrap();
            tracing::info!("Tx count: {}", wallet.transactions().count());
            tracing::info!("Balance: {}", wallet.balance().total().to_sat());
            let last_revealed = wallet.derivation_index(KeychainKind::External).unwrap();
            tracing::info!("Last revealed External: {}", last_revealed);
            tracing::info!(
                "Last revealed Internal: {}",
                wallet.derivation_index(KeychainKind::Internal).unwrap()
            );
            tracing::info!("Local chain tip: {}", wallet.local_chain().tip().height());
            let next = wallet.peek_address(KeychainKind::External, last_revealed + 1);
            tracing::info!("Next receiving address: {next}");
            requester.add_script(next.address).await.unwrap();
            break;
        }
    }
}
