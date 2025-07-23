# Preface

In my conversations with wallet developers, I've seen some common themes come up when it comes to compact block filters. Generally developers can quantify some of the _properties_ of syncing wallets with compact block filters, but oftentimes the implementation details are fuzzy. With this blog, I am going to dive into the details, and share what to expect if you are considering block filters. Not all platforms, wallets and users would benefit from block filters, but my hope is to convince you, the developer, that both simple and private wallet syncing is possible for your app!

# The Protocol

It's best to start with what it means to be a "compact block filter client". A compact block filter is a concise representation of what bitcoin scripts are in a block. These filters can be queried for a match against scripts that the user owns. Oftentimes these filters are only a few hundred bytes. A client is simply a program that downloads these filters for a particular range of blocks, queries them, and downloads the block in the case of a match.

You may have a couple questions now:
1. Who is serving the filters?
2. How does the client know what filters to request?

The answer to 1. is relatively simple. Any computer running a full-archival bitcoin node, like Bitcoin Core, can serve filters by selecting the configurations. That's it! The answer to 2. is a bit more interesting. The client must download the block headers from peers they connect to and decide what the chain of most proof of work is. Once the client has decided on a chain, the filters may be downloaded and queried.

!!! notice
    Some steps are omitted for simplicity, however the primary actions for the client are: select a chain, download filters, download blocks

This implies a few things. First, the client must communicate using the bitcoin peer-to-peer protocol, which is facilitated directly using TCP connections. This can be an advantage to your apps, as no HTTPS dependencies are required, reducing the binary size and potential for CVE vulnerabilities. Next, the client must store some information about chain, albeit not that much data. Third, the client must maintain a list and find peers to connect to, which we will dive into later.

# Why do all of this?

A primary advantage of using block filters is user privacy. Requesting the block filters does not reveal any sensitive information, and bitcoin blocks are often full of many transactions. The client may also connect to multiple nodes and randomly select who to request blocks from. For users that have low resource devices, but would like strong privacy guarantees, block filters are the perfect option.

The reliability of block filters is also dependent on the properties of the bitcoin network. If a reasonable amount of nodes are discover-able and serving filters, then the user experience is seamless and reliable. In contrast, using server-based protocols may incur rate-limiting and downtime. For users broadcasting transactions, this also implies they should never have to broadcast transactions to the same node twice, another great privacy win.

On the experimental side, the block filter client is a "pseudo-node" itself. New types of wallets may run the client 24/7, and constantly sync the user's wallet with the chain. From the perspective of the user, their sync speed would be as fast as it takes to log-in to their dedicated server! My hope is block filters will invigorate self-hosting businesses and products for users to run their own infrastructure.

# The Numbers

We've reviewed some of the properties, let's look at the metrics. We will begin with some on-device data, and transition into network-wide statistics.

### Storage

The impact on storage is minimal. At the time of writing, for the client to follow the chain of most work, roughly 0.072 gigabytes is required to store _at a maximum_. Most users will be using SegWit or Taproot, so this footprint should be more like 0.030 gigabytes. The book of addresses a client must maintain is configurable, but a reasonable list is only around 0.50 - 1.0 **megabytes**!

### Memory

The use of memory is also low. For syncing a month or two of data, the runtime overhead is around 30 to 40 megabytes, as measured on an iPhone. For recovering wallets, this may reach 70 to 80 megabytes, but this still doesn't come close to an application like YouTube.

### Bandwidth

Bandwidth will fluctuate depending on how many transactions a user expects to send and receive. For a few months of data with a handful of transactions, the network usage should sit around 100 megabytes. However this may change for wallet recoveries. Users can expect to download up to a gigabyte or more for old wallets, so mobile users should be plugged into power and on WiFi for this step. After the initial recovery, users can sync while on the go! Usual syncs should be possible on cellular data in most regions.

### Energy and CPU Impact

There are many hashing operations performed when syncing compact block filters, so the CPU usage tends to be higher than most consumer applications. While this is not a problem for short a duration, users that have not synced their wallet in 6 months or more should likely connect their phone to power. CPU usage may be cleverly spread out as well. If the underlying operating system allows it, scheduling a background sync with the OS when the user is likely to be connected to power will make the user experiences seamless, as their phone has stayed up-to-date with the chain.

### Time

Sync times are highly variable, but on WiFi and with a fast per, a year of data can be synced in only a minute or two. Months of data may take 10-30 seconds, and a week should only take a moment. When network conditions are spotty, the numbers are harder to quantify. This is yet another reason to consider scheduling background tasks, as a daily sync should only take a second!

### Distribution of Peers

Developer Nick Johnson [ran a census](https://census.yonson.dev/) of what services bitcoin nodes are offering. Many nodes were polled - over 300,000 - however only around 11,000 were reachable. Of these, around 1,000 maintain a compact block filters index.

On the one hand, having 1,000 potential peers is low considering how many bitcoin nodes exist. Yet that is also great from the client point of view, as the client essentially has 1,000 potential servers. The only problem is the client must poll these peers to find useful ones.

A mitigation to polling a bunch of peers is using DNS seeders. These seeders are constantly crawling the bitcoin network and finding good peers, but relying solely on DNS implies a trust in the seeder. This is tricky for wallet developers and users, as they may want a fast syncing experience, but do not want to fully trust the seeder. A great option is to pick some peers from DNS for a sync, and others from the local address book.

# Challenges

Now we turn to the design space to improve the user experience of light clients. I hope by now you are considering a block filter integration for your users, so here are the last couple caveats to consider.

### Unconfirmed Transactions

Many users come to expect an instant feedback when a transaction enters the server's memory pool in which they receive money. This is both a philosophical and technical problem, as current light clients cannot maintain a large memory pool, may not be online at the time of transaction gossip, and cannot validate transactions. Furthermore, even validated transactions may be changed via RBF, and the user may not receive the bitcoin they expect. If your users are advanced in their bitcoin knowledge, they should have at least a vague idea of why they will need to wait for a block.

For users that are moving their bitcoin off the exchange for the first time, this wait-time may be confusing or scary. My first suggestion is still to wait for a block confirmation, but to inform the user they should check back in 10 minutes if they expect to receive a payment. Even if the exchange shows the transaction as sent, it should still remain "pending", so informative user interfaces are a potential solution to this problem.

If that is still not enough, one could connect over the peer-to-peer network and monitor transaction gossip. I advice against this approach on mobile, as there is a non-zero, even high, probability the gossip messages are missed by the time the user opens their app. However, if your app is a self-hosted "always on" program, monitoring peer to peer gossip is far more acceptable, as gossip is being monitored even if the user is not logged in.

### Fee Estimation

Due to the construction of a bitcoin block, the amounts for each input cannot be known with simply the block data itself. One would need the transaction outputs corresponding to the outpoints contained in the block. This data is sometimes referred to as "undo data". Unfortunately, this means the light client cannot estimate fee rates for individual transactions in recent blocks.

Servers are available to estimate fees, but this re-introduces HTTPs dependencies in the stack. If your use cases involves a small binary and strict dependency graph, there is still a solution to give fee rate approximation. The average fee rate paid in a block may be computed by taking `(Coinbase output - Block subidy) / Block Weight`. Averages are more effected by outliers, so exceptionally low or high fee rates on individual transactions may skew this number. Yet, it may be good enough for your users. 

This is an open area of research, and may be further improved with light weight machine learning models. However, if HTTPs is used in your app already, a server is recommended to fetch fees. As a final note, there are a mounting number of use cases for sending the "undo data" over the peer-to-peer network. If this feature is implemented, many aspects of the light client experience may be improved, including fee rate estimation.

