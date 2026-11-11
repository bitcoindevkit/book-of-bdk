# Sync a wallet with Kyoto

[BIP157](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki) and [BIP158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki) define a protocol for light clients to sync with the Bitcoin network without downloading the entire set of blocks in the chain of most work. These proposals define _compact block filters_, which allow a client to download a small commitment
for the scripts contained in each block. These commitments, or filters, may be checked for inclusion of scripts owned by a user. In the event of a match, the light client may download and verify a block indeed contains a relevant transaction. Syncing via compact block filters offers privacy advantages over other chain sources, as the nodes serving the blocks to the client are only aware that the client is interested in an entire block, which may contain thousands of transactions.

One such implementation of this protocol is [Kyoto](https://github.com/2140-dev/kyoto), which is a node and client for compact block filter based syncing. The [`bdk_kyoto`](https://github.com/bitcoindevkit/bdk-kyoto) crate supports an integration between Kyoto and `bdk_wallet`, so developers using `bdk_wallet` have a simple option to provide privacy-preserving and memory-conservative wallet syncing for their users.

The following example uses the `bdk_kyoto` crate to recover and update a `bdk_wallet` using compact block filters.

### Add required bdk dependencies to your `Cargo.toml` file

```toml title="Cargo.toml"
--8<-- "examples/rust/syncing/kyoto/Cargo.toml"
```

### Create and sync the wallet

```rust title="main.rs"
--8<-- "examples/rust/syncing/kyoto/src/main.rs"
```

### A note on unconfirmed transactions, recoveries, sync and full-scan

The entire set of scripts is checked against each block filter as new blocks are gossiped to the Kyoto node. Because the scripts are not checked iteratively, there is not a semantic difference between "sync" and "full scan". Rather, Kyoto checks filters against the scripts revealed by each keychain in the wallet, plus the `lookahead` number of scripts beyond the last revealed index. How the sync starts is chosen with a `SyncConfig`: `new_wallet_sync` for a wallet that has never been used, `sync_from_last_checkpoint` for a wallet that has synced before, and `wallet_recovery_sync` to recover a wallet from a checkpoint in the chain. During a recovery, each block that pays one of the wallet's scripts reveals that script's index, moving the lookahead forward. The `lookahead` therefore acts as the gap limit of the recovery, and should be set larger for wallets that may have handed out many addresses before receiving payments to them.

Unconfirmed transactions pose a problem for light clients, as connections are untrusted. As such, users will be unaware of transactions they have received until they are confirmed in a block. While this tradeoff may be cumbersome, the benefits may outweigh the costs for privacy-conscious users.