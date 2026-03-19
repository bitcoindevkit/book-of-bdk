use anyhow::Context;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::rusqlite;
use bdk_wallet::rusqlite::OpenFlags;
use bdk_wallet::KeychainKind::{self, External, Internal};
use bdk_wallet::Wallet;

// --8<-- [start:setup]
const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdufyJrFBAzSzoC5ANzovUKZ76md8EHq6hFEsVBv9SpgqaetP1WkD18VqF1xWza8kQGNtFZkNDuCDyXDMyNpLVJ7QXTqeiGG/84'/1'/0'/0/*)#72k0lrja";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdufyJrFBAzSzoC5ANzovUKZ76md8EHq6hFEsVBv9SpgqaetP1WkD18VqF1xWza8kQGNtFZkNDuCDyXDMyNpLVJ7QXTqeiGG/84'/1'/0'/1/*)#07nwzkz9";
const NETWORK: Network = Network::Testnet;

// path to old pre1 db
const BDK_DB_PATH: &str = "./bdk-example.sqlite";
// path to new db
const BDK_WALLET_DB_PATH: &str = "./bdk-wallet-example.sqlite";
// --8<-- [end:setup]

// Steps for migrating wallet state from an original `bdk` pre-1.0 version to a new
// `bdk_wallet` 1.0 or greater version.

// To run: change `BDK_DB_PATH` to point to the location of the old database file and
// modify the descriptors and network above to fit your setup. Before running, there
// should not be any persisted data at the new path `BDK_WALLET_DB_PATH`.

// --8<-- [start:main]
fn main() -> anyhow::Result<()> {
    // --8<-- [start:new]
    // Create new wallet
    let mut db = rusqlite::Connection::open(BDK_WALLET_DB_PATH)?;
    let mut new_wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(NETWORK)
        .create_wallet(&mut db)
        .context("failed to create wallet")?;
    // --8<-- [end:new]

    // --8<-- [start:pre1]
    // Get new wallet keychain descriptor hashes
    let external_checksum = new_wallet.descriptor_checksum(External);
    let internal_checksum = new_wallet.descriptor_checksum(Internal);

    // Get pre v1 wallet keychains and verify checksums match current wallet descriptors
    let mut pre_v1_db =
        rusqlite::Connection::open_with_flags(BDK_DB_PATH, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let pre_v1_keychains = bdk_wallet::migration::get_pre_v1_wallet_keychains(&mut pre_v1_db)?;
    assert!(!pre_v1_keychains.is_empty(), "no pre v1 keychain found");

    if let Some(pre_v1_external) = pre_v1_keychains.iter().find(|k| k.keychain == External) {
        assert_eq!(pre_v1_external.checksum, external_checksum);
        // Restore revealed external keychain to pre v1 address index
        let _ = new_wallet.reveal_addresses_to(
            KeychainKind::External,
            pre_v1_external.last_derivation_index,
        );
        println!(
            "Found and set pre v1 external keychain ({}) last derivation index to {}",
            external_checksum, pre_v1_external.last_derivation_index
        );
    } else {
        println!("no external pre v1 keychain found");
    }

    if let Some(pre_v1_internal) = pre_v1_keychains.iter().find(|k| k.keychain == Internal) {
        assert_eq!(pre_v1_internal.checksum.clone(), internal_checksum);
        // Restore revealed internal keychain to pre v1 address index
        let _ = new_wallet.reveal_addresses_to(Internal, pre_v1_internal.last_derivation_index);
        println!(
            "Found and set pre v1 internal keychain ({}) last derivation index to {}",
            internal_checksum, pre_v1_internal.last_derivation_index
        );
    } else {
        println!("no internal pre v1 keychain found");
    }
    // --8<-- [end:pre1]

    // --8<-- [start:persist]
    // Persist new wallet
    new_wallet.persist(&mut db)?;
    // --8<-- [end:persist]

    Ok(())
}
// --8<-- [end:main]
