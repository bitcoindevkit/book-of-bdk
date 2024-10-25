# Full Wallet Example

This page illustrates core wallet functionality, including:

- Generating descriptors
- Recovering a wallet with descriptors
- Creating and broadcasting a transaction

!!! tip
    The logic for this page is broken down in in 3 separate examples in the [examples source code](https://github.com/bitcoindevkit/book-of-bdk/tree/master/examples/rust). If you are following along with the code examples you will need to copy and paste descriptors between them.

## Generating Descriptors

First we [create signet descriptors](keys-descriptors/descriptors.md) for our wallet.

```rust title="examples/rust/descriptors/src/main.rs"
--8<-- "examples/rust/descriptors/src/main.rs:main"
```

Notice we are creating private descriptors here in order to sign transactions later on.

## Recovering a Wallet with Descriptors

Next, lets use those descriptors to load up our wallet. Replace the descriptors in the [quickstart](./quickstart.md) example with either your private or publickey descriptors (either will work here) and run it to sync a wallet, check the balance, and generate a new address:

```rust title="examples/rust/quickstart/src/main.rs"
--8<-- "examples/rust/quickstart/src/main.rs:address"
```

### Request satoshis from the Mutinynet faucet

We can now use our new address to request some sats from the [Mutinynet faucet](https://faucet.mutinynet.com/). After requesting sats, you can view the transaction in their [Mempool Explorer instance](https://mutinynet.com/) (click the link on the faucet confirmation page or put the txid in the search bar of the mempool explorer). After a minute or so you should see the transaction confirmed. We can also re-run the `quickstart` example and see that our wallet now has some funds!

## Creating and Broadcasting a Transaction

Finally we use the wallet to send some satoshis. Put your private descriptors into the `transaction` example and run it to create, sign, and broadcast a transaction.

```rust title="examples/rust/transaction/src/main.rs"
--8<-- "examples/rust/transaction/src/main.rs:main"
```

Again we can view the transaction in the Mutinynet explorer or re-run the `quickstart` example to see that our wallet has less funds.
