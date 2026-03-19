# Migrating from 0.X

So you're ready to migrate from a pre-1.0 `bdk` version to `bdk_wallet`, congratulations!
This document contains some helpful tips that should make the process as seamless as possible. The process applies to pre-1.0 `bdk` wallets, such as [`bdk` v0.30][0], backed by a SQLite database being migrated to a new [`bdk_wallet` v2.4 or v3.0][1] based wallet. The `bdk_wallet` can use any database that implements [`WalletPersister`][2].

To migrate your wallet data to a new version of bdk, essentially all you need to do is grab the _last known address derivation index_ for each keychain from the old db and set them in your new db. After restoring your keychain derivation indexes, your next wallet sync will refetch the rest of your wallet's transaction data. Doing this means we don't need to perform a full scan because we already have the derivation indexes (doing a full scan would check for used addresses based on the stop gap which is unnecessary).

This migration is important because without that metadata the new wallet may end up reusing receive addresses, which should be avoided for privacy reasons.

!!! note
    The migration process outlined below will not automatically restore the wallet's transaction data or local view of the blockchain. Thanks to the public ledger however, we can restore all the pertinent information for this wallet using one of the blockchain client libraries supported by BDK.

## Overview

1. Create the new wallet using the same descriptors
2. Get the keychain info from the old wallet db file
3. Verify the old and new keychain descriptor checksums match
4. Restore the keychain(s) revealed address indexes
5. Persist the new database

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:main"
```

## Walkthrough

In your Rust project replace your pre-1.0 `bdk` dependency with `bdk_wallet`.

```toml title="Cargo.toml"
--8<-- "examples/rust/migrate-version/Cargo.toml:deps"
```

Take a minute to define a few constants, for example the file path to the current pre-1.0 `bdk` database and the path to be used for the new `bdk_wallet` database.
The descriptors and network shown here are for illustration; you should substitute them with your own.
Note that because we'll be creating a fresh database there must not yet be a persisted `bdk_wallet` database file at the new path.

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:setup"
```

Now create the new `bdk_wallet` wallet using the same descriptors as your original pre-1.0 `bdk` Wallet:

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:new"
```

And then retrieve the keychain details from the original pre-1.0 `bdk` wallet sqlite database file, verify the descriptor checksums match, and set your new `bdk_wallet` based wallet to use the correct keychain address derivation indexes.

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:pre1"
```

And finally you will need to persist your new wallet.

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:persist"
```

Now that we have a new database and have properly restored our keychain address indexes, you will need to sync with the blockchain to recover the wallet's transactions. See the page on how to [sync your wallet with electrum](../cookbook/syncing/electrum) as an example, but you can use any blockchain client backend to re-sync your wallet.

[0]: https://docs.rs/bdk/0.30.0/bdk/
[1]: https://docs.rs/bdk_wallet/3.0.0/bdk_wallet/
[2]: https://docs.rs/bdk_wallet/3.0.0/bdk_wallet/trait.WalletPersister.html