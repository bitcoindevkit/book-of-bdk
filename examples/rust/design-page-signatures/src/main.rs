// This code is a companion to the "Signing Transactions Without Wallet" page at
// https://bookofbdk.com/design/signing

#![allow(unused_imports)]

use bdk_esplora::esplora_client;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::EsploraExt;
use bdk_wallet::bitcoin::{Address, Amount, FeeRate, Network, Psbt};
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResponse};
use bdk_wallet::miniscript::descriptor::KeyMapWrapper;
use bdk_wallet::miniscript::psbt::PsbtExt;
use bdk_wallet::miniscript::Descriptor;
use bdk_wallet::signer::SignersContainer;
use bdk_wallet::{AddressInfo, KeychainKind};
use bdk_wallet::{SignOptions, Wallet};
use std::process::exit;
use std::str::FromStr;

const STOP_GAP: usize = 20;
const PARALLEL_REQUESTS: usize = 1;

fn main() {
    // Using the "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome" mnemonic and BIP-86
    let public_descriptor: &str = "tr([5bc5d243/86'/1'/0']tpubDC72NVP1RK5qwy2QdEfWphDsUBAfBu7oiV6jEFooHP8tGQGFVUeFxhgZxuk1j6EQRJ1YsS3th2RyDgReRqCL4zqp4jtuV2z7gbiqDH2iyUS/0/*)#xh44xwsp";
    let public_change_descriptor: &str = "tr([5bc5d243/86'/1'/0']tpubDC72NVP1RK5qwy2QdEfWphDsUBAfBu7oiV6jEFooHP8tGQGFVUeFxhgZxuk1j6EQRJ1YsS3th2RyDgReRqCL4zqp4jtuV2z7gbiqDH2iyUS/1/*)#hrs5mmqe";

    // Create the wallet
    let mut wallet = Wallet::create(public_descriptor, public_change_descriptor)
        .network(Network::Regtest)
        .create_wallet_no_persist()
        .unwrap();

    // Sync the wallet
    let client: esplora_client::BlockingClient =
        Builder::new("http://127.0.0.1:3002").build_blocking();

    print!("\nSyncing wallet...");
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResponse<KeychainKind> = client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .unwrap();

    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();
    println!(" complete.");

    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());

    if balance.total().to_sat() < 50000 {
        println!("Your wallet does not have sufficient balance for the following steps!");
        // Reveal a new address from your external keychain
        let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
        println!(
            "Send Regtest coins to {} (index {})",
            address.address, address.index
        );
        exit(0)
    }

    // Create a recipient
    let recipient_address = Address::from_str("bcrt1qxzh0r7mlztv3m8vxet5xxnsy9zh7j5tshh6vhp")
        .unwrap()
        .require_network(Network::Regtest)
        .unwrap();

    let send_amount: Amount = Amount::from_sat(12345);

    // Create a transaction
    let mut builder = wallet.build_tx();
    builder
        .fee_rate(FeeRate::from_sat_per_vb(4).unwrap())
        .add_recipient(recipient_address.script_pubkey(), send_amount);

    let mut psbt: Psbt = builder.finish().unwrap();

    // Using the "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome" mnemonic and BIP-86
    let private_descriptor: &str = "tr(tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/86'/1'/0'/0/*)#x627tk5a";
    let private_change_descriptor: &str = "tr(tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/86'/1'/0'/1/*)#hw0lkry9";
    let secp_ctx = wallet.secp_ctx();

    // —————————————————————————————————————————————————————————————————————————————————————————————
    // APPROACH 1: Psbt::sign
    // —————————————————————————————————————————————————————————————————————————————————————————————
    println!("\nSinging using Psbt::sign");
    let (_, receive_keymap) = Descriptor::parse_descriptor(secp_ctx, private_descriptor).unwrap();
    let (_, change_keymap) =
        Descriptor::parse_descriptor(secp_ctx, private_change_descriptor).unwrap();

    let mut combined_keymap = receive_keymap;
    combined_keymap.extend(change_keymap);
    psbt.sign(&KeyMapWrapper::from(combined_keymap), secp_ctx)
        .unwrap();
    psbt.finalize_mut(secp_ctx).unwrap();

    // —————————————————————————————————————————————————————————————————————————————————————————————
    // APPROACH 2: Wallet::sign_with_signers
    // —————————————————————————————————————————————————————————————————————————————————————————————
    // println!("\nSigning using Wallet::sign_with_signers");
    // let (_, receive_keymap) =
    //     Descriptor::parse_descriptor(secp_ctx, private_descriptor).unwrap();
    // let (_, change_keymap) =
    //     Descriptor::parse_descriptor(secp_ctx, private_change_descriptor).unwrap();
    //
    // let receive_signers_container = SignersContainer::build(
    //     receive_keymap,
    //     wallet.public_descriptor(KeychainKind::External),
    //     secp_ctx,
    // );
    // let change_signers_container = SignersContainer::build(
    //     change_keymap,
    //     wallet.public_descriptor(KeychainKind::Internal),
    //     secp_ctx,
    // );
    //
    // let signers: &[&SignersContainer; 2] =
    //     &[&receive_signers_container, &change_signers_container];
    //
    // let psbt_was_signed_and_finalized: bool = wallet
    //     .sign_with_signers(&mut psbt, signers, SignOptions::default())
    //     .unwrap();
    // assert!(psbt_was_signed_and_finalized);

    // Get the transaction from the PSBT and broadcast
    let tx = psbt.extract_tx().unwrap();
    client.broadcast(&tx).unwrap();
    println!("Transaction broadcast! Txid: {}", tx.compute_txid());
}
