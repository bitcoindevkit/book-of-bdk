# Anchors

There are a few versions of anchors, but they all have in common that they implement the trait `bdk_chain::Anchor`. For users of `bdk::Wallet`, the most important of these anchors is `bdk_chain::ConfirmationTimeHeightAnchor`.

You can think of an anchor as the glue that binds a transaction to a block. If a transaction is anchored in a block that is in the best chain, this transaction must therefore also be in the best chain. If that "anchor block" ever gets reorged and the transaction does not have any other anchors, the transaction becomes unconfirmed (since it's not tied to any particular block anymore!). A Transaction can have multiple anchors.

Every Anchor implementation contains a `BlockId` parameter, and must implement the `Ord` trait, meaning that any two anchors can always be compared and ordered relative to each other, facilitating sorting based on their `BlockId` values.

## ConfirmationTimeHeightAnchor

```rust
pub struct ConfirmationTimeHeightAnchor {
    /// The anchor block.
    pub anchor_block: BlockId,
    /// The confirmation height of the chain data being anchored.
    pub confirmation_height: u32,
    /// The confirmation time of the chain data being anchored.
    pub confirmation_time: u64,
}
```

A `TxGraph` is generic in its `Anchor` type.
