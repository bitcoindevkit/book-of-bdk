# Chain Oracles

!!!note
    There is currently only one implementation of the `ChainOracle` trait (`bdk_chain::local_chain::LocalChain`), but the Nakamoto crate would potentially provide the second implementation of it.

### Why do we need chain oracles?

Chain oracles can be thought of as these constructs that you can call `ChainOracle.is_block_in_chain(block: BlockId, chain_tip: BlockId)` on. How is that useful?

1. In order to build a balance, a wallet needs to know which transaction outputs it owns.
2. To know this, a wallet needs 2 things: 

    1. transactions with outputs that are controlled by its descriptors
    2. these transactions to actually be included in the best chain (confirmed).

3. To know this (which transaction are confirmed), a wallet keeps track of a `TxGraph` which connects transactions to anchors.
4. For an anchor to be useful, it needs to be part of the best chain (if you anchored your tx to a block that gets reorged, it's not a confirmed transaction anymore).
5. We confirm that anchors are in the LocalChain (i.e. ChainOracle) by querying it through the `LocalChain.is_block_in_chain()` method.
6. **If** the anchor is in the best chain -> your transaction is anchored to this anchor -> your transaction must therefore be in the best chain, -> the outputs in this transaction can be added to your balance.
