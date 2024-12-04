//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use anyhow::{anyhow, Error};
use std::str::FromStr;
use web_sys::console;

use bdk_wallet::{
    bip39::Mnemonic,
    bitcoin::{bip32::DerivationPath, key::Secp256k1, AddressType, Network},
    descriptor,
    descriptor::IntoWalletDescriptor,
};
use bdk_wasm::WalletWrapper;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn new_test_wallet() -> Result<WalletWrapper, String> {
    let mnemonic_str = "drip drum plug universe beyond gasp cram action hurt keep awake tortoise luggage return luxury net jar awake mimic hurry critic curtain quiz kit";
    let esplora_url = "https://mutinynet.com/api";

    let mnemonic = Mnemonic::from_str(mnemonic_str).unwrap();
    let (descriptor, change_descriptor) =
        mnemonic_to_descriptor(mnemonic, Network::Signet, AddressType::P2wpkh).expect("descriptor");

    console::log_1(&format!("descriptor: {}", descriptor).into());
    console::log_1(&format!("change_descriptor: {}", change_descriptor).into());

    WalletWrapper::new(
        "signet".into(),
        descriptor,
        change_descriptor,
        esplora_url.to_string(),
    )
}

#[wasm_bindgen_test]
async fn test_wallet() {
    let wallet = new_test_wallet().expect("wallet");
    wallet.sync(5).await.expect("sync");

    let staged = wallet.take_staged().expect("staged");
    console::log_1(&format!("staged: {}", staged).into());

    let staged2 = wallet.take_staged().expect("staged");
    console::log_1(&format!("staged2: {}", staged2).into());

    let merged = wallet.take_merged(staged).expect("merged");
    console::log_1(&format!("merged: {}", merged).into());

    let first_address = wallet.peek_address(0);
    assert_eq!(
        first_address,
        "tb1q8vl3qjdxnm54psxn5vgzdf402ky23r0jjfd8cj".to_string()
    );

    let balance = wallet.balance();
    assert_eq!(balance, 0);

    let new_address = wallet.get_new_address();
    console::log_1(&format!("new_address: {}", new_address).into());
}

pub fn mnemonic_to_descriptor(
    mnemonic: Mnemonic,
    network: Network,
    address_type: AddressType,
) -> Result<(String, String), Error> {
    let mnemonic_with_passphrase = (mnemonic, None);
    let secp = Secp256k1::new();

    let purpose = match address_type {
        AddressType::P2wpkh => "84h",
        AddressType::P2tr => "86h",
        AddressType::P2sh => "49h",
        _ => "44h",
    };

    let coin_type: &str = match network {
        Network::Bitcoin => "0h",
        _ => "1h",
    };

    let external_path = DerivationPath::from_str(&format!("m/{}/{}/0h/0", purpose, coin_type))?;
    let internal_path = DerivationPath::from_str(&format!("m/{}/{}/0h/1", purpose, coin_type))?;

    let (external_descriptor, internal_descriptor) = match address_type {
        AddressType::P2pkh => {
            let ext = descriptor!(pk((mnemonic_with_passphrase.clone(), external_path)))?;
            let int = descriptor!(pk((mnemonic_with_passphrase, internal_path)))?;
            (ext, int)
        }
        AddressType::P2sh => {
            let ext = descriptor!(sh(wpkh((mnemonic_with_passphrase.clone(), external_path))))?;
            let int = descriptor!(sh(wpkh((mnemonic_with_passphrase, internal_path))))?;
            (ext, int)
        }
        AddressType::P2wpkh => {
            let ext = descriptor!(wpkh((mnemonic_with_passphrase.clone(), external_path)))?;
            let int = descriptor!(wpkh((mnemonic_with_passphrase, internal_path)))?;
            (ext, int)
        }
        AddressType::P2tr => {
            let ext = descriptor!(tr((mnemonic_with_passphrase.clone(), external_path)))?;
            let int = descriptor!(tr((mnemonic_with_passphrase, internal_path)))?;
            (ext, int)
        }
        _ => {
            return Err(anyhow!("Unsupported address type"));
        }
    };

    let (external_wallet_descriptor, _) =
        external_descriptor.into_wallet_descriptor(&secp, network)?;
    let (internal_wallet_descriptor, _) =
        internal_descriptor.into_wallet_descriptor(&secp, network)?;

    external_wallet_descriptor.sanity_check()?;
    internal_wallet_descriptor.sanity_check()?;

    Ok((
        external_wallet_descriptor.to_string(),
        internal_wallet_descriptor.to_string(),
    ))
}
