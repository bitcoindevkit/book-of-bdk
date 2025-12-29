# Project Organization

Within the [`bitcoindevkit` GitHub organization](https://github.com/bitcoindevkit), the BDK team maintains a suite of Rust crates which provide both easy-to-use high level APIs and powerful lower level components to use when building more advanced bitcoin software.

## Source Code

The core project is split up into several crates:

- [`bdk_wallet`][bdk-crates]: Contains the central high level `Wallet` type
   that is built from the low-level mechanisms provided by the other components.
- [`bdk_chain`][bdk-crates]: Tools for storing and indexing chain data.
- [`bdk_sqlite`][bdk-crates]: A simple SQLite relational database client for persisting bdk_chain data.
- [`bdk_esplora`][bdk-crates]: Extends the [`esplora-client`][esplora-client]
   crate with methods to fetch chain data from an esplora HTTP server in the form that
   [`bdk_chain`][bdk-crates] and `Wallet` can consume.
- [`bdk_electrum`][bdk-crates]: Extends the [`electrum-client`][electrum-client]
   crate with methods to fetch chain data from an electrum server in the form that
   [`bdk_chain`][bdk-crates] and `Wallet` can consume.
- [`bdk_bitcoind_rpc`][bdk-crates]: For extracting and emitting blockchain data from `bitcoind` RPC interface.
- [`bdk-ffi`](https://github.com/bitcoindevkit/bdk-ffi): The wrapper library that exposes the Rust APIs to our target language bindings through [uniffi](https://github.com/mozilla/uniffi-rs).

[bdk-crates]: https://github.com/bitcoindevkit/bdk/tree/master/crates
[esplora-client]: https://docs.rs/esplora-client/
[electrum-client]: https://docs.rs/electrum-client/
