# Creating Keys and Descriptors

BDK is a descriptor-first library. This page explores how to build them and how they interact with other standards like BIP-39 recovery phrases.

!!! danger
    The keys and descriptors used in **The Book of BDK** are for illustration purposes only; **UNDER NO CIRCUMSTANCES** should any of the keys or descriptors containing private data be used for real money. Entropy generation should be carried out in a secure environment using cryptographically secure random number generators ([CSPRNG](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator)).

## Using descriptor templates
BDK offers utility constructs called _descriptor templates_, which allow you to build descriptors for the four most common script types (BIP 44/49/84/86) with minimal effort.

The following will build and print the full string representation of taproot ([BIP-86](https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki)) internal and external descriptors. We print both the public key descriptors (for HD wallet address generation only) and private key descriptors (for full wallet functionality including transaction signing).

```rust
--8<-- "examples/rust/descriptors/src/main.rs:main"
```
