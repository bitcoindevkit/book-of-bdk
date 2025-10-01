---
hide:
  - toc
  - footer
---

# Wallet Support For Public Multipath Descriptors

!!! note "Overview"

    * **Lead Developer:** [@schjonhaug]
    * **Ticket:** [#11]
    * **Pull Request:** [#275]
    * **Feature Type:** Non-breaking

## Overview
  - We add a new API, `Wallet::create_multipath(descriptor)` following the same pattern as `create()` and `create_single()`.
  - [BIP 389](https://github.com/bitcoin/bips/blob/master/bip-0389.mediawiki) compliance with exactly 2-path validation (receive and change).
  - The multipath descriptor must be a public descriptor.
  
```rust title="Example"
let multipath_desc = "wpkh([9a6a2580/84'/1'/0']tpubabcde.../‹0;1›/*)";
let wallet = Wallet::create_multipath(multipath_desc)
      .network(Network::Testnet)
      .create_wallet_no_persist()?;

// Automatically creates separate receive and change descriptors
let receive_address = wallet.peek_address(KeychainKind::External, 0);  // Uses path /0/*
let change_address = wallet.peek_address(KeychainKind::Internal, 0);   // Uses path /1/*
```

## Why Do This?

Multipath descriptors are fairly common nowadays, and there was a lot of demand for supporting a constructor on the wallet that would handle them gracefully.

## Notes

- The constructor only allows exactly 2-path multipath descriptors to ensure proper receive/change separation.

<br>

[@schjonhaug]: https://github.com/schjonhaug
[#11]: https://github.com/bitcoindevkit/bdk_wallet/issues/11
[#275]: https://github.com/bitcoindevkit/bdk_wallet/pull/275
