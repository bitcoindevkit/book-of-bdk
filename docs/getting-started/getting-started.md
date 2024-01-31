# Getting Started

### Install Rust

See the Rust [Getting Started] page to install the Rust development tools.

### Using BDK in a Rust project

Follow these steps to use BDK in your own rust project with the async `esplora` blockchain client.

!!! tip
    For now, we suggest using the latest `master` branch versions of BDK crates.
    As an example, for BDK:

    ```shell
    cargo add bdk
    ```

1. Create a new Rust project:

```shell
cargo init my_bdk_app
cd my_bdk_app
```

2. Add `bdk` to your `Cargo.toml` file. Find the latest `bdk@1` release on [`crates.io`](https://crates.io/crates/bdk/versions), for example:

```shell
cargo add bdk@1.0.0-alpha.5
```

3. Add other required dependencies:

```shell
cargo add bdk_esplora@0.7.0
cargo add bdk_file_store@0.5.0
```

See the [Wallet with Electrum Example](../book/electrum-wallet.md) page for how to create and sync a wallet.

[Getting Started]: https://www.rust-lang.org/learn/get-started
