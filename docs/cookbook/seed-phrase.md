# From Seed Phrase to Descriptors

BDK wallets require the use of descriptors, but recovery phrases (also called seed phrases) are a common and popular backup solution. Creating descriptors from a recovery phrase is a workflow requried widely and BDK makes this easy with its _descriptor templates_, which are offered for common descriptors ([BIP 44/49/84/86](https://docs.rs/bdk_wallet/latest/bdk_wallet/descriptor/template/index.html)).

!!! note "Feature Flags"

    The current example requires the feature `keys-bip39` for `bdk_wallet`.
    You can add it with:
    
    ```bash
    cargo add bdk_wallet --features="keys-bip39"
    ```

### Example

```rust
use bdk_wallet::bitcoin::Network;
use bdk_wallet::keys::{bip39::Mnemonic, DerivableKey, ExtendedKey};
use bdk_wallet::template::Bip84;
use bdk_wallet::KeychainKind;

const RECOVERY_PHRASE: &str = "one answer find clarify hire van aspect crystal brisk shoot rain permit";

pub fn main() -> () {
    let words = Mnemonic::parse(RECOVERY_PHRASE).expect("Invalid seed");
    
    // Create extended key to generate descriptors
    let xkey: ExtendedKey = words.into_extended_key().expect("keys-bip39 feature required");
    let xprv = xkey.into_xprv(Network::Testnet).expect("private key missing from ExtendedKey");

    // Build descriptors from xprv, choosing from any of the templates
    let descriptor = Bip84(xprv.clone(), KeychainKind::External);
    let change_descriptor = Bip84(xprv, KeychainKind::Internal);

    // Use the descriptors to build a wallet
    // let wallet = Wallet::new(descriptor, change_descriptor, Network::Testnet).unwrap();
}
```
