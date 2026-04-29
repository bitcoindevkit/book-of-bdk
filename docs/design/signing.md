# A New Way of Signing Your Transactions

## Overview

Starting with [miniscript `13.0.0`]() there is a new way of signing transactions withing the bdk ecosystem, keeping private key material out of the `Wallet` entirely. This allows you to use public descriptors in the `Wallet` for address derivation and coin tracking, build transactions with `TxBuilder` to obtain a PSBT, and then later on sign that PSBT using the `Psbt::sign` API from [rust-bitcoin].

## Why Do This?

In previous workflows it was common to construct a `Wallet` with a private descriptor, letting BDK both track coins and sign transactions. Mixing these responsibilities has a few downsides:

- **Private key material lives in the same process as network I/O and coin selection.** This is not necessary, and keeping the responsibility for private keys separate is simply good practice.
- **It doesn't model most real signing architectures.** In practice, keys often live on a hardware device, in a separate process, or behind an HSM. Adding them to the `Wallet` for the duration of the process is unecessary, and many applications were already separating these concerns in less ergonomic ways than is now possible.
- **PSBTs exist precisely to separate these roles.** The PSBT format was designed so that an unsigned transaction can be handed off to one or more independent signers, and keeping that separation in code makes use of this design explicitly.

## A New Potential Signature Flow

### 1. Create the Wallet with Public Descriptors

Use public descriptors to create your `Wallet`. Note that the version of miniscript that bdk_wallet currently ships with (12.X) does not have the required API on the `Psbt` type. The examples below make this explicit by aliasing the `Descriptor` type to `Miniscript12Descriptor` and the signing descriptor to `Miniscript13Descriptor`.

```rust
use bdk_wallet::Wallet;
use bdk_wallet::miniscript::descriptor::Descriptor as Miniscript12Descriptor;
use bdk_wallet::keys::DescriptorPublicKey as Miniscript12DescriptorPublicKey;
use bitcoin::Network;

let descriptor = Miniscript12Descriptor::<Miniscript12DescriptorPublicKey>::from_str("tr([5bc5d243/86'/1'/0']tpubDC72NVP1.../0/*)#xh44xwsp").unwrap();

let wallet = Wallet::create_single(descriptor)
    .network(Network::Regtest)
    .create_wallet_no_persist()?;
```

The `Wallet` can now derive addresses, track UTXOs, and build transactions without ever holding a private key.

### 2. Build a PSBT with `TxBuilder`

```rust
use bdk_wallet::wallet::tx_builder::TxOrdering;
use bitcoin::Amount;

let recipient = Address::from_str("bc1q...")?.assume_checked();

let mut psbt = {
    let mut builder = wallet.build_tx();
    builder
        .add_recipient(recipient.script_pubkey(), Amount::from_sat(50_000));
    builder.finish()?
};
```

### 3. Sign with `Psbt::sign`

Load your private key separately from secure storage, a hardware wallet integration, or an air-gapped device, and call `Psbt::sign` directly on the PSBT:

```rust
use miniscript::Descriptor as Miniscript13Descriptor;
use miniscript::DescriptorPublicKey as Miniscript13DescriptorPublicKey;

// You must use miniscript 13.0.0 or above for this to work, see https://github.com/rust-bitcoin/rust-miniscript/pull/851
let secp = bitcoin::secp256k1::Secp256k1::new();
let result: (Miniscript13Descriptor<Miniscript13DescriptorPublicKey>, KeyMap) = Miniscript13Descriptor::parse_descriptor(&secp, "tr(tprv8ZgxMBicQKsPdWAHbugK.../86'/1'/0'/0/*)#x627tk5a").unwrap();
let keymap = &result.1;
psbt.sign(keymap, &secp);
```

`Psbt::sign` accepts anything that implements rust-bitcoin's [`GetKey`](https://docs.rs/bitcoin/0.32.8/bitcoin/psbt/trait.GetKey.html) trait.

### 4. Finalize and Broadcast

```rust
use bdk_wallet::miniscript::psbt::PsbtExt;

let finalized = psbt.finalize(&secp)?;
let tx = psbt.extract_tx()?;
client.broadcast(&tx)?;
```

## Summary

| Step | Responsibility | API |
|------------------------------------|------------------------------|---------------------------------------------|
| Address derivation & coin tracking | `Wallet` (public descriptor) | `Wallet::create`                            |
| Transaction building               | `Wallet`                     | `Wallet::build_tx()`                        |
| Signing                            | Your key store               | `Psbt::sign` (rust-bitcoin)                 |
| Finalization & broadcast           | `Wallet` + client            | `Wallet::finalize_psbt`, `client.broadcast` |

This separation means your signing code can be swapped out or upgraded independently of your wallet logic, and private key material never needs to touch the same code paths as network I/O or persistence.

[@oleonardolima]: https://github.com/oleonardolima
[#70]: https://github.com/bitcoindevkit/bdk_wallet/issues/70
[#235]: https://github.com/bitcoindevkit/bdk_wallet/pull/235
[rust-bitcoin]: https://docs.rs/bitcoin/latest/bitcoin/psbt/struct.Psbt.html#method.sign
