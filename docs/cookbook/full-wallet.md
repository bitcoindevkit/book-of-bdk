# Full Wallet Example

This page will illustrate core wallet functionality including:

- Generating descriptors
- Recovering a wallet with descriptors
- Creating and broadcasting a transaction

Note: the logic to do this is in 3 separate examples, if you're following along with the code examples you will need to copy and paste descriptors between examples.

## Generating Descriptors

First lets run the [descriptors](../keys-descriptors/descriptors) example to our descriptors which will define our HD wallet:

```rust title="examples/rust/descriptors/src/main.rs"
// ...
--8<-- "examples/rust/descriptors/src/main.rs:main"
// ...
```

Notice there are two types of descriptors generated: private key descriptors (with full signing capabilities) and public key descriptors (primarily for generating invoice addresses). We will be using our private key descriptors to send a transaction later.

## Recovering a Wallet with Descriptors

Next lets use those descriptors to recover our wallet. Replace the descriptors in the [quickstart](./quickstart.md) example with either your private or publickey descriptors (either will work here) and run it to sync a wallet, check the balance, and generate a new invoice address:

```rust title="examples/rust/quickstart/src/main.rs"
// ...
--8<-- "examples/rust/quickstart/src/main.rs:address"
// ...
```

### Request Sats from Mutinynet Faucet

We can now use our new invoice address to request some sats from the [Mutinynet Faucet](https://mutinynet.com/faucet). After requesting sats you can view the transaction in the [Mempool](https://mutinynet.com/) (click the link on the confirmation page or put the transaction id in the search bar). After a minute or so you should see the transaction confirmed. We can also re-run the `quickstart` example and see that our wallet now has some funds!

## Creating and Broadcasting a Transaction

Finally we can test out sending some sats ourselves. Put your private key descriptors into the `transaction` example and run it to create, sign, and broadcast a transaction.

```rust title="examples/rust/transaction/src/main.rs"
// ...
--8<-- "examples/rust/transaction/src/main.rs:main"
// ...
```

Again we can view the transaction in the mempool or re-run the `quickstart` example to see that our wallet has less funds. Our funds will only be reduced by the fee amount because we sent the funds back to ourselves.




