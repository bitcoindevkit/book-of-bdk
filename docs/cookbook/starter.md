# Full Wallet Example

This page details the code in the <a href="https://github.com/bitcoindevkit/book-of-bdk/tree/master/companion-code/fullwallet" target="_blank">fullwallet</a> example in the `companion-code` folder of the _Book of BDK_.

!!! tip
    This page is up-to-date with version `1.0.0-beta.5` of `bdk_wallet`.

## Create a new Rust project

```shell
cargo init fullwallet
cd fullwallet
```

## Add required dependencies to your `Cargo.toml` file

```toml
--8<-- "companion-code/fullwallet/Cargo.toml"
```

## Create a wallet, sync it and display the balance in `src/main.rs`

We'll give a breakdown of the key pieces of this code in the next section.
```rust title="companion-code/fullwallet/src/main.rs""
--8<-- "companion-code/fullwallet/src/main.rs:file"
```

## Build and run:

The wallet will take a few seconds to sync, then you should see the wallet balance printed in the terminal.
```shell
cargo build
cargo run
```

## Let's take a closer look at what's going on in `src/main.rs`:

## Descriptors

First we need some <a href="https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md" target="_blank">descriptors</a> to instantiate our wallet. In this example we will use public key descriptors to simply display the balance of a wallet. To actually sign transactions you will need to use a wallet that is instantiated with private key descriptors. Refer to the [Working with Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate your own private key descriptors.
```rust
--8<-- "companion-code/fullwallet/src/main.rs:descriptors"
```
These are taproot `tr()` descriptors using a public key on testnet (or signet) `tpub` as described in <a href="https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki" target="_blank">BIP86</a>. The `EXTERNAL_DESCRIPTOR` is an HD wallet with a path for generating addresses to give out externally for payment. We also have a second `INTERNAL_DESCRIPTOR` that we can use to generate addresses to pay ourseves change when sending payments (remeber that <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#outpoint" target="_blank">UTXOs</a> must be spent if full, so you often want to make change).

## Blockchain Client and Network

This example is using an <a href="https://github.com/Blockstream/esplora" target="_blank">Esplora</a> client on <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch11_blockchain.adoc#signet-the-proof-of-authority-testnet" target="_blank">Signet</a> hosted by the BDK team.
```rust
--8<-- "companion-code/fullwallet/src/main.rs:client"
```
Other options for blockchain clients include running an Electrum light wallet or using RPC on a bitcoind fullnode. We are using Esplora in this example as it is the most powerfull of these three options.
This example also used the Signet network, which is a test network that has some control mechanisms that ensure the network state is pretty similar to the blockchain mainnet (Testnet doesn't have those guarantees). You may alternatively want to run this example wallet using a locally hosted Regtest network, however the details of how to set that up are beyond the scope of this example.

## Scan

Once we have our wallet setup and connected to the network, we scan the network to detect UTXOs relevant to our wallet.
```rust
--8<-- "companion-code/fullwallet/src/main.rs:scan"
```
This scanning process is detailed in [Full Scan vs Sync](./syncing/full-scan-vs-sync.md). The scanning process checks child pubkeys for the descriptors specified in the wallet to detect UTXOs that can be spent by the wallet. That scan data is then applied to the wallet.

### Display Wallet Balance

Finally we can print the `wallet.balance()` to see how many sats we have available based on the information gathered in the scanning process.