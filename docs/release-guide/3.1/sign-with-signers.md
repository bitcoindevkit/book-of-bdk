# `Wallet::sign_with_signers` API

!!! note "Overview"

    * **Lead Developer:** [@noahjoeris]
    * **Pull Request:** [#490]
    * **Feature Type:** Non-Breaking

## Overview

`Wallet::sign_with_signers` is a new method that accepts an explicit list of `SignersContainer` objects and signs a PSBT using them in the order provided. The existing `Wallet::sign` method now simply delegates to `sign_with_signers`, passing the wallet's own internal signer containers, so there is no change to existing callers.

## Why Do This?

Previously, the only way to sign a PSBT with BDK was through `Wallet::sign`, which always used the signers that were embedded in the wallet at construction time. This made it difficult to:

- Sign with external hardware signers or air-gapped devices whose keys are not stored inside the wallet
- Compose multiple signing devices in a custom order
- Keep the wallet itself key-free (watch-only) while still driving the signing flow

`sign_with_signers` decouples the signing logic from wallet-internal key storage, and is a step toward eventually allowing `Wallet` to be fully signer-agnostic.

## New Method on `Wallet`

```rust
pub fn sign_with_signers(
    &self,
    psbt: &mut Psbt,
    signers: &[&SignersContainer],
    sign_options: SignOptions,
) -> Result<bool, SignerError>
```

Signer containers are processed in the order provided. Within each container, signers are ordered by their [`SignerOrdering`]. The method returns `true` if the PSBT was finalized, or `false` if further signatures are still needed.

## Example: Signing with an External Signer

The following example builds a `SignersContainer` from a raw descriptor containing a private key (simulating an external signer), then passes it to `sign_with_signers`:

```rust
use bdk_wallet::*;
use bdk_wallet::bitcoin::*;
use bdk_wallet::bitcoin::{NetworkKind, secp256k1::Secp256k1};
use bdk_wallet::descriptor::IntoWalletDescriptor;
use bdk_wallet::signer::SignersContainer;

// Descriptor with private key material — represents an external signer
let signer_descriptor = "tr([73c5da0a/86'/0'/0']tprv8fMn4hSKPRC1oaCPqxDb1JWtgkpeiQvZhsr8W2xuy3GEMkzoArcAWTfJxYb6Wj8XNNDWEjfYKK4wGQXh3ZUXhDF2NcnsALpWTeSwarJt7Vc/0/*)";

let secp = Secp256k1::new();
let (_, keymap) = signer_descriptor
    .into_wallet_descriptor(&secp, NetworkKind::Test)
    .unwrap();

// Build a SignersContainer from the keymap
let external_signers = SignersContainer::build(
    keymap,
    wallet.public_descriptor(KeychainKind::External),
    wallet.secp_ctx(),
);

// Build and sign a transaction
let to_address = wallet.next_unused_address(KeychainKind::External).address;
let mut psbt = {
    let mut builder = wallet.build_tx();
    builder.drain_to(to_address.script_pubkey()).drain_wallet();
    builder.finish()?
};

let finalized = wallet.sign_with_signers(
    &mut psbt,
    &[&external_signers],
    SignOptions::default(),
)?;
assert!(finalized);
```

## Relationship to `Wallet::sign`

`Wallet::sign` is unchanged and continues to work exactly as before. Internally it now calls:

```rust
self.sign_with_signers(
    psbt,
    &[self.signers.as_ref(), self.change_signers.as_ref()],
    sign_options,
)
```

Callers who already use `Wallet::sign` do not need to change anything.

[@noahjoeris]: https://github.com/noahjoeris
[#490]: https://github.com/bitcoindevkit/bdk_wallet/pull/490
