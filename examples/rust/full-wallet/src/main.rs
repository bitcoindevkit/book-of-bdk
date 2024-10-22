// detailed documentation for this code can be found at https://bitcoindevkit.github.io/book-of-bdk/cookbook/quickstart/
// --8<-- [start:file]
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult};
use bdk_wallet::keys::bip39::{Language, Mnemonic, WordCount};
use bdk_wallet::keys::{GeneratedKey, GeneratableKey};
use bdk_wallet::miniscript::Tap;
use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::template::{Bip86, DescriptorTemplate};

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;


fn main() -> () {

    let mnemonic: GeneratedKey<_, Tap> =
    Mnemonic::generate((WordCount::Words12, Language::English))
        .expect("Failed to generate mnemonic");
    println!("generated Seed Words:");
    println!("{}", mnemonic.to_string());
    println!("save these to recover your wallet later");

    let seed = mnemonic.to_seed("");
    let xprv: Xpriv =
        Xpriv::new_master(Network::Signet, &seed).expect("Failed to create master key");
    println!("created Master Private Key:");
    println!("{}", xprv);

    let (descriptor, _key_map, _) = Bip86(xprv, KeychainKind::External)
        .build(Network::Signet)
        .expect("Failed to build external descriptor");
    println!("external descriptor: {}", descriptor);

    let (change_descriptor, _change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
        .build(Network::Signet)
        .expect("Failed to build internal descriptor");
    println!("internal descriptor: {}", change_descriptor);

    // Create the wallet
    let mut wallet: Wallet = Wallet::create(descriptor, change_descriptor)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    // Reveal a new address from your external keychain
        // doing this just to show it is an HD wallet 
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Sync the wallet
    // --8<-- [start:client]
    let client: esplora_client::BlockingClient = Builder::new("http://signet.bitcoindevkit.net").build_blocking();
    // --8<-- [end:client]
    
    // --8<-- [start:scan]
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
    // --8<-- [end:scan]

    // TODO: tx build + broadcast, recovery, storage, etc.
}
// --8<-- [end:file]