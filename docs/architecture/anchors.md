# Anchors

There are a few versions of anchors, but they all have in common that they implement the trait `bdk_chain::Anchor`. For users of `bdk::Wallet`, the most important of these anchors is `bdk_chain::ConfirmationTimeHeightAnchor`.

A `TxGraph` is generic in its Anchor type. 

You can think of an anchor as the glue that binds a transaction to a block. If a transaction is anchored in a block that is in the best chain, this transaction is therefore also in the best chain. If that block ever gets reorged, the transaction becomes unconfirmed. A Transaction can have multiple anchors. 

