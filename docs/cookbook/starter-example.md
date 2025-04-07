# Simple Wallet Example

This page provides a starter example showcasing how BDK can be used` to create, sync, and manage a wallet using an Esplora client as a blockchain data source. Familiarity with this example will help you work through the more advanced pages in this section.

You can find [working code examples](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples) of this example in three programming languages: [Rust](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/rust), [Swift](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/swift), and [Kotlin](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/kotlin). (Note: some additional language bindings are available for BDK, see [3rd Party Bindings](../getting-started/3rd-party-bindings.md)).

## Create a new project

=== "Rust"

    ```shell
    cargo init starter-example
    cd starter-example
    ```

=== "Swift"

    ```shell
    swift package init --type executable
    ```

=== "Kotlin"

    ```shell
    gradle init
    ```

## Add required dependencies

=== "Rust"

    ```toml title="Cargo.toml"
    --8<-- "examples/rust/starter-example/Cargo.toml"
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

    ```kotlin title="build.gradle.kts"
    repositories {
        mavenCentral()
    }

    dependencies {
        // for jvm
        implementation("org.bitcoindevkit:bdk-jvm:1.1.0")
        // for Android
        implementation("org.bitcoindevkit:bdk-android:1.1.0")
    }
    ```

## Use descriptors

To create a wallet using BDK, we need some <a href="https://github.com/bitcoin/bitcoin/blob/master/doc/descriptors.md" target="_blank">descriptors</a> for our wallet. This example uses public descriptors (meaning they cannot be used to sign transactions) on Signet. Step 7 and below will fail unless you replace those public descriptors with private ones of your own and fund them using Signet coins through a faucet. Refer to the [Creating Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate your own private descriptors.

=== "Rust"

    ```rust
    --8<-- "examples/rust/starter-example/src/main.rs:descriptors"
    ```

=== "Swift"

    // TODO
    ```swift
    --8<-- "examples/swift/quickstart/Sources/main.swift:descriptors"
    ```

=== "Kotlin"

    // TODO
    ```kotlin
    --8<-- "examples/kotlin/quickstart/main.kt:descriptors"
    ```

These are taproot descriptors (`tr()`) using public keys on Signet (`tpub`) as described in <a href="https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki" target="_blank">BIP86</a>. The first descriptor is an HD wallet with a path for generating addresses to give out externally for payments. The second one is used by the wallet to generate addresses to pay ourselves change when sending payments (remember that <a href="https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06_transactions.adoc#outpoint" target="_blank">UTXOs</a> must be spent in full, so you often need to make change).

## Create or load a wallet

Next let's load up our wallet.

```rust title="examples/rust/starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:wallet"
```

## Sync the wallet

Now let's build an Esplora client and use it to request transaction history for the wallet.

```rust title="examples/rust/starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:client"
```

In cases where you are using new descriptors that do not have a balance yet, the example will request a new address from the wallet and print it out so you can fund the wallet. Remember that this example uses Signet coins!

```rust title="examples/rust/starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:address"
```

## Send a transaction

For this step you'll need a wallet built with private keys, funded with some Signet satoshis. You can find a faucet [here](https://signet25.bublina.eu.org/) to get some coins.

Let's prepare to send a transaction. The two core choices here are where to send the funds and how much to send. We will send funds back to the faucet return address; it's good practice to send test sats back to the faucet when you're done using them.

```rust title="examples/rust/starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:recipient"
```

Here we are sending 5000 sats back to the faucet (make sure the wallet has at least this much balance, or change this value).

Finally we are ready to build, sign, and broadcast the transaction:

```rust title="examples/rust/starter-example/src/main.rs"
--8<-- "examples/rust/starter-example/src/main.rs:transaction"
```

We can view our transaction on the Signet [mempool.space](https://mempool.space/signet) explorer.
