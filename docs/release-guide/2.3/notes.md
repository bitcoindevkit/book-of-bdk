---
hide:
  - toc
---

# 2.3 Release Notes

!!! note "Overview"

    * **Release Date:** December 5, 2025
    * **Changelog:** [Link](https://github.com/bitcoindevkit/bdk_wallet/blob/wallet-2.3.0/wallet/CHANGELOG.md)  

## Notable Changes

- Adds new methods `Wallet::apply_block_events` and `Wallet::apply_block_connected_to_events`. [Learn more about this feature here](./wallet-events.md).
- A fix was added to `Wallet::build_fee_bump` so that it does not throw an error when missing a parent transaction when the input UTXO was added to the original, pre-fee-bump transaction with `TxBuilder::add_foreign_utxo`.

<br>
