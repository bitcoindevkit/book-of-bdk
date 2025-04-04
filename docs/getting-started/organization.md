# Project Organization

Within the [`bitcoindevkit` GitHub organization](https://github.com/bitcoindevkit){:target="_blank"}, the BDK team maintains a suite of Rust crates which provide both easy-to-use high level APIs and powerful lower level components to use when building more advanced bitcoin software.

## Source Code

The core project is split up into several crates in the `bdk/crates` directory:

- [`bdk_wallet`][bdk-crates]: Contains the central high level `Wallet` type
   that is built from the low-level mechanisms provided by the other components.
- [`bdk_chain`][bdk-crates]: Tools for storing and indexing chain data.
- [`bdk_file_store`][bdk-crates]: A persistence backend for storing chain data in a single file.
- [`bdk_sqlite`][bdk-crates]: A simple SQLite relational database client for persisting bdk_chain data.
- [`bdk_esplora`][bdk-crates]: Extends the [`esplora-client`][esplora-client]
   crate with methods to fetch chain data from an esplora HTTP server in the form that
   [`bdk_chain`][bdk-crates] and `Wallet` can consume.
- [`bdk_electrum`][bdk-crates]: Extends the [`electrum-client`][electrum-client]
   crate with methods to fetch chain data from an electrum server in the form that
   [`bdk_chain`][bdk-crates] and `Wallet` can consume.
- [`bdk_bitcoind_rpc`][bdk-crates]: For extracting and emitting blockchain data from `bitcoind` RPC interface. 

[bdk-crates]: https://github.com/bitcoindevkit/bdk/tree/master/crates
[esplora-client]: https://docs.rs/esplora-client/
[electrum-client]: https://docs.rs/electrum-client/
