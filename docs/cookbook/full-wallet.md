# Full Wallet Example

This page illustrates core wallet functionality, including:

- Generating descriptors
- Wallet creation, persistence, and loading
- Full scan and light weight sync
- Creating and broadcasting a transaction

!!! tip
    The logic for this page is split between 2 separate examples in the [examples source code](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/rust). One to create descriptors and a second for everything else. If you are following along with the code examples you will need to copy and paste your private descriptors you get from the first example into the second. We leave descriptor creation in a separate example because bdk does not handle private descriptor (or private key) storage, that is up to the wallet developer.

## Generating Descriptors

First we [create signet descriptors](keys-descriptors/descriptors.md) for our wallet.

```rust title="examples/rust/descriptors/src/main.rs"
--8<-- "examples/rust/descriptors/src/main.rs:main"
```

Notice we are creating private descriptors here in order to sign transactions later on.

## Full Scan and Address Generation (First Run)

Next, lets use those descriptors to load up our wallet. Replace the placeholder descriptors in the `full-wallet` example with your private descriptors:

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:descriptors"
```

We are going to run this example twice. On the first run it will do a full scan for your wallet, persist that chain data, generate a new address for you, and display your current wallet balance, it will then attempt to build a transaction, but will fail becuase we don't have any funds yet. We will use the new address (from the first run) to request funds from the <a href="https://faucet.mutinynet.com/" target="_blank" rel="noopener noreferrer">Mutinynet faucet</a> so we can build a transaction on the second run. On the second run it will load the data from the previous run, do a light weight sync to check for updates (no need to repeat the full scan), and then build and broadcast a transaction. Let's go through this step by step.

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:persist"
```

In the quickstart example we simply used an in-memory wallet, with no persistence. But here we are saving wallet data to a file. Notice that we are providing our private descriptors during wallet load. This is because bdk never stores private keys, that responsibility is on the wallet developer (you). The data we are loading here does not include the private keys, but we want our wallet to have signing capabilities, so we need to provide our private descriptors during wallet load. If we get a wallet back from the load attempt, we'll use that, otherwise we'll create a new one. Since this is our first run nothing will be loaded and a new wallet will be created.

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:scan"
```

Next we'll fetch data from our blockchain client. On the first run, we don't yet have any data, so we need to do a full scan. We then persist the data from the scan.
Finally, we'll print out an address that we can use to request funds. You should also see the current balance printed out, it should be 0 since this is a brand new wallet. Note that we persist the wallet after generating the new address; this is to avoid re-using the same address as that would compromise our privacy (on subsequent runs you'll notice the address index incremented).

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:address"
```

The process will then error out, indicating we don't have enough funds to send a transaction.

### Request satoshis from the Mutinynet faucet

We can now use our new address to request some sats from the <a href="https://faucet.mutinynet.com/" target="_blank" rel="noopener noreferrer">Mutinynet faucet</a>. After requesting sats, you can view the transaction in their <a href="https://mutinynet.com/" target="_blank" rel="noopener noreferrer">Mempool Explorer instance</a> (click the link on the faucet confirmation page or put the txid in the search bar of the mempool explorer). After a minute or so you should see the transaction confirmed. We can also re-run the `full-wallet` example and see that our wallet now has some funds!

## Load, Sync, and Send a Transaction (Second Run)

Now that we have some funds, we can re-run the `full-wallet` example. Since we persisted data from the previous run, this time our wallet will be loaded. You do not need to provide descriptors to load wallet data, however, if you don't you will not have signing capabilities, so here we do provide our private descriptors in the loading process:

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:persist"
```

Since we already have some data from the previous run, it will not do a full scan, but only a sync which is faster and less data intensive.

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:scan"
```

Now that we have funds, let's prepare to send a transaction. We need to decide where to send the funds and how much to send.We will send funds back to the mutiny faucet return address. It's good practice to send test sats back to the faucet when you're done using them.

```rust title="examples/rust/full-wallet/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:faucet"
```

Here we are preparing to send 5000 sats back to the mutiny faucet, so you'll need to request at more sats than that or change this value (it's good practice to send test sats back to the faucet when you're done using them).

Finally we are ready to build, sign, and broadcast the transaction:

```rust title="examples/rust/transaction/src/main.rs"
--8<-- "examples/rust/full-wallet/src/main.rs:transaction"
```

We are manually determining the fee rate to be 4 sat/vb (satoshis per virtual byte).

Again we can view the transaction in the Mutinynet explorer or re-run the `full-wallet` example to see that our wallet has less funds.

### Drain the Wallet

When you're done with the test sats that you got from the faucet, you should send the remaining funds back to the faucet by [draining the wallet](/cookbook/transactions/transaction-builder/#spend-all-funds).