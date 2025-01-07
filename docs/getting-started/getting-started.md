# Getting Started

!!! notice
    This book is up-to-date with version `1.0.0` of the `bdk_wallet` rust library.

### Install Rust

See the Rust [Getting Started]{:target="_blank"} page to install the Rust development tools.

### Using BDK in a Rust project

Follow these steps to use BDK in your own rust project with the `electrum` blockchain client.

1. Create a new Rust project:

```shell
cargo init my_bdk_app
cd my_bdk_app
```

2. Add `bdk_wallet` to your `Cargo.toml` file. Find [the latest `bdk_wallet` release on crates.io][bdk_wallet on crates.io]{:target="_blank"}, for example:

```shell
cargo add bdk_wallet@1.0.0
```

3. Add other required dependencies:

```shell
cargo add bdk_electrum@0.20.1
```

Follow the [Sync a Wallet with Electrum](../cookbook/syncing/electrum.md) page for a simple example of how to create and sync a wallet.

[Getting Started]: https://www.rust-lang.org/learn/get-started
[bdk_wallet on crates.io]: https://crates.io/crates/bdk_wallet/versions
