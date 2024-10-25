// detailed documentation for this code can be found at https://bitcoindevkit.github.io/book-of-bdk/cookbook/quickstart/
// --8<-- [start:file]
use bdk_wallet::bitcoin::Address;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::{Network, Amount};
use bdk_wallet::SignOptions;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult};

use std::str::FromStr;

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;
const SEND_AMOUNT: Amount = Amount::from_sat(5000);

// --8<-- [start:main]
const DESCRIPTOR_PRIVATE_EXTERNAL: &str = "[your private external descriptor here ...]";
const DESCRIPTOR_PRIVATE_INTERNAL: &str = "[your private internal descriptor here ...]";

fn main() -> Result<(), anyhow::Error> {
    let (mut wallet, client) = recover_wallet();

    // Use the Mutinynet faucet return address
    let address = Address::from_str("tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v")
        .unwrap()
        .require_network(Network::Signet)
        .unwrap();

    // Transaction Logic
    let mut tx_builder = wallet.build_tx();
    tx_builder.add_recipient(address.script_pubkey(), SEND_AMOUNT);

    let mut psbt = tx_builder.finish()?;
    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized);

    let tx = psbt.extract_tx()?;
    client.broadcast(&tx)?;
    println!("Tx broadcasted! Txid: {}", tx.compute_txid());

    Ok(())
}
// --8<-- [end:main]

fn recover_wallet() -> (Wallet, esplora_client::BlockingClient) {
    // see examples/rust/quickstart for more details
    let mut wallet: Wallet = Wallet::create(DESCRIPTOR_PRIVATE_EXTERNAL, DESCRIPTOR_PRIVATE_INTERNAL)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();
    println!("Syncing wallet...");
    let client: esplora_client::BlockingClient = Builder::new("https://mutinynet.com/api").build_blocking();
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
    (wallet, client)
}
// --8<-- [end:file]
