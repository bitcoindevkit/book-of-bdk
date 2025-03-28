# Full Scan vs Sync

The BDK libraries rely on two syncing concepts we call _**Full Scan**_ and _**Sync**_. This page explains the difference between the two and recommendations on when to use each.

## Full Scan

A _full scan_ is an operation that aims to build a complete picture of a wallet's UTXOs from scratch, with no prior data or knowledge about the wallet required. It involves querying the blockchain data client for ScriptPubKeys that the wallet owns, typically at a number of indices on two keychains, an external keychain and an internal one (often called a _change_ keychain). Because the wallet doesn't know which scripts/addresses have been given out and potentially have funds associated with them, it needs to query the blockchain client for a group of addresses, see if any of those have balances, and keep asking for more until a number of addresses in a row for each keychain are unused (the exact number is configurable and is what's typically known as a _stop gap_). The operation also returns a new chain tip the wallet uses to update its [`LocalChain`](https://docs.rs/bdk_chain/latest/bdk_chain/local_chain/struct.LocalChain.html).

A full scan is only needed in cases where the wallet is unknown and is not loaded from persistence. In almost all other instances, a sync should be performed.

## Sync

A _sync_ is a related operation which can be thought of as "catching up" on the latest things that have happened on the chain since the last full scan or sync. A sync only makes sense if the list of addresses given out is known, and will query the blockchain data client for those scripts only. It also returns a new chain tip the wallet uses to update its [`LocalChain`](https://docs.rs/bdk_chain/latest/bdk_chain/local_chain/struct.LocalChain.html).

A sync is a smaller and more optimal operation than a full scan, and should be used in almost all cases after an initial full scan has been performed.

## Additional Considerations

The following heuristics work well for most standard wallets:

1. If you are recovering a wallet which currently holds or has ever held a balance _or_ are creating a wallet from scratch, your first syncing operation should be a _full scan_. This will allow the wallet to discover which scripts have been used and create an accurate overall balance. All other syncing operations should be _sync_.
2. If you are loading a wallet from persistence for which a _full scan_ has already been performed and related data has been persisted, your syncing operations should be _sync_.
3. If you are operating on a wallet which is shared with other entities which might reveal addresses, your local wallet and its `TxGraph` will not know about these revealed scripts. In this case, and if you suspect addresses might have been revealed, you should perform a _full scan_.

## Additional Considerations — Really Big Wallets

On really big wallets, a custom, application-defined choice of which spks to sync at any given time/trigger is probably more appropriate.

For example, if a wallet has 2500 addresss revealed and your application tries to stay on a 10s loop... it is not a good idea (and doesn't really make sense anyway) to try and sync it all on every iteration.

Some pooling of the spks in different buckets would probably work best, and this would be defined at the application layer. Variables to include in the choice of which SPKs to sync would probably include last known transaction for the SPK and last time it was synced.

<br>
