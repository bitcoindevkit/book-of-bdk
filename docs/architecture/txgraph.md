# TxGraph
TxGraph contains transactions and indexes them so you can easily traverse the graph of those transactions.

A `TxGraph` contains four private fields:
```rust
pub struct TxGraph<A = ()> {
    // All transactions that the graph is aware of in format: `(tx_node, tx_anchors, tx_last_seen)`
    txs: HashMap<Txid, (TxNodeInternal, BTreeSet<A>, u64)>,
    spends: BTreeMap<OutPoint, HashSet<Txid>>,
    anchors: BTreeSet<(A, Txid)>,
    // This exists so that `TxGraph::outspends()` can return a reference.
    empty_outspends: HashSet<Txid>,
}
```

You add a transaction to your transaction graph using `graph.insert_tx(tx)`. This operation returns a `bdk_chain::ChangeSet`, a structure containing the changes applied to your TxGraph. This `ChangeSet` is the part you persist. You could then apply this ChangeSet to a fresh, empty TxGraph on a different session.

```rust
let mut graph = TxGraph::<()>::default();
println!("Empty graph: {:#?}\n", graph);

// Empty graph: TxGraph {
//     txs: {},
//     spends: {},
//     anchors: {},
//     empty_outspends: {},
// }
```
