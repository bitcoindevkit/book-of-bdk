---
hide:
  - toc
---

# 2.1 Release Notes

!!! note "Overview"

    * **Release Date:** August 6, 2025
    * **Changelog:** [Link](https://github.com/bitcoindevkit/bdk_wallet/blob/wallet-2.1.0/wallet/CHANGELOG.md)  

## Notable Changes

- The transaction builder adds 2 new methods: `TxBuilder::exclude_unconfirmed` and `TxBuilder::exclude_below_confirmations`. [See details here](./txbuilder-exclude.md).
- Wallets can now be created using public multipath descriptors. [See details here](./multipath.md).
