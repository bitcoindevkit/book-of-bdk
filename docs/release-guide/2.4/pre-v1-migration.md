# Pre-v1 Migration Helper

!!! note "Overview"

    * **Lead Developer:** [@notmandatory]
    * **Pull Requests:** [#365], [#415]
    * **Feature Type:** Non-Breaking

## Overview

`get_pre_v1_wallet_keychains` is a new function in the `wallet::migration` module that reads a legacy pre-1.0 BDK SQLite database and returns the keychain metadata needed to construct an equivalent post-1.0 wallet. It has been backported from the 3.0 release.

## Why Do This?

Users who created wallets with BDK versions prior to 1.0 stored their state in a SQLite schema that is not compatible with the current `bdk_wallet` persistence format. Before this helper existed, migrating from a pre-1.0 database required manually querying the old schema to extract keychain information. `get_pre_v1_wallet_keychains` encapsulates that logic and surfaces the data in a structured form.

## New Function

```rust
pub fn get_pre_v1_wallet_keychains(
    conn: &mut Connection,
) -> Result<Vec<PreV1WalletKeychain>, PreV1MigrationError>
```

The function takes a `rusqlite::Connection` to the old database and returns a list of [`PreV1WalletKeychain`] values, one per keychain found in the database.

```rust
pub struct PreV1WalletKeychain {
    /// "External" or "Internal"
    pub keychain: KeychainKind,
    /// Last derived key index stored in the old database
    pub last_derivation_index: u32,
    /// Descriptor checksum — must match the corresponding post-1.0 descriptor
    pub checksum: String,
}
```

## Migration Flow

```rust
use bdk_wallet::wallet::migration::get_pre_v1_wallet_keychains;
use bdk_chain::rusqlite::Connection;

// Open the old pre-1.0 BDK database
let mut old_conn = Connection::open("old_wallet.db")?;
let keychains = get_pre_v1_wallet_keychains(&mut old_conn)?;

for kc in &keychains {
    println!(
        "{:?}: last_index={}, checksum={}",
        kc.keychain, kc.last_derivation_index, kc.checksum
    );
}

// Use the keychain info to reveal addresses up to the last known index
// in your newly created post-1.0 wallet, ensuring no previously used
// addresses fall outside the revealed range.
```

!!! warning

    The `checksum` field must match the descriptor checksum of the corresponding keychain in your new wallet. If there is a mismatch, the wallet descriptors may not be equivalent and a migration should not proceed.

[@notmandatory]: https://github.com/notmandatory
[#365]: https://github.com/bitcoindevkit/bdk_wallet/pull/365
[#415]: https://github.com/bitcoindevkit/bdk_wallet/pull/415
[`PreV1WalletKeychain`]: https://docs.rs/bdk_wallet/2.4.0/bdk_wallet/wallet/migration/struct.PreV1WalletKeychain.html
