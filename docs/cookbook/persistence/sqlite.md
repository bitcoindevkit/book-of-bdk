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

```toml
[dependencies]
bdk_wallet = { version = "1.0.0", features = ["rusqlite"] }
```

To create a sqlite-based persisted wallet, simply call the `create_wallet()` with a valid db connection on the wallet builder:

```rust
use bdk_wallet::rusqlite::Connection;

let mut conn = Connection::open(file_path)?;
let wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
    .network(Network::Signet)
    .create_wallet(&mut conn)
    .expect("valid wallet and db connection");
```

After performing an operation that returns data that should be persisted, use the `persist()` method on the wallet:

```rust
let address = wallet.next_unused_address(KeychainKind::External);
wallet.persist(&mut conn)?;
```

<br>
