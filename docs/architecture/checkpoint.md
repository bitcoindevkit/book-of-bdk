# CheckPoints

A `CheckPoint` is a type that contains a block height and hash, and potentially a pointer to a previous `CheckPoint`. For example, printing the CheckPoint for the genesis block of testnet, you get this:
```shell
CheckPoint(CPInner { block: BlockId { height: 0, hash: 0x000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943 }, prev: None })
```
