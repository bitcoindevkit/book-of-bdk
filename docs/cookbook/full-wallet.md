# Full Wallet Example

This page will do a walk through of 3 examples to illustrate core wallet functionality including:

- Generating seeds and keys
- Recovering a wallet with seeds
- Creating and broadcasting a transaction

## Generating seeds and keys

First lets run the [descriptors](../keys-descriptors/descriptors) example to generate seeds, keys, and descriptors:

```rust title="examples/rust/descriptors/src/main.rs""
--8<-- "examples/rust/descriptors/src/main.rs"
```

## Recovering a wallet with seeds

Next lets use those seeds to recover our wallet. Put your seed phrase into the [wallet-recovery](../keys-descriptors/wallet-recovery) example and run it to sync a wallet, check the balance, and generate a new address:

```rust title="examples/rust/wallet-recovery/src/main.rs""
--8<-- "examples/rust/wallet-recovery/src/main.rs"
```

Note: use of the term 'recover' here might be confusing since we just created the wallet, however we are not persisting any data at the moment, so we will need to recover our wallet every time we want to use it.

### Request sats from Mutinynet Faucet

After recovering our wallet, we printed out a new address that funds can be sent to. Let's now hop over to the [Mutinynet Faucet](https://mutinynet.com/faucet) and request some sats. After requesting sats you can view the transaction in the [Mempool](https://mutinynet.com/) (click the link on the confirmation page or put the transaction id in the search bar). After a minute or so you should see the transaction confirmed. We can also re-run the `wallet-recovery` example and see that our wallet now has some funds!

## Creating and broadcasting a transaction

Finally we can test out sending some sats ourselves. Put your seed phrase into the `transaction` example and run it to create, sign, and broadcast a transaction.

```rust title="examples/rust/transaction/src/main.rs""
--8<-- "examples/rust/transaction/src/main.rs"
```

Again we can view the transaction in the mempool or re-run the `wallet-recovery` example to see that our wallet has less funds. Our funds will only be reduced by the fee amount because we sent the funds back to ourselves.




