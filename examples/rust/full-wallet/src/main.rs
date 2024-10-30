use bdk_wallet::bitcoin::Address;
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::{Network, Amount};
use bdk_wallet::SignOptions;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult, SyncRequestBuilder, SyncResult};
use std::str::FromStr;
use bdk_wallet::rusqlite::Connection;

const DB_PATH: &str = "full-wallet.sqlite3";
const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;

// --8<-- [start:descriptors]
const DESCRIPTOR_PRIVATE_EXTERNAL: &str = "[your private external descriptor here ...]";
const DESCRIPTOR_PRIVATE_INTERNAL: &str = "[your private internal descriptor here ...]";
// Example private descriptors
// const DESCRIPTOR_PRIVATE_EXTERNAL: &str = "tr(tprv8ZgxMBicQKsPdJuLWWArdBsWjqDA3W5WoREnfdgKEcCQB1FMKfSoaFz9JHZU71HwXAqTsjHripkLM62kUQar14SDD8brsmhFKqVUPXGrZLc/86'/1'/0'/0/*)#fv8tutn2";
// const DESCRIPTOR_PRIVATE_INTERNAL: &str = "tr(tprv8ZgxMBicQKsPdJuLWWArdBsWjqDA3W5WoREnfdgKEcCQB1FMKfSoaFz9JHZU71HwXAqTsjHripkLM62kUQar14SDD8brsmhFKqVUPXGrZLc/86'/1'/0'/1/*)#ccz2p7rj";
// --8<-- [end:descriptors]

fn main() -> Result<(), anyhow::Error> {
    // --8<-- [start:persist]
    let mut conn = Connection::open(DB_PATH)?;

    let wallet_opt = Wallet::load()
        .descriptor(KeychainKind::External, Some(DESCRIPTOR_PRIVATE_EXTERNAL))
        .descriptor(KeychainKind::Internal, Some(DESCRIPTOR_PRIVATE_INTERNAL))
        .extract_keys()
        .check_network(Network::Signet)
        .load_wallet(&mut conn)?;

    let (mut wallet, is_new_wallet) = if let Some(loaded_wallet) = wallet_opt {
        (loaded_wallet, false)
    } else {
        (Wallet::create(DESCRIPTOR_PRIVATE_EXTERNAL, DESCRIPTOR_PRIVATE_INTERNAL)
            .network(Network::Signet)
            .create_wallet(&mut conn)?, true)
    };
    // --8<-- [end:persist]

    // --8<-- [start:scan]
    let client: esplora_client::BlockingClient = Builder::new("https://mutinynet.com/api").build_blocking();
    // Sync the wallet
    if is_new_wallet {
        // Perform a full scan
        println!("Performing full scan...");
        let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
        let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)?;
        wallet.apply_update(update).unwrap();
    } else {
        // Perform a regular sync
        println!("Performing regular sync...");
        let sync_request: SyncRequestBuilder<(KeychainKind, u32)> = wallet.start_sync_with_revealed_spks();
        let update: SyncResult = client.sync(sync_request, PARALLEL_REQUESTS)?;
        wallet.apply_update(update).unwrap();
    };
    wallet.persist(&mut conn)?;
    // --8<-- [end:scan]

    // --8<-- [start:address]
    // Reveal a new address from your external keychain
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);
    wallet.persist(&mut conn)?;
    // --8<-- [end:address]

    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());

    // --8<-- [start:faucet]
    // Use the Mutinynet faucet return address
    let address = Address::from_str("tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v")
        .unwrap()
        .require_network(Network::Signet)
        .unwrap();

    let send_amount: Amount = Amount::from_sat(5000);
    // --8<-- [end:faucet]

    // --8<-- [start:transaction]
    // Transaction Logic
    let mut tx_builder = wallet.build_tx();
    tx_builder.add_recipient(address.script_pubkey(), send_amount);

    let mut psbt = tx_builder.finish()?;
    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized);

    let tx = psbt.extract_tx()?;
    client.broadcast(&tx)?;
    println!("Tx broadcasted! Txid: {}", tx.compute_txid());
    // --8<-- [end:transaction]

    Ok(())
}
