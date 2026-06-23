// Detailed documentation for this code can be found at https://bitcoindevkit.github.io/book-of-bdk/cookbook/starter-example/

use bdk_esplora::esplora_client;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::EsploraExt;
use bdk_wallet::bitcoin::{Address, Amount, FeeRate, Network, Psbt};
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResponse};
use bdk_wallet::miniscript::Descriptor;
use bdk_wallet::rusqlite::Connection;
use bdk_wallet::signer::SignersContainer;
use bdk_wallet::{AddressInfo, KeychainKind};
use bdk_wallet::{SignOptions, Wallet};
use std::process::exit;
use std::str::FromStr;

const STOP_GAP: usize = 20;
const PARALLEL_REQUESTS: usize = 1;
const DB_PATH: &str = "starter.sqlite3";

fn main() {
    println!("\nWelcome to the Book of BDK Starter Example Wallet!");
    // --8<-- [start:descriptors]
    // Using the "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome" mnemonic and BIP-86
    let descriptor: &str = "tr([5bc5d243/86'/1'/0']tpubDC72NVP1RK5qwy2QdEfWphDsUBAfBu7oiV6jEFooHP8tGQGFVUeFxhgZxuk1j6EQRJ1YsS3th2RyDgReRqCL4zqp4jtuV2z7gbiqDH2iyUS/0/*)#xh44xwsp";
    let change_descriptor: &str = "tr([5bc5d243/86'/1'/0']tpubDC72NVP1RK5qwy2QdEfWphDsUBAfBu7oiV6jEFooHP8tGQGFVUeFxhgZxuk1j6EQRJ1YsS3th2RyDgReRqCL4zqp4jtuV2z7gbiqDH2iyUS/1/*)#hrs5mmqe";
    // --8<-- [end:descriptors]

    // --8<-- [start:wallet]
    // Initiate the connection to the database
    let mut conn = Connection::open(DB_PATH).expect("Can't open database");

    // Create the wallet
    let wallet_opt = Wallet::load()
        .descriptor(KeychainKind::External, Some(descriptor))
        .descriptor(KeychainKind::Internal, Some(change_descriptor))
        .check_network(Network::Regtest)
        .load_wallet(&mut conn)
        .unwrap();

    let mut wallet = if let Some(loaded_wallet) = wallet_opt {
        loaded_wallet
    } else {
        Wallet::create(descriptor, change_descriptor)
            .network(Network::Regtest)
            .create_wallet(&mut conn)
            .unwrap()
    };
    // --8<-- [end:wallet]

    // --8<-- [start:client]
    // Sync the wallet
    let client: esplora_client::BlockingClient =
        Builder::new("http://127.0.0.1:3002").build_blocking();

    println!("Syncing wallet...");
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResponse<KeychainKind> = client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .unwrap();

    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();

    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
    // --8<-- [end:client]

    // --8<-- [start:address]
    if balance.total().to_sat() < 50000 {
        println!("Your wallet does not have sufficient balance for the following steps!");
        // Reveal a new address from your external keychain
        let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
        println!(
            "Send Regtest coins to {} (index {})",
            address.address, address.index
        );
        wallet.persist(&mut conn).expect("Cannot persist");
        exit(0)
    }
    // --8<-- [end:address]

    // --8<-- [start:recipient]
    // Use a faucet return address
    let faucet_address = Address::from_str("bcrt1qxzh0r7mlztv3m8vxet5xxnsy9zh7j5tshh6vhp")
        .unwrap()
        .require_network(Network::Regtest)
        .unwrap();

    let send_amount: Amount = Amount::from_sat(12345);
    // --8<-- [end:recipient]

    // --8<-- [start:transaction]
    let mut builder = wallet.build_tx();
    builder
        .fee_rate(FeeRate::from_sat_per_vb(4).unwrap())
        .add_recipient(faucet_address.script_pubkey(), send_amount);

    let mut psbt: Psbt = builder.finish().unwrap();
    // --8<-- [end:transaction]

    // --8<-- [start:sign]
    // Using the "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome" mnemonic and BIP-86
    let private_descriptor: &str = "tr(tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/86'/1'/0'/0/*)#x627tk5a";
    let private_change_descriptor: &str = "tr(tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/86'/1'/0'/1/*)#hw0lkry9";

    let (_, external_keymap) =
        Descriptor::parse_descriptor(wallet.secp_ctx(), private_descriptor).unwrap();
    let (_, internal_keymap) =
        Descriptor::parse_descriptor(wallet.secp_ctx(), private_change_descriptor).unwrap();
    let secp = bdk_wallet::bitcoin::secp256k1::Secp256k1::new();

    let external_signers_container = SignersContainer::build(
        external_keymap,
        wallet.public_descriptor(KeychainKind::External),
        &secp,
    );
    let internal_signers_container = SignersContainer::build(
        internal_keymap,
        wallet.public_descriptor(KeychainKind::Internal),
        &secp,
    );

    let signers: &[&SignersContainer; 2] =
        &[&external_signers_container, &internal_signers_container];

    let psbt_was_signed_and_finalized: bool = wallet
        .sign_with_signers(&mut psbt, signers, SignOptions::default())
        .unwrap();
    assert!(psbt_was_signed_and_finalized);

    let tx = psbt.extract_tx().unwrap();
    client.broadcast(&tx).unwrap();
    println!("Transaction broadcast! Txid: {}", tx.compute_txid());
    // --8<-- [end:sign]
}
