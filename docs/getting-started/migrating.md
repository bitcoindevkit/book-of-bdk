# Migrating from 0.X

!!! tip
    This page is up-to-date with version `1.0.0-beta.5` of bdk.

So you're ready to migrate to BDK version 1.0, congratulations!
This document contains some helpful tips that, with the help of some automation, should make the process as seamless as possible.

The below steps are for migrating wallet details from the old [`bdk` v0.30][0] to the new [`bdk_wallet` v1.0][1].
This procedure can be applied to wallets backed by a SQLite database.
Of particular concern is the ability to restore the _last known address index_ for each keychain.
This is important because without that metadata the new wallet may end up reusing receive addresses, which should be avoided for privacy reasons, although it should not cause loss of funds.

!!! tip
    NB: The migration process outlined below will not automatically restore the wallet's transaction data or local view of the blockchain.
    Thanks to the public ledger however, we can restore all the pertinent information for this wallet using one of the blockchain client libraries supported by BDK.

## Overview

1. Load an old database
1. Get last revealed addresses
1. Create new wallet
1. Restore revealed addresses
1. Write to new database
1. Sync

<!-- overview -->
```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:main"
```

## Walkthrough

In a new rust project add these dependencies to Cargo.toml

<!-- deps -->
```toml title="Cargo.toml"
--8<-- "examples/rust/migrate-version/Cargo.toml:deps"
```

Because there are two versions of bdk in the same project, we need to pay attention to how types are imported.
To avoid name clashes or any sort of mismatch resolving types that appear similar, we use fully qualified syntax, for example `bdk::bitcoin::Network::Testnet`.
You'll notice in some cases we can get around this annoyance by casting a value to another rust primitive or standard library type such as `String`.

<!-- imports -->
```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:use"
```

<!-- setup -->
Take a minute to define a few constants, for example the file path to the current database and the path to be used for the new database.
The descriptors and network shown here are for illustration; you should substitute them with your own.
Note that because we'll be creating a fresh database there should not already exist a persisted wallet at the new path.

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:setup"
```

<!-- old -->
Now retrieve the last revealed addresses from the `old_wallet`.

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:old"
```

<!-- new -->
For the `new_wallet` we should be using the same descriptors and network as before.
If the given descriptors contain secret keys, then the wallet will be able to sign transactions as well.

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:new"
```

<!-- sync -->
Now that we have a new database and have properly restored our addresses, you will want to sync with the blockchain to recover the wallet's transactions.
Below is an example of doing a `sync` using `bdk_esplora` but the exact method of syncing will depend on your application.
Happy migrating and see you on [v1.0][1]!

```rust title="examples/rust/migrate-version/src/main.rs"
--8<-- "examples/rust/migrate-version/src/main.rs:sync"
```

[0]: https://docs.rs/bdk/0.30.0/bdk/
[1]: https://docs.rs/bdk_wallet/1.0.0-beta.5/bdk_wallet/
