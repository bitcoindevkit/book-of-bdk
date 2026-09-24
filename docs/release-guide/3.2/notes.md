# 3.2 Release Notes

!!! info "Overview"

    * **Release Date:** September 22, 2026
    * **Changelog:** [Link](https://github.com/bitcoindevkit/bdk_wallet/blob/v3.2.0/CHANGELOG.md)  

## Notable Changes

- [`Wallet::create_psbt`](create-psbt.md): A new, *unstable* API for building PSBTs using `PsbtParams`, built on miniscript planning and `bdk_tx`. It ships alongside `Wallet::replace_by_fee` and is only available if you enable the `bdk-tx` cargo feature and the `--cfg bdk_wallet_unstable` rustc flag.
- [Deprecated Signing APIs](signing-deprecation.md): APIs that store signing keys on `Wallet` and sign with them (`Wallet::sign`, `add_signer`, `set_keymap`, `policies`, `TxBuilder::policy_path`, and others) are now deprecated. Use caller-owned keys with `Psbt::sign` or `Wallet::sign_with_signers` instead. `TxBuilder::set_condition` is the new way to choose a spending path.
- `Wallet::try_finalize_psbt`: A new finalization method that returns a `FinalizePsbtOutcome` with the result for each input, so you can see which inputs could not be finalized and why. `Wallet::finalize_psbt` has not changed ([#433](https://github.com/bitcoindevkit/bdk_wallet/pull/433)).

## Bug Fixes

- Unconfirmed UTXOs are now filtered according to BIP-431 (TRUC) rule 2 during coin selection, so v3 transactions only spend unconfirmed outputs from v3 transactions, and non-v3 transactions only spend unconfirmed outputs from non-v3 transactions ([#478](https://github.com/bitcoindevkit/bdk_wallet/pull/478))
- Fixed enforcement of the write-once `descriptor`, `change_descriptor`, and `network` fields of `ChangeSet`, which could previously be overwritten in release builds ([#517](https://github.com/bitcoindevkit/bdk_wallet/pull/517))
