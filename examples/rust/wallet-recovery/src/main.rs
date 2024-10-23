use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::KeychainKind;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use bdk_wallet::keys::bip39::Mnemonic;
use bdk_wallet::AddressInfo;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult};

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;


// const RECOVERY_PHRASE: &str = "[your 12 word seed phrase here ...]";
// const RECOVERY_PHRASE: &str = "one answer find clarify hire van aspect crystal brisk shoot rain permit"; // example
const RECOVERY_PHRASE: &str = "holiday marble tide globe license stumble rescue antenna monitor sea half sauce"; // example

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
        "----------------  Descriptor  ------------------------------\n{:?}\n{:?}\n",
        descriptor.to_string_with_secret(&key_map), // privkey
        descriptor.to_string() // pubkey
    );
    println!(
        "----------------  Change Descriptor  -----------------------\n{:?}\n{:?}\n",
        change_descriptor.to_string_with_secret(&change_key_map),
        change_descriptor.to_string()
    );

    // TODO: Create wallet and persist
    
    // Create the wallet
    let mut wallet: Wallet = Wallet::create(descriptor, change_descriptor)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    // Sync the wallet
    // --8<-- [start:client]
    let client: esplora_client::BlockingClient = Builder::new("https://mutinynet.com/api").build_blocking();
    // --8<-- [end:client]
    
    println!("Syncing wallet...");
    // --8<-- [start:scan]
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();

    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
    // --8<-- [end:scan]

    // Reveal a new address from your external keychain
    // doing this just to show it is an HD wallet 
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);
}
