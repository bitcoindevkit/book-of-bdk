# Readme

The directories inside [syncing](./syncing/) each contain an example showcased in the book. If you have the [just](https://just.systems/) tool installed, you can launch any of the examples using
```shell
just syncing-<cratename>
```

To see the list of all available examples (_just recipes_) run

```shell
just --list
```

Alternatively, you can run
```
cd syncing-<cratename>
cargo run --bin <cratename>
```

## Individual examples
- The [electrum](./syncing/electrum/) example is the companion to the [Electrum Wallet](https://bitcoindevkit.github.io/book-of-bdk/book/electrum-wallet/) page.
- The [esplora](./syncing/esplora/) example is the companion to the [Esplora Wallet](https://bitcoindevkit.github.io/book-of-bdk/book/esplora-wallet/) page.
