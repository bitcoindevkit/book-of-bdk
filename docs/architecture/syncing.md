# Understanding the `bdk_chain` crate
The bdk_chain crate handles everything to do with bdk's internal representation of blockchain state and data. It's main types are `LocalChain` and `TxGraph`.

The goal of this crate is to give wallets the mechanisms needed to:

1. Figure out what data they need to fetch.
2. Process the data in a way that never leads to inconsistent states.
3. Fully index that data and expose it to be consumed without friction.

The design goals for these mechanisms are:

1. Data source agnostic — nothing in `bdk_chain` cares about where you get data from or whether you do it synchronously or asynchronously. If you know a fact about the blockchain, you can just tell `bdk_chain`'s APIs about it, and that information will be integrated if it can be done consistently.
2. Error-free APIs.
3. Data persistence agnostic — `bdk_chain` does not care where you cache on-chain data, what you
   cache or how you fetch it.
