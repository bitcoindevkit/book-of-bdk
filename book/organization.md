# Project Organization
Within the [`bitcoindevkit/bdk`] repository the BDK team maintains a suite of rust crates which provide both an easy to use high level API as well as powerful lower level components to used when building more advanced bitcoin software.

The project is split up into several crates in the `/crates` directory:

- [`bdk`]: Contains the central high level `Wallet` type that is built from the low-level mechanisms provided by the other components
- [`chain`]: Tools for storing and indexing chain data
- [`file_store`]: A (experimental) persistence backend for storing chain data in a single file.
- [`esplora`]: Extends the [`esplora-client`] crate with methods to fetch chain data from an esplora HTTP server in the form that [`bdk_chain`] and `Wallet` can consume.
- [`electrum`]: Extends the [`electrum-client`] crate with methods to fetch chain data from an electrum server in the form that [`bdk_chain`] and `Wallet` can consume.

[`bitcoindevkit/bdk`]: https://github.com/bitcoindevkit/bdk
[`esplora-client`]: https://docs.rs/esplora-client/
[`electrum-client`]: https://docs.rs/electrum-client/
[`bdk_chain`]: https://docs.rs/bdk-chain/
