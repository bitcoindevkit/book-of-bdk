# Working with Descriptors
BDK is a descriptor-first library. This page explores how to build them and how they interact with other standards like BIP-39 recovery phrases.

!!!danger
    The keys and descriptors used in **The Book of BDK** are for illustration purposes only; **UNDER NO CIRCUMSTANCES** should any of the keys or descriptors containing private data be used for real money. Entropy generation should be carried out in a secure environment using cryptographically secure random number generators ([CSPRNG](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator)).

## Using descriptor templates
BDK offers utility constructs called _descriptor templates_, which allow you to build descriptors for the four most common script types (BIP 44/49/84/86) with minimal effort.

The following will build and print the full string representation of taproot ([BIP-86](https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki)) internal and external descriptors.

```rust
use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::bitcoin::secp256k1::rand;
use bdk_wallet::bitcoin::secp256k1::rand::RngCore;
use bdk_wallet::KeychainKind;
use bdk_wallet::template::{Bip86, DescriptorTemplate};

fn main() -> () {
    let mut seed: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let network: Network = Network::Signet;
    let xprv: Xpriv = Xpriv::new_master(network, &seed).unwrap();
    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External).build(network).unwrap();
    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal).build(network).unwrap();

    println!(
        "Descriptor: {:?}",
        descriptor.to_string_with_secret(&key_map)
    );
    println!(
        "Change Descriptor: {:?}",
        change_descriptor.to_string_with_secret(&change_key_map)
    );
}
```
