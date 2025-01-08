# SQLite Database

The SQLite persistence is a great default for many use cases, and is a good place to start if you're not sure which persistence to choose from.

By default when using the `bdk_wallet` library, all information about the wallet is held in memory, and will be destroyed upon termination of the process unless saved to persistence. 

When information important to the wallet is added to it, the wallet will add it to its staged area. Whenever you want to save this information to persistence, call the `Wallet.persist(&mut db)`.

The operations that affect the wallet and produce a changeset are things like:

- Revealing new addresses
- Sync operations that pick up new UTXOs

Once those things are persisted, upon loading of the database changeset the wallet would be able to rehydrate its [TxGraph](), which includes UTXOs, transaction history, and latest blocks known to the wallet. This means that a wallet that's been loaded from such a persistence will not require a _Full Scan_ but rather simply a _Sync_.

See our page on the [difference between the full scan and sync operations](../syncing/full-scan-vs-sync.md) for more on this topic.

## Example

The sqlite wallet does not require any additional dependencies above the `bdk_wallet` dependency:

```toml title="Cargo.toml"
--8<-- "examples/rust/persistence/sqlite/Cargo.toml:deps"
```

To load an existing sqlite-based persisted wallet use `Wallet::load()`. You may then optionally verify the loaded descriptors match what you expect. If the provided descriptors contain private keys you can also extract these keys into the wallets keystore. Private keys are never stored in the wallet database. You may also verify the wallet network during loading.

```rust
--8<-- "examples/rust/persistence/sqlite/src/main.rs:load"
```

If during wallet loading no wallet database file is found you can create a sqlite-based persisted wallet with `Wallet::create()` with a valid db connection and other wallet builder parameters:

```rust
--8<-- "examples/rust/persistence/sqlite/src/main.rs:create"
```

After performing an operation that returns data that should be persisted, use the `persist()` method on the wallet:

```rust
--8<-- "examples/rust/persistence/sqlite/src/main.rs:address"
```

<br>
