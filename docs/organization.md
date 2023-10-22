# Project Organization

Within the [`bitcoindevkit/bdk`][bitcoindevkit/bdk] repository the BDK team maintains
a suite of rust crates which provide both an easy-to-use high level API
and powerful lower level components to used when building more advanced bitcoin software.

The project is split up into several crates in the `/crates` directory:

- [`bdk`][bitcoindevkit/bdk]: Contains the central high level `Wallet` type
   that is built from the low-level mechanisms provided by the other components
- [`chain`][bitcoindevkit/bdk]: Tools for storing and indexing chain data
- [`file_store`][bitcoindevkit/bdk]: A (experimental) persistence backend for storing chain data in a single file.
- [`esplora`][bitcoindevkit/bdk]: Extends the [`esplora-client`][esplora-client]
   crate with methods to fetch chain data from an esplora HTTP server in the form that
   [`bdk_chain`][bitcoindevkit/bdk] and `Wallet` can consume.
- [`electrum`][bitcoindevkit/bdk]: Extends the [`electrum-client`][electrum-client]
   crate with methods to fetch chain data from an electrum server in the form that
   [`bdk_chain`][bitcoindevkit/bdk] and `Wallet` can consume.

[bitcoindevkit/bdk]: https://github.com/bitcoindevkit/bdk
[esplora-client]: https://docs.rs/esplora-client/
[electrum-client]: https://docs.rs/electrum-client/
[bdk_chain]: https://docs.rs/bdk-chain/
