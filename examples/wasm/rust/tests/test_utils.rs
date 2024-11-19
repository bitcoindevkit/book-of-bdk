use std::str::FromStr;

use anyhow::{anyhow, Error};
use bdk_wallet::{
    bip39::Mnemonic,
    bitcoin::{bip32::DerivationPath, key::Secp256k1, AddressType, Network},
    descriptor,
    descriptor::IntoWalletDescriptor,
};

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
        _ => "44h",
    };

    let coin_type = match network {
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
