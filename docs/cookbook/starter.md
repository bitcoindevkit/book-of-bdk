# Full Wallet Example

!!! tip
    This page is up-to-date with version `1.0.0-beta.5` of `bdk_wallet`.

## Create a new Rust project

```shell
cargo init fullwallet
cd fullwallet
```

## Add required dependencies to your `Cargo.toml` file

```toml
--8<-- "companion-code/fullwallet/Cargo.toml"
```

## Create your descriptors

Refer to the [Working with Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate descriptors. This page will assume you are working on signet with the following BIP86 descriptors:
```rust
--8<-- "companion-code/fullwallet/src/main.rs:descriptors"
```

## Create a wallet, sync it, build a transaction, and broadcast it in `src/main.rs`

```rust title="companion-code/fullwallet/src/main.rs""
--8<-- "companion-code/fullwallet/src/main.rs:main"
```

## Build and run:

```shell
cargo build
cargo run
```