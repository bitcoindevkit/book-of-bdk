// detailed documentation for this code can be found at https://bitcoindevkit.github.io/book-of-bdk/cookbook/quickstart/
// --8<-- [start:file]
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::{Network, Amount};
use bdk_wallet::SignOptions;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult};
use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use bdk_wallet::keys::bip39::Mnemonic;

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;
const SEND_AMOUNT: Amount = Amount::from_sat(5000);

// const RECOVERY_PHRASE: &str = "[your 12 word seed phrase here ...]";
// const RECOVERY_PHRASE: &str = "one answer find clarify hire van aspect crystal brisk shoot rain permit"; // example
const RECOVERY_PHRASE: &str = "holiday marble tide globe license stumble rescue antenna monitor sea half sauce"; // example


fn main() -> Result<(), anyhow::Error> {
    let (mut wallet, client) = recover_wallet();

    // Reveal a new address from your external keychain
        // we will send a payment to ourselves to demonstrate tx building and broadcasting
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Transaction Logic:
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

fn recover_wallet() -> (Wallet, esplora_client::BlockingClient) {
    // see examples/rust/wallet-recovery for more details
    let mnemonic = Mnemonic::parse(RECOVERY_PHRASE).expect("Invalid seed! Be sure to replace the value of RECOVERY_PHRASE with your own 12 word seed phrase.");
    let seed = mnemonic.to_seed("");
    let xprv: Xpriv =
        Xpriv::new_master(Network::Signet, &seed).expect("Failed to create master key");
    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External)
        .build(Network::Signet)
        .expect("Failed to build external descriptor");
    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
        .build(Network::Signet)
        .expect("Failed to build internal descriptor");
    let mut wallet: Wallet = Wallet::create(descriptor.to_string_with_secret(&key_map), change_descriptor.to_string_with_secret(&change_key_map))
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