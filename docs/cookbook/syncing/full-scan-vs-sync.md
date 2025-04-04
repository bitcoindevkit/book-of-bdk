# Full Scan vs Sync

Two of the four blockchain client libraries maintained by BDK (Electrum and Esplora) use what we refer to as _SPK-based syncing_ (as opposed to _block-by-block_ syncing). This SPK-based syncing relies on two concepts we call _**Full Scan**_ and _**Sync**_. This page explains the difference between the two, the tradeoffs made when choosing each, and recommendations on when to use them.

## Full Scan

A _full scan_ is an operation that aims to build a complete picture of a wallet's UTXOs from scratch, with no prior data or knowledge about the wallet. It involves querying the blockchain data client for ScriptPubKeys that the wallet owns, typically at a number of indices on the wallet's keychains. 

Because the wallet doesn't know which scripts/addresses have been given out and potentially have funds associated with them, it needs to query the blockchain client for a group of addresses, see if any of those have balances, and keep asking for more until a number of addresses in a row for each keychain are unused (the exact number is configurable and is what's typically known as a _stop gap_). The operation also returns a new chain tip the wallet uses to update its [`LocalChain`](https://docs.rs/bdk_chain/latest/bdk_chain/local_chain/struct.LocalChain.html).

A full scan is needed in cases where the wallet is unknown and is not loaded from persistence, or in cases where other software wallets might have revealed/used addresses on the keychains since the last time the wallet was used (like how some users might have wallets they use on both mobile apps and desktop applications, and the mobile wallet has no way of knowing if the desktop wallet was used since the last time it was synced up).

## Sync

A _sync_ is a related operation which can be thought of as "catching up" on the latest things that have happened on the chain since the last full scan or sync. A sync only makes sense if the list of addresses given out is known, and will query the blockchain data client for those scripts only. It also returns a new chain tip the wallet uses to update its [`LocalChain`](https://docs.rs/bdk_chain/latest/bdk_chain/local_chain/struct.LocalChain.html).

A sync is a smaller and more optimal operation than a full scan, and should often be used in cases after an initial full scan has been performed.

## Practical Examples

Here are 4 different examples with wallets of different sizes and stop gap choices. The numbers in the Full Scan and Sync columns are the resulting number of scripts requested to the Electrum or Esplora instances.

| Example | Addr. Revealed | Stop Gap | Full Scan | Sync |
| :-----: | :------------: | :------: | :-------: | :--: |
| **A**   | 7              | 20       | 27        | 7    |
| **B**   | 7              | 1000     | 1007      | 7    |
| **C**   | 500            | 20       | 520       | 500  |
| **D**   | 500            | 1000     | 1500      | 500  |

## Additional Considerations — Server Load

Note that the choice of full scan vs sync as well as the stop gap on the wallet influences the load on the server infrastructure that services those requests. Whether these servers are maintained by you or you are using publicly available servers, understanding those choices is important.

In particular, the difference between a full scan and a sync depends on your choice of stop gap and the current size of the wallet like so:

1. If the stop gap is small, the difference between your sync and full scan will be smaller than if the stop gap is very big. Wallets **A** and **B** above have revealed 7 addresses each, yet wallet **B**'s full scan is more than 100x its sync while wallet **A**'s is only 4x. Using a sync on wallet **B** will make a big difference on your server load (particularly if you sync often).
3. The difference between the full scan and sync diminishes the bigger the wallet becomes. On a very small wallet (wallets **A** and **B** above), the stop gap makes a big part of the number of SPKs synced, whereas for bigger wallets , the stop gap makes up a much smaller percentage of the total number of SPKs synced. For example on wallet **C** (big wallet but small stop gap) the difference between the full scan and the sync is minimal.

## Additional Considerations — Sync, But Not Always!

We can think of a sync as a more narrow but optimized approach to syncing. It works really well for some situations, but is less applicable as a "one-worklflow-to-rule-them-all". In particular, wallets where the user might also be using their descriptors on _other_ software can create problems with the sync workflow, since the wallet is not aware that new addresses have been revealed, and will not include them in its sync request.

Here are situations for which sync works and performs best:

- Cases where you know the user only uses their descriptors with the current wallet (for example if the user has specified this as a toggle in settings), and a full scan has already been performed once.
- Situations where you call your syncing workflow on a loop, and you can safely assume that you're the only user of the descriptors at any given time. For example if you loop on 15s intervals you could have your loop full scan on first iteration and sync on further iterations, with a full scan every X number of minutes/iterations.

Production applications need either a mix of both (and if so the option to trigger a full scan on user request, not just on first startup), or, if they want to only use one, use the full scan (to cover all cases and situations where users use their backups on different wallets/software).

## Additional Considerations — General Heuristics

The following heuristics work well for most standard wallets:

1. If you are recovering a wallet which currently holds or has ever held a balance _or_ are creating a wallet from scratch, your first syncing operation should be a _full scan_. This allows the wallet to discover which scripts have been used and create an accurate overall balance. If your sofware is certain that the users is only using their descriptors with this wallet, all other syncing operations can be _sync_.
2. If you are loading a wallet from persistence for which a _full scan_ has already been performed and related data has been persisted, your syncing operations can be _sync_. If you believe some addresses have been revealed in the meantime, do a full scan on startup.
3. If you are operating on a wallet which is shared with other entities which might reveal addresses, your local wallet and its `TxGraph` will not know about these revealed scripts. In this case, and if you suspect addresses might have been revealed, you should perform a _full scan_.

## Additional Considerations — Really Big Wallets

On really big wallets, a custom, application-defined choice of which spks to sync at any given time/trigger is probably more appropriate.

For example, if a wallet has 2500 addresss revealed and your application tries to stay on a 10s loop... it is not a good idea (and doesn't really make sense anyway) to try and sync it all on every iteration.

Some pooling of the spks in different buckets would probably work best, and this would be defined at the application layer. Variables to include in the choice of which SPKs to sync would probably include last known transaction for the SPK and last time it was synced.

<br>
