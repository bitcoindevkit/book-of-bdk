# 3.0 (Release Candidate 1) Release Notes

!!! info "Overview"

    * **Release Date:** March 10 (release candidate 1)
    * **Changelog:** [Link](https://github.com/bitcoindevkit/bdk_wallet/blob/master/CHANGELOG.md)  

## Notable Changes

- [UTXO Locking API](utxo-locking.md): There are new methods on `Wallet` to lock and unlock UTXOs by outpoint, with lock status persisted in `ChangeSet`.
- [Caravan Import/Export](caravan-export.md): We added support for importing and exporting wallets in the Caravan multisig coordinator format.
- [ChangeSet Version Compatibility](changeset-versioning.md): We formalized our policy around version compatibility for `ChangeSet`, including supported upgrade/downgrade paths and a deprecation cycle.
- [PSBT Signing Flow](psbt-signing.md): Starting in 3.0, we now recommend a new pattern for signing PSBTs using `Psbt::sign` from rust-bitcoin, and keeping private keys out of the `Wallet`.
