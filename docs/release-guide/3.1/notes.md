# 3.1 Release Notes

!!! info "Overview"

    * **Release Date:** June 14, 2026
    * **Changelog:** [Link](https://github.com/bitcoindevkit/bdk_wallet/blob/master/CHANGELOG.md)  

## Notable Changes

- [`Wallet::sign_with_signers`](sign-with-signers.md): A new method that gives callers fine-grained control over the signing process by accepting a custom list of `SignersContainer`. The existing `Wallet::sign` now delegates to it, leaving the existing API unchanged.
- [`LoadParams::two_path_descriptor`](two-path-descriptor.md): A convenience method on `LoadParams` for validating that both the external and internal keychains match an expected two-path descriptor when loading a wallet from persistence.

## Bug Fixes

This release includes a number of important bug fixes:

- Coin selection now returns required UTXOs first ([#390](https://github.com/bitcoindevkit/bdk_wallet/pull/390))
- Wallet name generation now uses the explicit public descriptor checksum ([#482](https://github.com/bitcoindevkit/bdk_wallet/pull/482))
- Fixed per-UTXO fee rounding in coin selection ([#479](https://github.com/bitcoindevkit/bdk_wallet/pull/479))
- Fixed non-witness UTXO validation in `add_foreign_utxo` and PSBT signing ([#461](https://github.com/bitcoindevkit/bdk_wallet/pull/461), [#471](https://github.com/bitcoindevkit/bdk_wallet/pull/471))
- Fixed panics on hardened derivation paths in PSBT key origins ([#458](https://github.com/bitcoindevkit/bdk_wallet/pull/458))
- Fixed `MultiXPrv`/`MultiXPub` handling in network kind detection ([#459](https://github.com/bitcoindevkit/bdk_wallet/pull/459))
- Fixed descriptor index tracking in `combinations()` ([#453](https://github.com/bitcoindevkit/bdk_wallet/pull/453))
- Fixed variable binding names in `l:` and `u:` descriptor fragment modifiers ([#454](https://github.com/bitcoindevkit/bdk_wallet/pull/454))
- Fixed `allow_all_sighashes` being ignored in `SignerWrapper::sign_input` ([#476](https://github.com/bitcoindevkit/bdk_wallet/pull/476))
- Fixed reachable panics in `Utxo::txout` for the `Foreign` variant ([#487](https://github.com/bitcoindevkit/bdk_wallet/pull/487))
