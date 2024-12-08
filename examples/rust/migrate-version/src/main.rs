// --8<-- [start:use]
use anyhow::Context;

use bdk::database::SqliteDatabase;
use bdk::wallet::AddressIndex;

use bdk_wallet::bitcoin::Network;
use bdk_wallet::rusqlite;
use bdk_wallet::KeychainKind;
use bdk_wallet::Wallet;
// --8<-- [end:use]

// --8<-- [start:setup]
const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/0/*)";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/1/*)";
const NETWORK: Network = Network::Testnet;

// path to old db
const BDK_DB_PATH: &str = "./.bdk-example.sqlite";
// path to new db
const BDK_WALLET_DB_PATH: &str = "./.bdk-wallet-example.sqlite";
// --8<-- [end:setup]

// Steps for migrating wallet state from the old `bdk` 0.30 to the new `bdk_wallet` 1.0.

// To run: change `BDK_DB_PATH` to point to the location of the old database file and
// modify the descriptors and network above to fit your setup. Before running, there
// should not be any persisted data at the new path `BDK_WALLET_DB_PATH`.

// --8<-- [start:main]
fn main() -> anyhow::Result<()> {
    // --8<-- [start:old]
    // Open old wallet
    let db = SqliteDatabase::new(BDK_DB_PATH);
    let old_wallet = bdk::Wallet::new(
        EXTERNAL_DESCRIPTOR,
        Some(INTERNAL_DESCRIPTOR),
        bdk::bitcoin::Network::Testnet,
        db,
    )?;

    // Get last revealed addresses for each keychain
    let addr = old_wallet.get_address(AddressIndex::LastUnused)?;
    println!("Last revealed external {} {}", addr.index, addr.address);
    let external_derivation_index = addr.index;
    let last_revealed_external = addr.address.to_string();

    let addr = old_wallet.get_internal_address(AddressIndex::LastUnused)?;
    println!("Last revealed internal {} {}", addr.index, addr.address);
    let internal_derivation_index = addr.index;
    let last_revealed_internal = addr.address.to_string();
    // --8<-- [end:old]

    // --8<-- [start:new]
    // Create new wallet
    let mut db = rusqlite::Connection::open(BDK_WALLET_DB_PATH)?;
    let mut new_wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(NETWORK)
        .create_wallet(&mut db)
        .context("failed to create wallet")?;

    // Retore revealed addresses
    let _ = new_wallet.reveal_addresses_to(KeychainKind::External, external_derivation_index);
    let _ = new_wallet.reveal_addresses_to(KeychainKind::Internal, internal_derivation_index);

    // Persist new wallet
    new_wallet.persist(&mut db)?;

    println!("\n========== New database created. ==========");

    let addr = new_wallet
        .list_unused_addresses(KeychainKind::External)
        .last()
        .unwrap();
    assert_eq!(addr.to_string(), last_revealed_external);
    println!("Last revealed external {} {}", addr.index, addr.address);
    let addr = new_wallet
        .list_unused_addresses(KeychainKind::Internal)
        .last()
        .unwrap();
    println!("Last revealed internal {} {}", addr.index, addr.address);
    assert_eq!(addr.to_string(), last_revealed_internal);
    // --8<-- [end:new]

    Ok(())
}
// --8<-- [end:main]

/* Extra: sync with esplora

// --8<-- [start:sync]
use bdk_esplora::{esplora_client, EsploraExt};

let client = esplora_client::Builder::new(ESPLORA_URL).build_blocking();

let request = wallet
    .start_sync_with_revealed_spks()
    .inspect(|item, prog| {
        if let SyncItem::Spk(index, script) = item {
            let address = Address::from_script(script, NETWORK).unwrap();
            let progress = prog.consumed() as f32 / prog.total() as f32;
            eprintln!("[ SYNCING {:.2}% ] {:?} {}", 100.0 * progress, index, address);
            std::io::stdout().flush().unwrap();
        }
    });

let update = client.sync(request, PARALLEL_REQUESTS)?;

wallet.apply_update(update)?;
wallet.persist(&mut db)?;
// --8<-- [end:sync]
*/
