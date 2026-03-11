# UTXO Locking API

!!! note "Overview"

    * **Lead Developer:** [@ValuedMammal]
    * **Tickets:** [#166]
    * **Pull Request:** [#259]
    * **Feature Type:** Breaking

## Overview

New methods on `Wallet` allow users to lock and unlock UTXOs by outpoint. Locked UTXOs are excluded from coin selection and their lock status is persisted in the wallet's `ChangeSet`.

## Why Do This?

There are situations where users want to permanently exclude a UTXO from being spent, or reserve it for a future transaction while continuing to make other transactions in the meantime. Previously there was no way to express or persist this intent, and coin selection had no concept of locked outputs. This API fills that gap.

## New Methods on `Wallet`

```rust
// Lock a UTXO so it is excluded from coin selection
wallet.lock_outpoint(outpoint)?;

// Unlock a previously locked UTXO
wallet.unlock_outpoint(outpoint);

// Check whether a specific outpoint is locked
let locked: bool = wallet.is_outpoint_locked(outpoint);

// Iterate over all locked outpoints
let locked_outpoints: Vec<OutPoint> = wallet.list_locked_outpoints().collect();

// Iterate over locked outpoints that are also unspent (spendable once unlocked)
let locked_unspent: Vec<OutPoint> = wallet.list_locked_unspent().collect();
```

A locked UTXO remains excluded from coin selection until explicitly unlocked. To eventually spend the coin, the user must call `unlock_outpoint` first. Note that locking does not prevent a UTXO from being used as a manually selected input, so you retain full control.

## Breaking Change: New `ChangeSet` Field

Lock status is now persisted. This required adding a new field to the wallet `ChangeSet`:

- **Breaking:** `wallet::ChangeSet` has a new member field `locked_outpoints`

[@ValuedMammal]: https://github.com/ValuedMammal
[#166]: https://github.com/bitcoindevkit/bdk_wallet/issues/166
[#259]: https://github.com/bitcoindevkit/bdk_wallet/pull/259
[ChangeSet Version Compatibility]: changeset-versioning.md
