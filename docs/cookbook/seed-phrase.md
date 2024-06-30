# Starting from a Seed Phrase

In order to generate a wallet from a seed phrase, it is necessary to generate descriptors from the seed phrase first. The extended key can be generated from the seed phrase and used with a descriptor template to build a wallet.

!!! note "Feature Flags"

    The current example requires the feature `keys-bip39` for `bdk_wallet`.
    You can add with:
    
    ```bash
    cargo add bdk_wallet --features="keys-bip39"
    ```

## Descriptor Templates

BDK offers templates for common descriptors: [BIP 44/49/84/86](https://docs.rs/bdk_wallet/latest/bdk_wallet/descriptor/template/index.html).

```rust

use bdk_wallet::{
    bitcoin::Network,
    keys::{bip39::Mnemonic, DerivableKey, ExtendedKey},
    template::Bip84,
    wallet::Wallet,
    KeychainKind,
};

pub fn from_seed_phrase() -> Wallet {
    let words =
        Mnemonic::parse("one answer find clarify hire van aspect crystal brisk shoot rain permit")
            .expect("Invalid seed");
    // create extended key to generate descriptors
    let xkey: ExtendedKey = words.into_extended_key().expect("keys-bip39 feature required");
    let xprv = xkey.into_xprv(Network::Testnet).expect("private key missing from ExtendedKey");

    // build descriptors from xprv, choosing from any of the templates
    let descriptor = Bip84(xprv.clone(), KeychainKind::External);
    let change_descriptor = Bip84(xprv, KeychainKind::Internal);

    // finish building new wallet
    let wallet = Wallet::new(descriptor, change_descriptor, Network::Testnet).unwrap();
    wallet
}

```
