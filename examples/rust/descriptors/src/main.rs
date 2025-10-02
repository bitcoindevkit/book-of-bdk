use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::secp256k1::rand;
use bdk_wallet::bitcoin::secp256k1::rand::RngCore;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use bdk_wallet::KeychainKind;

fn main() {
    // --8<-- [start:main]
    let mut seed: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let network: Network = Network::Signet;
    let xprv: Xpriv = Xpriv::new_master(network, &seed).unwrap();
    println!("Generated Master Private Key:\n{}\nWarning: be very careful with private keys when using MainNet! We are logging these values for convenience only because this is an example on SigNet.\n", xprv);

    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External)
        .build(Network::Signet)
        .expect("Failed to build external descriptor");

    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
        .build(Network::Signet)
        .expect("Failed to build internal descriptor");

    let descriptor_string_priv = descriptor.to_string_with_secret(&key_map);
    let change_descriptor_string_priv = change_descriptor.to_string_with_secret(&change_key_map);
    // --8<-- [end:main]

    println!(
        "----------------  Descriptors  ------------------------------\n# Private Key (External)\n{:?}\n\n# Private Key (Internal)\n{:?}\n\n# Public Key (External)\n{:?}\n\n# Public Key (Internal)\n{:?}\n",
        descriptor_string_priv,
        change_descriptor_string_priv,
        descriptor.to_string(),
        change_descriptor.to_string()
    );
}
