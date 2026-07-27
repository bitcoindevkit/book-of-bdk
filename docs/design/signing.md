# Signing Transactions Without Wallet

BDK is moving toward a cleaner separation between the two jobs the `Wallet` type previously performed:

1. Derives addresses, tracks UTXOs and balances, and builds transactions.
2. Holds private key material and sign PSBTs.

These two responsibilities do not need to be coupled. As of `bdk_wallet` 3.1.0, there are two supported paths for signing your transactions without ever embedding private keys into the `Wallet` itself. The recommended approach is to call `Psbt::sign` from rust-bitcoin directly on the PSBT. For cases where you need fine-grained control over signing behavior via `SignOptions`, the new `Wallet::sign_with_signers` method is also available.

Over time, the ability to pass private key material to the `Wallet` constructor will be deprecated entirely.

## Why Do This?

In previous workflows it was common to construct a `Wallet` with a private descriptor, letting BDK both track coins and sign transactions. Mixing these responsibilities has a few downsides:

- **Private key material lives in the same process as network I/O and coin selection.** This is not necessary, and keeping the responsibility for private keys separate is simply good practice.
- **It doesn't model most real signing architectures.** In practice, keys often live on a hardware device, in a separate process, or behind an HSM. Embedding them in the `Wallet` for the full lifetime of the process is unnecessary, and many applications were already separating these concerns in less ergonomic ways than is now possible.
- **PSBTs exist precisely to separate these roles.** The PSBT format was designed so that an unsigned transaction can be handed off to one or more independent signers. Keeping that separation in code leverages the power of that design.

## A New Potential Signature Flow

### 1. Create the Wallet with Public Descriptors

Use public descriptors to create your `Wallet`. The wallet can derive addresses, track UTXOs, and build transactions without ever holding a private key.

```rust
let public_descriptor: &str = "tr([5bc5d243/86'/1'/0']tpubDC72NVP1.../0/*)#xh44xwsp";
let public_change_descriptor: &str = "tr([5bc5d243/86'/1'/0']tpubDC72NVP1.../1/*)#hrs5mmqe";

let wallet = Wallet::create(descriptor, change_descriptor)
    .network(Network::Regtest)
    .create_wallet_no_persist()?;
```

### 2. Build a PSBT with `TxBuilder`

```rust
let recipient = Address::from_str("bcrt1qxzh0r7mlztv3m8vxet5xxnsy9zh7j5tshh6vhp")?.assume_checked();

let mut psbt = {
    let mut builder = wallet.build_tx();
    builder.add_recipient(recipient.script_pubkey(), Amount::from_sat(50_000));
    builder.finish()?
};
```

### 3. Sign the PSBT

There are two supported approaches. We recommend the `Psbt::sign` approach, unless you have special requirements that need `SignOptions`.

#### Option A: `Psbt::sign` (Recommended)

Load your private descriptor separately and call `Psbt::sign` directly on the PSBT. This approach uses rust-bitcoin's own signing machinery and does not involve the `Wallet` at all at signing time.

```rust
use bdk_wallet::miniscript::Descriptor;
use bdk_wallet::miniscript::DescriptorPublicKey;

let private_descriptor: &str = "tr(tprv8ZgxMBic.../86'/1'/0'/0/*)#x627tk5a";
let private_change_descriptor: &str = "tr(tprv8ZgxMBic.../86'/1'/0'/1/*)#hw0lkry9";
let secp_ctx = wallet.secp_ctx();

let (_, receive_keymap) = Descriptor::parse_descriptor(secp_ctx, private_descriptor).unwrap();
let (_, change_keymap) = Descriptor::parse_descriptor(secp_ctx, private_change_descriptor).unwrap();

let mut combined_keymap = receive_keymap;
combined_keymap.extend(change_keymap);

psbt.sign(&KeyMapWrapper::from(combined_keymap), secp_ctx).unwrap();
psbt.finalize_mut(secp_ctx).unwrap();
```

`Psbt::sign` accepts anything that implements rust-bitcoin's [`GetKey`](https://docs.rs/bitcoin/latest/bitcoin/psbt/trait.GetKey.html) trait. In miniscript 12.x, `KeyMap` does not implement `GetKey` directly — it must be wrapped in a `KeyMapWrapper` first, as shown in the example above.

#### Option B: `Wallet::sign_with_signers`

If you rely on `SignOptions` for custom signing behavior such as overriding sighash types, relaxing locktime enforcement, or other settings not available on `Psbt::sign`, use `Wallet::sign_with_signers`. This method accepts an explicit list of `SignersContainer` objects, processes them in the order provided, and returns `true` if the PSBT was finalized.

```rust
use bdk_wallet::descriptor::IntoWalletDescriptor;
use bdk_wallet::signer::SignersContainer;
use bdk_wallet::{KeychainKind, SignOptions};
use bitcoin::{secp256k1::Secp256k1, NetworkKind};

let private_descriptor: &str = "tr(tprv8ZgxMBic.../86'/1'/0'/0/*)#x627tk5a";
let private_change_descriptor: &str = "tr(tprv8ZgxMBic.../86'/1'/0'/1/*)#hw0lkry9";
let secp_ctx = wallet.secp_ctx();

let (_, receive_keymap) = Descriptor::parse_descriptor(secp_ctx, private_descriptor).unwrap();
let (_, change_keymap) = Descriptor::parse_descriptor(secp_ctx, private_change_descriptor).unwrap();

let receive_signers_container = SignersContainer::build(
    receive_keymap,
    wallet.public_descriptor(KeychainKind::External),
    secp_ctx,
);
let change_signers_container = SignersContainer::build(
    change_keymap,
    wallet.public_descriptor(KeychainKind::Internal),
    secp_ctx,
);

let signers: &[&SignersContainer; 2] = &[&receive_signers_container, &change_signers_container];

let psbt_was_signed_and_finalized: bool = wallet
    .sign_with_signers(&mut psbt, signers, SignOptions::default())
    .unwrap();

assert!(psbt_was_signed_and_finalized);
```

### 4. Broadcast

```rust
let tx = psbt.extract_tx()?;
client.broadcast(&tx)?;
```

## Summary

| Step | Responsibility | API |
|------|----------------|-----|
| Address derivation & coin tracking | `Wallet` (public descriptor) | `Wallet::create` |
| Transaction building | `Wallet` | `Wallet::build_tx()` |
| Signing (recommended) | Your key store | `Psbt::sign` (rust-bitcoin) |
| Signing (custom options) | Your key store | `Wallet::sign_with_signers` (bdk_wallet 3.1.0+) |
| Finalization & broadcast | `Wallet` + client | `psbt.finalize_mut`, `client.broadcast` |

This separation means your signing code can be swapped out or upgraded independently of your wallet logic. Private key material never needs to touch the same code paths as network I/O or persistence. Over time, the `Wallet` will move toward not accepting private keys at construction at all.

[rust-bitcoin]: https://docs.rs/bitcoin/latest/bitcoin/psbt/struct.Psbt.html#method.sign
