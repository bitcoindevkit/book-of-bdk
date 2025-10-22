use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::keys::bip39::Mnemonic;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use bdk_wallet::KeychainKind;

fn main() {
    println!("\n-------- Book of BDK Recovery Phrases Example ------------------------");

    // --8<-- [start:main]
    let recovery_phrase: &str = "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome";
    let mnemonic = Mnemonic::parse(recovery_phrase).expect("Invalid seed! Be sure to replace the value of RECOVERY_PHRASE with your own 12 word seed phrase.");
    let seed = mnemonic.to_seed("");
    let xprv = Xpriv::new_master(Network::Signet, &seed).expect("Failed to create master key");

    println!("# Master Private Key\n{xprv}\nWarning: be very careful with seeds and private keys when using MainNet! We are logging these values for convenience and demonstration purposes only.\n");

    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External)
        .build(Network::Signet)
        .expect("Failed to build external descriptor");

    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
        .build(Network::Signet)
        .expect("Failed to build internal descriptor");
    // --8<-- [end:main]

    println!(
        "Private Key (External)\n{:?}\n\nPrivate Key (Internal)\n{:?}\n\nPublic Key (External)\n{:?}\n\nPublic Key (Internal)\n{:?}\n",
        descriptor.to_string_with_secret(&key_map),
        change_descriptor.to_string_with_secret(&change_key_map),
        descriptor.to_string(),
        change_descriptor.to_string()
    );
}
