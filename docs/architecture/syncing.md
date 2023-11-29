# Understanding the `bdk_chain` crate
The bdk_chain crate handles everything to do with bdk's internal representation of blockchain state and data. It's main types are `LocalChain` and `TxGraph`.

The goal of this crate is to give wallets the mechanisms needed to:

1. Figure out what data they need to fetch.
2. Process the data in a way that never leads to inconsistent states.
3. Fully index that data and expose it to be consumed without friction.

The design goals for these mechanisms are:

1. Data source agnostic — nothing in `bdk_chain` cares about where you get data from or whether you do it synchronously or asynchronously. If you know a fact about the blockchain, you can just tell `bdk_chain`'s APIs about it, and that information will be integrated if it can be done consistently.
2. Error-free APIs.
3. Data persistence agnostic — `bdk_chain` does not care where you cache on-chain data, what you
   cache or how you fetch it.

## 1a. The `LocalChain` type
A good place to start understanding the bdk_chain crate is with the `LocalChain` type:
```rust
#[derive(Default)]
pub struct LocalChain {
    tip: Option<CheckPoint>,
    index: BTreeMap<u32, BlockHash>,
}
```

A `LocalChain` can be thought of as a loose collection of blocks we know about. This local chain can be sparse, i.e. it does not have to know about all blocks in order. 

- The `index` field holds an ordered map (the `BTreeMap` type) of all blocks it knows about with their given block hash. Note that because the block heights are the keys to this map, there can only be one blockhash for any given block height (you can't have two blocks 100 for example).
- The `tip` field is an optional [`CheckPoint`](#) type.

Two `LocalChain`s are considered equal if their `index` field is the same.

You can initially create a `LocalChain` by calling the `default()` associated function on it, which will return a `LocalChain` with no checkpoint and an empty map:
```rust
LocalChain {
    tip: None,              // No checkpoint
    index: BTreeMap::new(), // An empty map
}
```

This local chain can now be hydrated with the `LocalChain.apply_changeset()` method if you have a `ChangeSet` saved to persistence, or simply start applying updates coming from a client using the `LocalChain.apply_update()`.

<br>

## 1b. The `local_chain::Update` type
To update a `LocalChain`, you must provide it with an `Update`. The type is defined as such:
```rust
/// A struct to update [`LocalChain`].
///
/// This is used as input for [`LocalChain::apply_update`]. It contains the update's chain `tip` and
/// a flag `introduce_older_blocks` which signals whether this update intends to introduce missing
/// blocks to the original chain.
///
/// Block-by-block syncing mechanisms would typically create updates that builds upon the previous
/// tip. In this case, `introduce_older_blocks` would be `false`.
///
/// Script-pubkey based syncing mechanisms may not introduce transactions in a chronological order
/// so some updates require introducing older blocks (to anchor older transactions). For
/// script-pubkey based syncing, `introduce_older_blocks` would typically be `true`.
pub struct Update {
    /// The update chain's new tip.
    pub tip: CheckPoint,

    /// Whether the update allows for introducing older blocks.
    pub introduce_older_blocks: bool,
}
```
