use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::KeychainKind;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use bdk_wallet::keys::bip39::{Language, Mnemonic, WordCount};
use bdk_wallet::keys::{GeneratedKey, GeneratableKey};
use bdk_wallet::miniscript::Tap;

fn main() -> () {
    let mnemonic: GeneratedKey<_, Tap> =
    Mnemonic::generate((WordCount::Words12, Language::English))
        .expect("Failed to generate mnemonic");
    println!("Generated Seed Words:\n{}\nNote: save these to recover your wallet later.\nWarning: this seed generation method is for example purposes only! Seeds used on MainNet should be generated using the most random method possible.\n", mnemonic.to_string());

    let seed = mnemonic.to_seed("");
    let xprv: Xpriv =
        Xpriv::new_master(Network::Signet, &seed).expect("Failed to create master key");
    println!("Generated Master Private Key:\n{}\nWarning: be very careful with seeds and private keys when using MainNet! We are logging these values for convenience only because this is an example on SigNet.\n", xprv);

    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External)
        .build(Network::Signet)
        .expect("Failed to build external descriptor");

    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
        .build(Network::Signet)
        .expect("Failed to build internal descriptor");

    println!(
        "----------------  Descriptor  ------------------------------\n{:?}\n{:?}\n",
        descriptor.to_string_with_secret(&key_map), // privkey
        descriptor.to_string() // pubkey
    );
    println!(
        "----------------  Change Descriptor  -----------------------\n{:?}\n{:?}\n",
        change_descriptor.to_string_with_secret(&change_key_map),
        change_descriptor.to_string()
    );
}
