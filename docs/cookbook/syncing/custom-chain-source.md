# Building a Custom Chain Source

The Book of BDK offers guidance on using built-in chain-source crates like Electrum, Esplora, and Bitcoin Core RPC. However, as the BDK ecosystem grows, many developers find themselves building non-standard chain sources — streaming Electrum, Nostr-based syncs, custom compact block filter clients — which require constructing the `Update` struct manually.

This guide outlines the requirements for building a custom chain source and successfully applying updates to your wallet.

## When you need a custom chain source

You might need to build a custom chain source when:

- You are working with a backend not covered by `bdk_electrum`, `bdk_esplora`, or `bdk_bitcoind_rpc`.
- You need streaming or push-based sync models (e.g., Electrum's `blockchain.scripthash.subscribe`).
- You are integrating with a specialized protocol like Nostr or a custom P2P networking layer.
- You require fine-grained control over how transaction updates and block data are fetched and cached.

## The `Update` struct contract

A custom chain source provides the wallet with an `Update` struct containing discovered blockchain data. For a transaction to be properly recognized and affect the wallet's balance, it must include **temporal context**:

- **Confirmed transactions** need a `ConfirmationBlockTime` anchor linking the transaction to a specific block (height, hash, and timestamp).
- **Unconfirmed transactions** need a `seen_ats` entry recording when the transaction was first observed in the mempool.

The built-in chain-source crates handle this automatically. For example, `bdk_esplora` has an [`insert_anchor_or_seen_at_from_status()`](https://github.com/bitcoindevkit/bdk/blob/master/crates/esplora/src/lib.rs) helper that inserts either an anchor or a `seen_at` for every transaction — this is the canonical pattern to follow.

> **⚠️ Temporal context and balance calculation**
>
> At the `Wallet` level, `apply_update()` provides a safety net: it calls
> `TxGraph::apply_update_at(update, Some(now))`, which automatically injects
> `SystemTime::now()` as the `seen_at` value for any transaction missing
> temporal context. This means the balance will still reflect the transaction.
>
> However, if you work directly with `TxGraph` and call
> `apply_update_at(update, None)`, transactions without an anchor or `seen_at`
> entry will be stored in the graph but **silently excluded from balance
> calculations** by the canonicalization algorithm. The call returns
> successfully with no error or warning.
>
> Even when using `Wallet::apply_update()`, you should still provide accurate
> temporal context. Relying on the automatic `SystemTime::now()` injection
> means you lose the actual time the transaction was first observed by your
> chain source, which matters for correct ordering of conflicting transactions.

## Common pitfalls

### 1. Transactions without temporal context

This is the most common issue when building a custom chain source. A transaction added to a `TxUpdate` without an anchor or `seen_at` entry will not contribute to the wallet's balance when applied directly to a `TxGraph`.

This was independently flagged by the [Wizardsardine BDK Audit (Q4 2024)](https://bitcoindevkit.org/audits/2024_q4/bdk_audit_report/), which noted that documentation should mention the `TxUpdate` temporal context requirements.

**Imprecise — relies on Wallet's automatic injection:**
```rust
let mut tx_update = TxUpdate::default();
tx_update.txs.push(tx.into());
// No anchor, no seen_at — Wallet::apply_update() will inject SystemTime::now(),
// but you lose the actual observation time from your chain source.
```

**Precise — explicit temporal context:**
```rust
let mut tx_update = TxUpdate::default();
tx_update.txs.push(tx.into());

if confirmed {
    // For confirmed transactions: provide block anchor
    let anchor = ConfirmationBlockTime {
        block_id: BlockId { height: block_height, hash: block_hash },
        confirmation_time: block_timestamp,
    };
    tx_update.anchors.insert((anchor, txid));
} else {
    // For unconfirmed transactions: provide observation time
    tx_update.seen_ats.insert((txid, observed_timestamp));
}
```

### 2. Lookahead mismatch with external trackers

When an external script pubkey (SPK) tracker uses a larger lookahead than the wallet's internal setting, transactions at higher derivation indices are discovered but the wallet won't recognize the outputs as its own.

**Solution:** Set the wallet's lookahead to match your external tracker when creating the wallet:

```rust
Wallet::create(external_desc, internal_desc)
    .network(network)
    .lookahead(50) // Match your external tracker's lookahead
    .create_wallet(&mut db)?;
```

For a wallet loaded from persistence that was created with a smaller lookahead, call `reveal_addresses_to()` after loading:

```rust
wallet.reveal_addresses_to(KeychainKind::External, 50);
wallet.reveal_addresses_to(KeychainKind::Internal, 50);
```

## Minimal working example

Below is a simplified example showing how to construct an `Update` with proper temporal context for a confirmed transaction.

```rust
use bdk_wallet::bitcoin::{self, Amount, Network, Transaction, TxOut};
use bdk_wallet::chain::{BlockId, ConfirmationBlockTime};
use bdk_wallet::{KeychainKind, Update, Wallet};
use std::collections::BTreeMap;

// 1. Create wallet with both descriptors
let mut wallet = Wallet::create(external_desc, internal_desc)
    .network(Network::Signet)
    .create_wallet_no_persist()?;

// 2. Get an address to pay to
let addr = wallet.reveal_next_address(KeychainKind::External).address;

// 3. Build the TxUpdate with temporal context
let mut tx_update = bdk_core::TxUpdate::default();
tx_update.txs.push(tx.into());

// For a confirmed transaction, add a block anchor:
let anchor = ConfirmationBlockTime {
    block_id: BlockId {
        height: 800_000,
        hash: block_hash, // actual block hash from your chain source
    },
    confirmation_time: 1_690_000_000,
};
tx_update.anchors.insert((anchor, txid));

// For an unconfirmed transaction, use seen_ats instead:
// tx_update.seen_ats.insert((txid, observed_timestamp));

// 4. Apply the update
let update = Update {
    last_active_indices: BTreeMap::new(),
    tx_update,
    chain: None,
};
wallet.apply_update(update)?;

// 5. Verify the balance
println!("Balance: {}", wallet.balance().total());
```

## Testing your chain source

To verify your custom chain source produces correct balances, compare its output against a known-good chain source:

1. Initialize two identical wallets with the same descriptors and network.
2. Sync Wallet A using your custom chain source.
3. Sync Wallet B using `bdk_electrum` or `bdk_esplora`.
4. Assert that `wallet_a.balance() == wallet_b.balance()`.
5. Compare the transaction graph state, ensuring anchors and `seen_ats` entries are consistent.

This dual-wallet comparison approach is especially useful during development: any discrepancy between your chain source and a known-good one immediately reveals missing anchors, missing `seen_ats` entries, or lookahead mismatches.