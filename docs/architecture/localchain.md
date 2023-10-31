# The `LocalChain` type
A good place to start understanding the bdk_chain crate is with the `LocalChain` type:
```rust
#[derive(Default)]
pub struct LocalChain {
    tip: CheckPoint,
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

## The `local_chain::Update` type
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
