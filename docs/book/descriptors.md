# Creating Descriptors

## Using descriptor templates
BDK offers utility constructs called "descriptor templates", which allow you to build descriptors for the 4 most common script types (BIP 44/49/84/86) with minimal effort.

The following will build and print the full strings of taproot (BIP-86) internal and external descriptors.

```rust
use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::bitcoin::secp256k1::rand;
use bdk::bitcoin::secp256k1::rand::RngCore;
use bdk::KeychainKind;
use bdk::template::{Bip86, DescriptorTemplate};

fn main() -> () {
    let mut seed: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let network: Network = Network::Testnet;
    let xprv: ExtendedPrivKey = ExtendedPrivKey::new_master(network, &seed).unwrap();
    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External).build(network).unwrap();
    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::External).build(network).unwrap();

    println!(
        "Descriptor: {:?}",
        descriptor.to_string_with_secret(&key_map)
    );
    println!(
        "Change descriptor: {:?}",
        change_descriptor.to_string_with_secret(&change_key_map)
    );
}
```
