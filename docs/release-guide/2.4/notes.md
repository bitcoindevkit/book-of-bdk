# 2.4 Release Notes

!!! info "Overview"

    * **Release Date:** April 28, 2026
    * **Changelog:** [Link](https://github.com/bitcoindevkit/bdk_wallet/blob/wallet-2.4.0/wallet/CHANGELOG.md)  

## Notable Changes

This release focuses on backporting enhancements from the 3.0 release to the 2.x line.

- [Wallet Event Methods](wallet-events.md): Three new event-returning methods have been backported from 3.0—`apply_unconfirmed_txs_events`, `apply_evicted_txs_events`, and `events_helper`—completing the event system introduced in 2.3.
- [Pre-v1 Migration Helper](pre-v1-migration.md): `get_pre_v1_wallet_keychains` is a new utility for users upgrading from a pre-1.0 BDK SQLite database. It reads the legacy database and returns keychain metadata needed to construct a matching post-1.0 wallet.

## Other Changes

- The deprecation of the `wallet::signer` module has been reverted until the new PSBT signer is ready. Code using `wallet::signer` will continue to compile without warnings.
