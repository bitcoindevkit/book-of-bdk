# Project Organization

Within the [`bitcoindevkit` GitHub organization](https://github.com/bitcoindevkit){:target="_blank"}, the BDK team maintains
a suite of Rust crates which provide both an easy-to-use high level APIs and powerful lower level components to use when building more advanced bitcoin software.

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

## API Documentation

- [bdk_wallet](https://docs.rs/bdk_wallet/){:target="_blank"}
- [bdk_chain](https://docs.rs/bdk_chain/){:target="_blank"}
- [bdk_sqlite](https://docs.rs/bdk_sqlite/){:target="_blank"}
- [bdk_file_store](https://docs.rs/bdk_file_store/){:target="_blank"}
- [bdk_electrum](https://docs.rs/bdk_electrum){:target="_blank"}
- [bdk_esplora](https://docs.rs/bdk_esplora){:target="_blank"}
- [bdk_bitcoind_rpc](https://docs.rs/bdk_bitcoind_rpc){:target="_blank"}
- [bdk_hwi](https://docs.rs/hwi){:target="_blank"}
- [esplora-client]{:target="_blank"}
- [electrum-client]{:target="_blank"}

[bdk-crates]: https://github.com/bitcoindevkit/bdk/tree/master/crates
[esplora-client]: https://docs.rs/esplora-client/
[electrum-client]: https://docs.rs/electrum-client/
