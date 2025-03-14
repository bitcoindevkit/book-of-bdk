# Quick Start Example

This page provides an overview of how BDK can be leveraged to create and sync a wallet using an Esplora client. You can find [working code examples](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples) of this workflow in three programming languages: [Rust](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/rust), [Swift](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/swift), and [Kotlin](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/kotlin). (Note: some additional language bindings are available for BDK, see [3rd Party Bindings](../getting-started/3rd-party-bindings.md))

## Create a new project

=== "Rust"

    ```shell
    cargo init quickstart
    cd quickstart
    ```

=== "Swift"

    ```shell
    swift package init --type executable
    ```
    Or, if you're building an iOS app, create a new Swift project in Xcode.

=== "Kotlin"

    Create a new Kotlin project.

## Add required dependencies

=== "Rust"

    ```toml title="Cargo.toml"
    --8<-- "examples/rust/quickstart/Cargo.toml"
    ```

=== "Swift"

    ```toml title="Package.swift"
    --8<-- "examples/swift/quickstart/Package.swift"
    ```
    Or, if you're building an iOS app:
    
    1. From the Xcode File menu, select Add Package Dependencies...
    2. Enter `https://github.com/bitcoindevkit/bdk-swift` into the package repository URL search field and bdk-swift should come up
    3. For the Dependency Rule select `Exact Version`, enter the version number (same as Package.swift) and click Add Package

=== "Kotlin"

    ```kotlin title="build.gradle"
    --8<-- "examples/kotlin/quickstart/build.gradle"
    ```

## Create a wallet, sync it and display the balance

We'll give a breakdown of the key pieces of this code in the next section.
=== "Rust"

    ```rust title="examples/rust/quickstart/src/main.rs""
    --8<-- "examples/rust/quickstart/src/main.rs:file"
    ```

=== "Swift"

    ```swift title="examples/swift/quickstart/Sources/main.swift"
    --8<-- "examples/swift/quickstart/Sources/main.swift:file"
    ```

=== "Kotlin"

    ```kotlin title="examples/kotlin/quickstart/main.kt"
    --8<-- "examples/kotlin/quickstart/main.kt:file"
    ```

## Build and run:

The wallet will take a few seconds to sync, then you should see the wallet balance printed in the terminal.

=== "Rust"

    ```shell
    cargo build
    cargo run
    ```

=== "Swift"

    ```shell
    swift run
    ```
    Or run the project in Xcode.

=== "Kotlin"

    Run the project in your IDE.

## Let's take a closer look

### Descriptors

First we need some <a href="https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md" target="_blank">descriptors</a> to instantiate our wallet. In this example we use public key descriptors to simply display the balance of a wallet. To sign transactions you will need to use a wallet that is instantiated with private key descriptors. Refer to the [Creating Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate your own private descriptors.

=== "Rust"

    ```rust
    --8<-- "examples/rust/quickstart/src/main.rs:descriptors"
    ```

=== "Swift"

    ```swift
    --8<-- "examples/swift/quickstart/Sources/main.swift:descriptors"
    ```

=== "Kotlin"

    ```kotlin
    --8<-- "examples/kotlin/quickstart/main.kt:descriptors"
    ```
These are taproot `tr()` descriptors using public keys on testnet (or signet) `tpub` as described in <a href="https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki" target="_blank">BIP86</a>. The `descriptor` is an HD wallet with a path for generating addresses to give out externally for payment. We also have a second `change_descriptor` that we can use to generate addresses to pay ourselves change when sending payments (remember that <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#outpoint" target="_blank">UTXOs</a> must be spent in full, so you often need to make change).

### Blockchain Client and Network

This example is using an <a href="https://github.com/Blockstream/esplora" target="_blank">Esplora</a> client on the Mutinynet Signet network.

=== "Rust"

    ```rust
    --8<-- "examples/rust/quickstart/src/main.rs:client"
    ```

=== "Swift"

    ```swift
    --8<-- "examples/swift/quickstart/Sources/main.swift:client"
    ```

=== "Kotlin"

    ```kotlin
    --8<-- "examples/kotlin/quickstart/main.kt:client"
    ```

Other options for blockchain clients include running an Electrum light wallet or using RPC on a bitcoind fullnode. We are using Esplora in this example.

This example also used the Signet network. You may alternatively want to run this example wallet using a locally hosted Regtest network. The details of how to set that up are beyond the scope of this example.

### Scan

Once we have our wallet setup and connected to the network, we scan the network to detect UTXOs relevant to our wallet.

=== "Rust"

    ```rust
    --8<-- "examples/rust/quickstart/src/main.rs:scan"
    ```

=== "Swift"

    ```swift
    --8<-- "examples/swift/quickstart/Sources/main.swift:scan"
    ```

=== "Kotlin"

    ```kotlin
    --8<-- "examples/kotlin/quickstart/main.kt:scan"
    ```

This scanning process is detailed on the [Full Scan vs Sync](./syncing/full-scan-vs-sync.md) page. The scanning process checks child pubkeys for the descriptors specified in the wallet to detect UTXOs that are relevant to the wallet. That data is then applied to the wallet.

### Display Wallet Balance

Finally we can print the `wallet.balance()` to see how many sats we have available based on the information gathered in the scanning process.
