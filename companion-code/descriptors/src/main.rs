#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]

use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::bitcoin::secp256k1::rand;
use bdk::bitcoin::secp256k1::rand::RngCore;
use bdk::KeychainKind;
use bdk::template::{Bip86, DescriptorTemplate};

fn main() -> () {
    println!("\n--------------------------------------------------------------------------------");
    println!("Companion code for https://bitcoindevkit.github.io/book-of-bdk/book/descriptors/");
    println!("--------------------------------------------------------------------------------\n");

    let mut seed: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let network: Network = Network::Testnet;
    let xprv: ExtendedPrivKey = ExtendedPrivKey::new_master(network, &seed).unwrap();
    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External).build(network).unwrap();
    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal).build(network).unwrap();

    println!(
        "----------------  Descriptor  ------------------------------\n{:?}\n",
        descriptor.to_string_with_secret(&key_map)
    );
    println!(
        "----------------  Change Descriptor  -----------------------\n{:?}\n",
        change_descriptor.to_string_with_secret(&change_key_map)
    );
}
