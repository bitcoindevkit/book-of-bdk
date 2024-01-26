# CheckPoints

A `CheckPoint` is a type that contains a `BlockId` (height and hash), and potentially a pointer to a previous `CheckPoint`. For example, printing the CheckPoint for the genesis block of testnet, you get this:

```rust
CheckPoint(
    CPInner { 
        block: BlockId {
            height: 0,
            hash: 0x000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943
        },
        prev: None 
    }
)
```

You can see an example of how checkpoints chain into each other by building a `LocalChain` and printing the `tip` field, which contains its recentmost checkpoint (which in turns points to a previous checkpoint, all the way down to genesis) like so:

```rust
let local_chain = LocalChain::from_blocks(
    [
        (0, Hash::hash("zero".as_bytes())),
        (1, Hash::hash("first".as_bytes())),
        (2, Hash::hash("second".as_bytes())),
        (3, Hash::hash("third".as_bytes())),
        (12, Hash::hash("twelve".as_bytes())),
        (17, Hash::hash("seventeen".as_bytes())),
    ]
    .into_iter()
    .collect::<BTreeMap<u32, BlockHash>>(),
).unwrap();

println!("Local chain checkpoints: \n{:#?}\n", local_chain.tip());
```

Which will print:
```rust
CheckPoint(
    CPInner {
        block: BlockId {
            height: 17,
            hash: 0x8a0ca06e16959dd0d1e814fe3b1b2df6e1e01b7f8a8254d6501f765d7abca794,
        },
        prev: Some(
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
    },
)
```