# Quick Start Example

This page details the code in the quickstart example in the `examples` folder of the _Book of BDK_. You can view and run the code for the example in your preferred language:

=== "Rust"

    <a href="https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/rust/quickstart" target="_blank">Rust quickstart example -></a>

=== "Swift"

    <a href="https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/swift/quickstart" target="_blank">Swift quickstart example -></a>

=== "Kotlin"

    <a href="https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/kotlin/quickstart" target="_blank">Kotlin quickstart example -></a>

Note that these examples are meant to be run at the command line. If you're building a mobile app in an IDE like xcode or androidstudio the process (of project creation, dependency selection and running the code) may be different.

!!! tip
    This page is up-to-date with version `1.0.0-beta.5` of `bdk_wallet`.

## Create a new project

=== "Rust"

    ```shell
    cargo init quickstart
    ```

=== "Swift"

    ```shell
    swift package init --type executable
    ```
    Or, if you're building an iOS app, create a new Swift project in Xcode.

=== "Kotlin"

    Make a project folder and add the following files: `build.gradle.kts`, `src/main/kotlin/main.kt`

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
    2. Enter `https://github.com/bitcoindevkit/bdk-swift` into the package repository URL text field

=== "Kotlin"

    ```kotlin title="build.gradle.kts"
    --8<-- "examples/kotlin/quickstart/build.gradle.kts"
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

    ```kotlin title="examples/kotlin/quickstart/src/main/kotlin/main.kt"
    --8<-- "examples/kotlin/quickstart/src/main/kotlin/main.kt:file"
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

    ```shell
    gradle build
    gradle run
    ```

## Let's take a closer look:

## Descriptors

First we need some <a href="https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md" target="_blank">descriptors</a> to instantiate our wallet. In this example we will use public key descriptors to simply display the balance of a wallet. To actually sign transactions you will need to use a wallet that is instantiated with private key descriptors. Refer to the [Working with Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate your own private key descriptors.
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
    --8<-- "examples/kotlin/quickstart/src/main/kotlin/main.kt:descriptors"
    ```
These are taproot `tr()` descriptors using a public key on testnet (or signet) `tpub` as described in <a href="https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki" target="_blank">BIP86</a>. The `descriptor` is an HD wallet with a path for generating addresses to give out externally for payment. We also have a second `change_descriptor` that we can use to generate addresses to pay ourseves change when sending payments (remeber that <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#outpoint" target="_blank">UTXOs</a> must be spent if full, so you often want to make change).

## Blockchain Client and Network

This example is using an <a href="https://github.com/Blockstream/esplora" target="_blank">Esplora</a> client on <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch11_blockchain.adoc#signet-the-proof-of-authority-testnet" target="_blank">Signet</a> hosted by the BDK team.
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
    --8<-- "examples/kotlin/quickstart/src/main/kotlin/main.kt:client"
    ```
Other options for blockchain clients include running an Electrum light wallet or using RPC on a bitcoind fullnode. We are using Esplora in this example as it is the most powerfull of these three options.
This example also used the Signet network, which is a test network that has some control mechanisms that ensure the network state is pretty similar to the blockchain mainnet (Testnet doesn't have those guarantees). You may alternatively want to run this example wallet using a locally hosted Regtest network, however the details of how to set that up are beyond the scope of this example.

## Scan

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
    --8<-- "examples/kotlin/quickstart/src/main/kotlin/main.kt:scan"
    ```
This scanning process is detailed in [Full Scan vs Sync](./syncing/full-scan-vs-sync.md). The scanning process checks child pubkeys for the descriptors specified in the wallet to detect UTXOs that can be spent by the wallet. That scan data is then applied to the wallet.

### Display Wallet Balance

Finally we can print the `wallet.balance()` to see how many sats we have available based on the information gathered in the scanning process.