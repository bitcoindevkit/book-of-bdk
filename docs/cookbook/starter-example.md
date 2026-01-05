# Simple Starter Example

So you want to build a bitcoin wallet using BDK. Great! Here is the rough outline of what you need to do just that. A standard, simple example of a bitcoin wallet in BDK-land would require 3 core pillars:

<p style="text-align: center;">
  <img src="../../assets/wallets.png" style="height: 300px;">
</p>

1. **The `bdk_wallet` library**, which will provide two core types: the `Wallet` and the `TxBuilder`. This library will handle all the domain logic related to keeping track of which UTXOs you own, what your total balance is, creating and signing transactions, etc.
2. **A blockchain client**. Your wallet will need to keep track of blockchain data, like new transactions that have been added to the blockchain that impact your wallet, requesting these transactions from a Bitcoin Core node, an Electrum or Esplora server, etc.
3. **A persistence mechanism** for saving wallet data between sessions (note that this is not actually required). Things like which addresses the wallet has revealed and what is the state of the blockchain on its last sync are things that are kept in persistence and can be loaded on startup.

## Diving in!

This page provides a starter example showcasing how BDK can be used to create, sync, and manage a wallet using an Esplora client as a blockchain data source. Familiarity with this example will help you work through the more advanced pages in this section.

You can find [working code examples](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples) of this example in three programming languages: [Rust](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/rust), [Swift](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/swift), and [Kotlin](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/kotlin). (Note: some additional language bindings are available for BDK, see [3rd Party Bindings](../getting-started/3rd-party-bindings.md)).

!!!tip
    To complete this example from top to bottom, you'll need to create new descriptors and replace the ones provided. Once you do so, you'll run the example twice; on first run the wallet will not have any balance and will exit with an address to send funds to. Once that's done, you can run the example again and the wallet will be able to perform the later steps, namely creating and broadcasting a new transaction.

## Create a new project

```shell
cargo init starter-example
cd starter-example
```

## Add required dependencies

```toml title="Cargo.toml"
--8<-- "examples/rust/starter-example/Cargo.toml"
```

## Use descriptors

To create a wallet using BDK, we need some <a href="https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md" target="_blank">descriptors</a> for our wallet. This example uses public descriptors (meaning they cannot be used to sign transactions) on Signet. Step 7 and below will fail unless you replace those public descriptors with private ones of your own and fund them using Signet coins through a faucet. Refer to the [Creating Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate your own private descriptors.

!!!warning
    Note that if you replace the descriptors after running the example using the provided ones, you must delete or rename the database file or will get an error on wallet load.

```rust
--8<-- "examples/rust/starter-example/src/main.rs:descriptors"
```

These are taproot descriptors (`tr()`) using public keys on Signet (`tpub`) as described in <a href="https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki" target="_blank">BIP86</a>. The first descriptor is an HD wallet with a path for generating addresses to give out externally for payments. The second one is used by the wallet to generate addresses to pay ourselves change when sending payments (remember that <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#outpoint" target="_blank">UTXOs</a> must be spent in full, so you often need to make change).

## Create or load a wallet

Next let's load up our wallet.

```rust title="starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:wallet"
```

## Sync the wallet

Now let's build an Esplora client and use it to request transaction history for the wallet.

```rust title="starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:client"
```

In cases where you are using new descriptors that do not have a balance yet, the example will request a new address from the wallet and print it out so you can fund the wallet. Remember that this example uses Signet coins!

```rust title="starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:address"
```

## Send a transaction

For this step you'll need a wallet built with private keys, funded with some Signet satoshis. You can find a faucet [here](https://signet25.bublina.eu.org/) to get some coins.

Let's prepare to send a transaction. The two core choices here are where to send the funds and how much to send. We will send funds back to the faucet return address; it's good practice to send test sats back to the faucet when you're done using them.

```rust title="starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:recipient"
```

Here we are sending 5000 sats back to the faucet (make sure the wallet has at least this much balance, or change this value).

Finally we are ready to build, sign, and broadcast the transaction:

```rust title="starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:transaction"
```

We can view our transaction on the [mempool.space Signet explorer](https://mempool.space/signet).
