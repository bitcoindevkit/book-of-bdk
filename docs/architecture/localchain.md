# The `LocalChain` type
A good place to start understanding the bdk_chain crate is with the `LocalChain` type:
```rust
#[derive(Default)]
pub struct LocalChain {
    tip: CheckPoint,
    index: BTreeMap<u32, BlockHash>,
}
```

A `LocalChain` can be thought of as a loose collection of blocks we know about. This local chain can be sparse, i.e. it does not have to know about all blocks in order. The LocalChain type has two fields, `tip` and `index`:

- The `tip` field is an optional [`CheckPoint`](./checkpoint.md) type.
- The `index` field holds an ordered map (the `BTreeMap` type) of all blocks it knows about with their given block hash. Note that because the block heights are the keys to this map, there can only be one blockhash for any given block height (you can't have two blocks 100 for example).

Two `LocalChain`s are considered equal if their `index` field is the same.

The following code explores a small LocalChain example:
```rust
let local_chain = LocalChain::from_blocks(
    [
        (0, Hash::hash("zero".as_bytes())),
        (1, Hash::hash("first".as_bytes())),
        (2, Hash::hash("second".as_bytes())),
        (3, Hash::hash("third".as_bytes())),
        (12, Hash::hash("twelve".as_bytes())),
    ]
    .into_iter()
    .collect::<BTreeMap<u32, BlockHash>>(),
).unwrap();

println!("Local chain: \n{:#?}\n", local_chain);
```

```rust
LocalChain {
    tip: CheckPoint(
        CPInner {
            block: BlockId {
                height: 12,
                hash: 0x91a825c5c1eea6886cda4e98dac99d915697c362e19a2920d5a242e9b4fc5922,
            },
            prev: Some(
                CPInner {
                    block: BlockId {
                        height: 3,
                        hash: 0xb3803c0a544bad22bd52594014848a1dbf1a6308b69a4dbbb00306f9d9f3cb96,
                    },
                    prev: Some(
                        CPInner {
                            block: BlockId {
                                height: 2,
                                hash: 0x928411406d12ade8e2d0dfeb43f2d165923595cb68d89561c2ae7fc6b935840b,
                            },
                            prev: Some(
                                CPInner {
                                    block: BlockId {
                                        height: 1,
                                        hash: 0xcf0b7afa0779ec616649ecada0e3711b2acee4e5631289ef615b167cb0ac9f4b,
                                    },
                                    prev: Some(
                                        CPInner {
                                            block: BlockId {
                                                height: 0,
                                                hash: 0x2cf2de24e85d6179e06f842e74accef3bfa8a3fe0e194fafa30045c9a9187c92,
                                            },
                                            prev: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    ),
    index: {
        0: 0x2cf2de24e85d6179e06f842e74accef3bfa8a3fe0e194fafa30045c9a9187c92,
        1: 0xcf0b7afa0779ec616649ecada0e3711b2acee4e5631289ef615b167cb0ac9f4b,
        2: 0x928411406d12ade8e2d0dfeb43f2d165923595cb68d89561c2ae7fc6b935840b,
        3: 0xb3803c0a544bad22bd52594014848a1dbf1a6308b69a4dbbb00306f9d9f3cb96,
        12: 0x91a825c5c1eea6886cda4e98dac99d915697c362e19a2920d5a242e9b4fc5922,
    },
}
```

You can initially create a `LocalChain` by calling the `default()` associated function on it, which will return a `LocalChain` with no checkpoint and an empty map:
```rust
LocalChain {
    tip: None,              // No checkpoint
    index: BTreeMap::new(), // An empty map
}
```

This local chain can now be hydrated with the `LocalChain.apply_changeset()` method if you have a `ChangeSet` saved to persistence, or simply start applying updates coming from a client using the `LocalChain.apply_update()`.

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

!!! tip
    TODO: Expand on the ways localchains are combined and merged.

## Interacting with LocalChain

You mostly interact with a `LocalChain` through its `apply_update(update: Update)` and `apply_changeset(changeset: ChangeSet)` methods. Let's explore those in turn.

An update will usually be given to you by a client (TODO: verify and clarify this). But you can create an `Update` for testing purposes and update your `LocalChain` like so:

```rust
let mut chain = LocalChain::from_blocks(
    [
        (0, Hash::hash("zero".as_bytes())),
        (1, Hash::hash("first".as_bytes())),
        (2, Hash::hash("second".as_bytes())),
        (3, Hash::hash("third".as_bytes())),
    ]
    .into_iter()
    .collect::<BTreeMap<u32, BlockHash>>(),
).unwrap();

let other_chain = LocalChain::from_blocks(
    [
        (0, Hash::hash("zero".as_bytes())),
        (3, Hash::hash("third".as_bytes())),
        (5, Hash::hash("fifth".as_bytes())),
    ]
    .into_iter()
    .collect::<BTreeMap<u32, BlockHash>>(),
).unwrap();

let update = Update {
    tip: other_chain.tip(),
    introduce_older_blocks: true,
};

println!("################  Chain before update  #####################\n{:#?}\n", chain);
let changeset = chain.apply_update(update);
println!("################  Chain after update  #####################\n{:#?}\n", chain);
```

You'll find that the chain after update has simply incorporated the new `CheckPoint`s (its tip in the example now points to CheckPoint with BlockId.height 5).

```rust
// ################  Chain before update  #####################

LocalChain {
    tip: CheckPoint(
        CPInner {
            block: BlockId {
                height: 3,
                hash: 0xb3803c0a544bad22bd52594014848a1dbf1a6308b69a4dbbb00306f9d9f3cb96,
            },
            prev: Some( ... ), // Shortened for documentation
        },
    ),
    index: {
        0: 0x2cf2de24e85d6179e06f842e74accef3bfa8a3fe0e194fafa30045c9a9187c92,
        1: 0xcf0b7afa0779ec616649ecada0e3711b2acee4e5631289ef615b167cb0ac9f4b,
        2: 0x928411406d12ade8e2d0dfeb43f2d165923595cb68d89561c2ae7fc6b935840b,
        3: 0xb3803c0a544bad22bd52594014848a1dbf1a6308b69a4dbbb00306f9d9f3cb96,
    },
}

// ################  Chain after update  #####################

LocalChain {
    tip: CheckPoint(
        CPInner {
            block: BlockId {
                height: 5,
                hash: 0x5e890ebfef4ad8a21224842027383a02b6ebbe9056f7f924e838d9f62f2323af,
            },
            prev: Some(
                CPInner {
                    block: BlockId {
                        height: 3,
                        hash: 0xb3803c0a544bad22bd52594014848a1dbf1a6308b69a4dbbb00306f9d9f3cb96,
                    },
                    prev: Some( ... ), // Shortened for documentation
                },
            ),
        },
    ),
    index: {
        0: 0x2cf2de24e85d6179e06f842e74accef3bfa8a3fe0e194fafa30045c9a9187c92,
        1: 0xcf0b7afa0779ec616649ecada0e3711b2acee4e5631289ef615b167cb0ac9f4b,
        2: 0x928411406d12ade8e2d0dfeb43f2d165923595cb68d89561c2ae7fc6b935840b,
        3: 0xb3803c0a544bad22bd52594014848a1dbf1a6308b69a4dbbb00306f9d9f3cb96,
        5: 0x5e890ebfef4ad8a21224842027383a02b6ebbe9056f7f924e838d9f62f2323af,
    },
}
```

You'll find that applying an update (chain.apply_update(update: Update)) returns a `ChangeSet`. This changeset is what you'd persist.

In the example above, printing the `changeset` variable will return the following:

```rust
let changeset = chain.apply_update(update);
println!("## Changeset ##\n{:#?}\n", changeset);

// ## Changeset ##
Ok(
    {
        5: Some(
            0x5e890ebfef4ad8a21224842027383a02b6ebbe9056f7f924e838d9f62f2323af,
        ),
    },
)
```
