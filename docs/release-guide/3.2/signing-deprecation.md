# Deprecated Wallet-Owned Signing APIs

!!! note "Overview"

    * **Lead Developer:** [@noahjoeris](https://github.com/noahjoeris)
    * **Pull Request:** [#536](https://github.com/bitcoindevkit/bdk_wallet/pull/536) (backport of [#505](https://github.com/bitcoindevkit/bdk_wallet/pull/505))
    * **Feature Type:** Non-Breaking/Deprecation

## Overview

`bdk_wallet` 3.2.0 deprecates APIs for storing signing keys on `Wallet` and signing with them. They still compile; removal is planned for a later breaking release. See the [3.1 `Wallet::sign_with_signers` guide](../3.1/sign-with-signers.md) for the rationale and caller-owned signing flow.

New code should create wallets from public descriptors and keep keys outside the wallet. Private descriptors passed to `Wallet::create` still work and add keys for now.

## Deprecated APIs

| Deprecated                                                   | Replacement                                                                                                                                   |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `Wallet::sign`                                               | `Psbt::sign` (then finalize separately), or `Wallet::sign_with_signers` for `SignOptions` and finalization                                     |
| `Wallet::{add_signer, set_keymap, set_keymaps, get_signers}` | Keep your own `KeyMap` or `Xpriv`. Wrap a `KeyMap` in `KeyMapWrapper` for `Psbt::sign`, or build a `SignersContainer` for `sign_with_signers` |
| `CreateParams::keymap`, `LoadParams::{keymap, extract_keys}` | Same; do not attach extra keys during create or load                                                                                          |
| `Wallet::policies`                                           | `public_descriptor(keychain).extract_policy(...)` with caller-owned signers                                                                   |
| `TxBuilder::policy_path`                                     | `Policy::get_condition`, then `TxBuilder::set_condition`                                                                                      |

Descriptor secrets are not persisted, so loaded wallets do not recover them. Retain the keymap instead of calling `LoadParams::extract_keys`. `FullyNodedExport::export_wallet` now exports only public descriptors; retain private descriptors separately if you use them for recovery.

## Spending Conditions

For single-path descriptors, `TxBuilder` derives lock requirements without `set_condition`. Use it when choosing among paths with different timelocks. It replaces the signer-dependent IDs used by deprecated `policy_path` with a `Condition`.

In the example below, we extract a policy, select a path, derive its `Condition`, and pass it to `set_condition`. Signer availability and PSBT status are not needed to derive the lock requirements.

```rust
use std::collections::BTreeMap;

use bdk_wallet::descriptor::policy::BuildSatisfaction;
use bdk_wallet::descriptor::ExtractPolicy;
use bdk_wallet::signer::SignersContainer;
use bdk_wallet::KeychainKind;

let policy = wallet
    .public_descriptor(KeychainKind::External)
    .extract_policy(
        &SignersContainer::default(),
        BuildSatisfaction::None,
        wallet.secp_ctx(),
    )?
    .expect("descriptor has a spending policy");

let mut path = BTreeMap::new();
// Example: select branch 0 of a choice at the policy root.
path.insert(policy.id.clone(), vec![0]);
let condition = policy.get_condition(&path)?;

let mut builder = wallet.build_tx();
builder.set_condition(condition);
```

`set_condition` takes one `Condition` for the transaction. If both keychains may supply inputs with different lock requirements, combine their conditions with `Condition::merge`.

When the required lock is known, skip policy extraction and construct `Condition` directly. For an `older(6)` branch:

```rust
use bdk_wallet::bitcoin::Sequence;
use bdk_wallet::descriptor::Condition;

let mut builder = wallet.build_tx();
builder.set_condition(Condition {
    csv: Some(Sequence(6)),
    timelock: None,
});
```

`set_condition` takes precedence over `policy_path`. Without it, the deprecated path is resolved using wallet-owned signers, so only IDs from `Wallet::policies` match.
