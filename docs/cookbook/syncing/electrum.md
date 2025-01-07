# Sync a Wallet with Electrum

### Add required bdk dependencies to your `Cargo.toml` file

```toml title="Cargo.toml"
--8<-- "examples/rust/syncing/electrum/Cargo.toml:deps"
```

### Create and sync the wallet

```rust title="main.rs"
--8<-- "examples/rust/syncing/electrum/src/main.rs"
```
