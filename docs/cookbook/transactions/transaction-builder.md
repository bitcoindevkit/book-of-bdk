# Transaction Builder

The [Transaction Builder](https://docs.rs/bdk_wallet/latest/bdk_wallet/struct.TxBuilder.html) provides a convenient way to construct bitcoin transactions by offering a builder-type API that helps developers manage the typical requirements for building transactions: recipients, fees, signatures, and a whole lot more.

## Features

- **Flexible Input and Output Selection**: You can add or subtract inputs and outputs as needed.
- **Fees**: The builder calculates and adds fees automatically based on a specified fee rate or absolute target.
- **Finalization**: The builder helps finalize the transaction, ensuring signatures and other conditions are met.

## Usage

To start building a new transaction, instantiate the `TxBuilder` type provided by BDK by calling the [`Wallet::build_tx()`](https://docs.rs/bdk_wallet/latest/bdk_wallet/struct.Wallet.html#method.build_tx) method. On this builder you chain various methods to add inputs, outputs, and configure other settings. Finally, call [`TxBuilder::finish()`](https://docs.rs/bdk_wallet/latest/bdk_wallet/tx_builder/struct.TxBuilder.html#method.finish) to get a PSBT ready for signing.

```rust
let psbt = wallet.build_tx()
    .add_recipient(address.script_pubkey(), amount)
    .finish()?;
```

### Customizing Outputs

You can specify multiple recipients for the transaction, distributing funds across various addresses. The output is added as a script with an amount:

```rust
wallet.build_tx()
    .add_recipient(to_address1.script_pubkey(), amount1)
    .add_recipient(to_address2.script_pubkey(), amount2)
```

### Choosing Inputs

You can manually select specific UTXOs, as well as request to _not_ spend specific UTXOs:

```rust
wallet.build_tx()
    .add_utxo(utxo1)?
    .add_unspendable(utxo2)
    .add_recipient(address.script_pubkey(), amount)
```

Combining the above with `TxBuilder::manually_selected_only` will ensure that the wallet only spends UTXOs specified by the `TxBuilder::add_utxo` method.

Alternatively, let the wallet choose the inputs:

```rust
wallet.build_tx()
    .add_recipient(address.script_pubkey(), amount)
```

### Fee Management

The builder allows you to define a fee rate or an absolute fee:

```rust
wallet.build_tx()
    .fee_rate(FeeRate::from_sat_per_vb(4))
    .fee_absolute(Amount::from_sat(600))
```

Note that if you set the fee using both the fee_absolute and the fee_rate method, the wallet will use method was called last, as the fee rate and the absolute fee amount are mutually exclusive.

### Spend all funds

The transaction builder has a convenience method that will spend all UTXOs available to it (while respecting the unspendable UTXOs if any). Simple use of this method will simply increase the size of your change output, but you can combine it with the `TxBuilder::drain_to` method to ensure all funds are sent to a specific address:

```rust
let psbt = wallet.build_tx()
    .drain_wallet()
    .drain_to(address.script_pubkey())
```

### Set the nSequence value

The [`TxBuilder::set_exact_sequence`](https://docs.rs/bdk_wallet/1.0.0-beta.5/bdk_wallet/struct.TxBuilder.html#method.set_exact_sequence
) method allows users to set their nSequence value directly. This is also the way you would disable signaling for RBF, as the TxBuilder will by default enable it.

```rust
wallet.build_tx()
    .add_recipient(address.script_pubkey(), amount)
    .set_exact_sequence(sequence)
```

### Finalizing the PSBT

Once you’ve added inputs, outputs, and calculated the fees, you can build the psbt using the `TxBuilder::finish` method.

```rust
let psbt = wallet.build_tx()
    .finish()?;
```

The result is a PSBT ready to be signed and later on broadcast to the network.

## Signing the PSBT and extrating the transaction

Sign the PSBT using the `Wallet::sign` method and extract the finalized transaction from it using the Psbt::extract_tx` method for a ready-to-be-broadcast transaction.

```rust
wallet.sign(&mut psbt, SignOptions::default())?;
let tx = psbt.extract_tx()?;
```
