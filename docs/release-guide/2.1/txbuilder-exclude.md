---
hide:
  - toc
  - footer
---

# Exclude Inputs Based On Maturity When Building Transactions

!!! note "Overview"

    * **Lead Developer:** [@evanlinjin]
    * **Ticket:** [#143]
    * **Pull Request:** [#258]
    * **Feature Type:** Non-breaking

## Overview

- Two methods are added to the TxBuilder, allowing users to exclude inputs that are either (a) unconfirmed, or (b) below a certain confirmation threshold.

```rust title="Example 1"
let mut builder = wallet.build_tx();
builder
    .fee_rate(FeeRate::ZERO)
    .add_recipient(recipient_address.script_pubkey(), Amount::from_sat(50_000))
    .exclude_unconfirmed();
let tx = builder.finish().unwrap();
```

```rust title="Example 2"
let mut builder = wallet.build_tx();
builder
    .fee_rate(FeeRate::ZERO)
    .add_recipient(recipient_address.script_pubkey(), Amount::from_sat(50_000))
    .exclude_below_confirmations(6);
let tx = builder.finish().unwrap();
```

## Why Do This?

These are convenience methods that fit a broad number of use cases. Previously, users had to do custom UTXO selection if they needed to ensure they used inputs that were confirmed onchain, or had a certain number of confirmations. This allows users to keep the BDK coins selection algos working their magic while also refraining from picking inputs that are not yet confirmed or within a specific confirmation threshold.

<br>

[@evanlinjin]: https://github.com/evanlinjin
[#143]: https://github.com/bitcoindevkit/bdk_wallet/issues/143
[#258]: https://github.com/bitcoindevkit/bdk_wallet/pull/258
