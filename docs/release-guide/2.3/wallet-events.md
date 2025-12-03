---
hide:
  - toc
  - footer
---

# Applying Blocks to a Wallet Returns A List of Events

!!! note "Overview"

    * **Lead Developer:** [@tnull]
    * **Ticket:** [#6]
    * **Pull Request:** [#336]
    * **ADR:** [Link]
    * **Feature Type:** Non-breaking

## Overview

New methods are provided for applying a block to the `Wallet` that return a list of events the block generates.

```rust
let events: Vec<WalletEvent> = wallet.apply_block_events(block, height);
// OR
let events: Vec<WalletEvent> = wallet.apply_block_connected_to_events(block, height, connected_to);
```

## Why Do This?

When syncing a `Wallet` with new blockchain data using [Wallet::apply_block] or [Wallet::apply_block_connected_to], they do not return any value on success, and only a `CannotConnectError` if they fail. 

Users have asked for a concise list of events that reflect if or how new blockchain data has changed the blockchain tip and the status of transactions relevant to the wallet's balance. This information is useful for downstream libraries who rely on these events for triggering their own work, as well as for applications who want to notify users of wallet changes after syncing.

## Applying A Block to Wallet Now Returns `Vec<WalletEvent>`

This new feature returns `WalletEvent` user-facing events that are generated when a new block is applied to a wallet using the `Wallet::apply_block_events` or `Wallet::apply_block_connected_to_events` function. 

<br>

[@tnull]: https://github.com/tnull
[#6]: https://github.com/bitcoindevkit/bdk_wallet/issues/6
[#336]: https://github.com/bitcoindevkit/bdk_wallet/pull/336
[Wallet::apply_block]: https://docs.rs/bdk_wallet/2.3.0/bdk_wallet/struct.Wallet.html#method.apply_block
[Wallet::apply_block_connected_to]: https://docs.rs/bdk_wallet/2.3.0/bdk_wallet/struct.Wallet.html#method.apply_block_connected_to
[Link]: https://github.com/bitcoindevkit/bdk_wallet/blob/release/2.x/docs/adr/0003_events.md
