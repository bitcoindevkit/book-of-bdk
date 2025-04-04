# Sync a Wallet with Esplora

Syncing with Esplora uses what we refer to as _SPK-based syncing_ (see our [Full Scan vs Sync](./full-scan-vs-sync.md) page for more information on this).

The workflow for a full scan or sync consists of a 3-step process:

1. Ask the wallet for the data structure required.
2. Pass it to your blockchain client and request a full scan or sync.
3. The client returns an update, which you then apply to the wallet.

This workflow ensures that the wallet structure is not blocked while the syncing operation is performed.

### Add required bdk dependencies to your Cargo.toml file

```toml title="Cargo.toml"
--8<-- "examples/rust/syncing/esplora/Cargo.toml:deps"
```

### Create and sync the wallet

```rust title="main.rs"
--8<-- "examples/rust/syncing/esplora/src/main.rs"
```
