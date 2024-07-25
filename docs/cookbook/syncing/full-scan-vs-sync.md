# Full Scan vs Sync

The BDK libraries rely on two syncing concepts we call _**Full Scan**_ and _**Sync**_. This page explains the difference between the two and recommendations on when to use each.

## Full Scan

A _full scan_ is an operation that aims to build a complete picture of a wallet's UTXOs from scratch, with no prior data or knowledge about the wallet required. It involves querying the blockchain data client for ScriptPubKeys that the wallet owns, typically at a number of indices on two keychains, an external keychain and an internal one (often called a _change_ keychain). Because the wallet doesn't know which scripts/addresses have been given out and potentially have funds associated with them, it needs to query the blockchain client for a group of addresses, see if any of those have balances, and keep asking for more until a number of addresses in a row for each keychain are unused (the exact number is configurable and is what's typically known as a _stop gap_). The operation also returns an new chain tip the wallet uses to update its [`LocalChain`](https://docs.rs/bdk_chain/latest/bdk_chain/local_chain/struct.LocalChain.html).

A full scan is only needed in cases where the wallet is unknown and is not loaded from persistence. In almost all other instances, a sync should be performed.

## Sync

A _sync_ is a related operation which can be thought of as "catching up" on the latest things that have happened on the chain since the last full scan or sync. A sync only makes sense if the list of addresses given out is known, and will query the blockchain data client for those scripts only. It also returns an new chain tip the wallet uses to update its [`LocalChain`](https://docs.rs/bdk_chain/latest/bdk_chain/local_chain/struct.LocalChain.html).

A sync is smaller and more optimal operation than a full scan, and should be used in almost all cases after an initial full scan has been performed.

## Additional Considerations

The following heuristics works well for most standard wallets:

1. If you are recovering a wallet which you expect currently has or has ever had a balance _or_ are creating a wallet from scratch, your first syncing operation should be a _full scan_. This will allow the wallet to discover which scripts have been used and create an accurate overall balance. All other syncing operations should be _sync_.
2. If you are loading a wallet from persistence for which a _full scan_ has already been performed and related data has been persisted, your syncing operations should be _sync_.
3. If you are operating on a wallet which is shared with other entities which might reveal addresses, your local wallet and its `TxGraph` will not know about these revealed scripts. In this case and if you suspect addresses might have been revealed, you should perform a _full scan_.

## Examples

The workflow for a full scans and syncs are 3-step processes:

1. Ask the wallet for the data structure required;
2. Pass it to your blockchain client and request a full scan or sync;
3. The client returns an update, which you then apply to the wallet.

This workflow ensures that the wallet structure is not blocked while the syncing operation is performed.

#### Full Scan With Esplora

```rust
// Create the Esplora client
let client: esplora_client::BlockingClient = Builder::new("http://signet.bitcoindevkit.net").build_blocking();

// Full scan the wallet
let full_scan_request: FullScanRequest<KeychainKind> = wallet.start_full_scan()
let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)?;

// Apply the update to the wallet
wallet.apply_update(update)?;
```

#### Sync With Esplora

```rust
// Create the Esplora client
let client: esplora_client::BlockingClient = Builder::new("http://signet.bitcoindevkit.net").build_blocking();

// Sync the wallet
let sync_request: SyncRequest = wallet.start_sync_with_revealed_spks()
let update: SyncResult = client.sync(sync_request, PARALLEL_REQUESTS)?;

// Apply the update to the wallet
wallet.apply_update(update)?;
```

<br>
