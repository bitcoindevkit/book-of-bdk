# Getting Started

### Install Rust

See the Rust [Getting Started]{:target="_blank"} page to install the Rust development tools.

### Using BDK in a Rust project

Follow these steps to use BDK in your own rust project with the `electrum` blockchain client.

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

2. Add `bdk_wallet` to your `Cargo.toml` file. Find [the latest `bdk_wallet` release on crates.io][bdk_wallet on crates.io]{:target="_blank"}, for example:

```shell
cargo add bdk_wallet@1.0.0-alpha.13
```

3. Add other required dependencies:

```shell
cargo add bdk_electrum@0.15.0
cargo add bdk_sqlite@0.2.0
```

Follow the [Sync a Wallet with Electrum](../cookbook/wallet/electrum.md) page for a simple example of how to create and sync a wallet.

[Getting Started]: https://www.rust-lang.org/learn/get-started
[bdk_wallet on crates.io]: https://crates.io/crates/bdk_wallet/versions
