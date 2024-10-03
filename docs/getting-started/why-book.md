# Who Is This Book For?

The purpose of this book is to give a strong overview of the Bitcoin Development Kit family of libraries and how they can be used together to build production-grade bitcoin applications. We aim to provide a good understanding of how to leverage our libraries together, expose the options available to developers in terms of blockchain clients and persistence layers, as well as ways they can go deeper into lower-level crates if their needs are not met by the high-level APIs exposed in the `bdk_wallet` library.

Finally, the book is meant to get developers up to speed on general concepts pertaining to the BDK architecture as well as concrete examples of how to use our APIs in the different languages for which we provide language bindings libraries.

**What this book is not:**

- API documentation, nor a comprehensive listing of all APIs available in BDK libraries. We maintain [API docs](./api-documentation.md) on all our libraries for that purpose.
- A place to learn about core bitcoin concepts (PSBTs, UTXOs, Electrum protocol, BIPs, etc.). We provide links to great resources on these topics where appropriate.
- A comprehensive treatment of the tradeoff space developers face when building bitcoin applications (Esplora protocol vs compact block filters, onchain vs layer 2s, secure elements on mobile devices, etc.).
