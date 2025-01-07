use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::keys::bip39::Mnemonic;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use bdk_wallet::KeychainKind;

const RECOVERY_PHRASE: &str = "[your 12 word seed phrase here ...]";
// const RECOVERY_PHRASE: &str = "holiday marble tide globe license stumble rescue antenna monitor sea half sauce"; // example

fn main() -> () {
    let mnemonic = Mnemonic::parse(RECOVERY_PHRASE).expect("Invalid seed! Be sure to replace the value of RECOVERY_PHRASE with your own 12 word seed phrase.");
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
        "----------------  Descriptors  ------------------------------\nPrivate Key, External:\n{:?}\nPrivate Key, Internal:\n{:?}\nPublic Key, External:\n{:?}\nPublic Key, Internal:\n{:?}\n",
        descriptor.to_string_with_secret(&key_map), // privkey
        change_descriptor.to_string_with_secret(&change_key_map),
        descriptor.to_string(), // pubkey
        change_descriptor.to_string()
    );
}
