use bdk_kyoto::builder::{Builder, BuilderExt};
use bdk_kyoto::state::Idle;
use bdk_kyoto::wallets::Single;
use bdk_kyoto::{
    HashCheckpoint, Info, LightClient, Receiver, ScanType, TrustedPeer, UnboundedReceiver, Warning,
};
use bdk_wallet::bitcoin::params::Params;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::{KeychainKind, Wallet};
use tokio::select;

const RECEIVE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/0/*)";
const CHANGE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/1/*)";
const RECOVERY_LOOKAHEAD: u32 = 50;
const NUM_CONNECTIONS: u8 = 1;
const NETWORK: Network = Network::Regtest;

#[tokio::main]
async fn main() {
    // Initialize tracing
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Apply the recovery lookahead to the wallet
    let mut wallet = Wallet::create(RECEIVE, CHANGE)
        .network(NETWORK)
        .lookahead(RECOVERY_LOOKAHEAD)
        .create_wallet_no_persist()
        .unwrap();

    let scan_type = ScanType::Recovery {
        used_script_index: RECOVERY_LOOKAHEAD,
        checkpoint: HashCheckpoint::from_genesis(Params::REGTEST), // Signet
                                                                   // checkpoint: HashCheckpoint::new(
                                                                   //     300_000,
                                                                   //     "000000073002e4e1de008de89ee41db4baf8734c0f4e5ba9447bb0f1a301b02c"
                                                                   //         .parse::<BlockHash>()
                                                                   //         .unwrap(),
                                                                   // ),
    };

    let peer = TrustedPeer::from_hostname("127.0.0.1", 18444);

    // Build a light client and subscribe to receive log and wallet update channels.
    let idle_client: LightClient<Idle, Single> = Builder::new(NETWORK)
        .required_peers(NUM_CONNECTIONS)
        .add_peer(peer)
        .build_with_wallet(&wallet, scan_type)
        .unwrap();
    let (subscribed_client, log_subscribers, mut update_subscriber) = idle_client.subscribe();

    // managed_start() activates the client and returns the underlying node for manual task management.
    let client = subscribed_client.start();

    // Trace the logs with a custom function.
    tokio::task::spawn(async move {
        trace_logs(
            log_subscribers.info_subscriber,
            log_subscribers.warning_subscriber,
        )
        .await
    });

    // Extract a requester for shutdown later; this consumes the active client.
    let requester = client.requester();

    // Sync and apply updates to the wallet. We can do this a continual loop while the application is running.
    // Often this would occur on a separate thread than the underlying application user interface.
    let update = update_subscriber.update().await.unwrap();
    wallet.apply_update(update).unwrap();
    tracing::info!("Tx count: {}", wallet.transactions().count());
    tracing::info!("Balance: {}", wallet.balance().total().to_sat());
    let last_revealed_external = wallet.derivation_index(KeychainKind::External);
    tracing::info!("Last revealed external: {:?}", last_revealed_external);
    tracing::info!("Local chain tip: {}", wallet.local_chain().tip().height());
    let next = wallet.reveal_next_address(KeychainKind::External).address;
    tracing::info!("Next receiving address: {next}");
    requester.shutdown().unwrap();
}

/// Implement a custom logger that prints log messages to the console.
async fn trace_logs(mut info_rx: Receiver<Info>, mut warn_rx: UnboundedReceiver<Warning>) {
    loop {
        select! {
            warn = warn_rx.recv() => {
                if let Some(warn) = warn {
                    tracing::warn!("{warn}")
                }
            }
            info = info_rx.recv() => {
                if let Some(info) = info {
                    tracing::info!("{info}")
                }
            }
        }
    }
}
