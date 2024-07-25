# In-Memory Wallet

The simplest wallet is one that does _not_ have any persistence. All information about the wallet is held in memory, and will be destroyed upon termination of the process.

In-memory implies that the addresses the wallet has revealed, the syncing that has been performed including data on the transaction graph will not persist when the wallet is destroyed, and related operations will need to be performed again.

In general, this means performing a `full_scan()` when starting the wallet, because it has no knowledge of which addresses were given out and which scripts still have balances.

See our page on the [difference between the full scan and sync operations](../syncing/full-scan-vs-sync.md) for more on this topic.

## Example

The in-memory wallet does _not_ require any additional dependencies beyond the `bdk_wallet` dependency:

```toml
[dependencies]
bdk_wallet = { version = "=1.0.0-beta.1" }
```

To create an in-memory wallet, simply call `create_wallet_no_persist()` on the `Wallet` builder:

```rust
let mut wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
    .network(Network::Signet)
    .create_wallet_no_persist()
    .expect("valid wallet");
```

<br>
