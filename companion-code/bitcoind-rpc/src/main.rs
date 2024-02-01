#![allow(unused)]
use bdk_bitcoind_rpc::bitcoincore_rpc;
use bdk_bitcoind_rpc::bitcoincore_rpc::Auth;
use bdk_bitcoind_rpc::bitcoincore_rpc::Client;
use bdk_bitcoind_rpc::bitcoincore_rpc::RpcApi;
use bdk_bitcoind_rpc::Emitter;

use bdk::bitcoin::Amount;
use bdk::bitcoin::Network;

use bdk::wallet::AddressIndex;
use bdk::SignOptions;
use bdk::Wallet;

// Amount to send.
const AMOUNT: u64 = 2_000_000;

// `Result` boilerplate.
type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Create Wallet
    let external_descriptor = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly";

    let mut wallet = Wallet::new_no_persist(
        external_descriptor,
        None,
        Network::Regtest,
    )?;

    let balance = wallet.get_balance().total();
    println!("Wallet balance: {} sat", balance);

    let bdk_addr = wallet.get_address(AddressIndex::New).address;
    println!("Wallet new address: {}", bdk_addr);

    // Setup Core
    let client = bitcoincore_rpc::Client::new(
        "http://127.0.0.1:18443/wallet/test",
        Auth::UserPass("user".to_string(), "password".to_string()),
    )?;

    let height = match client.get_block_count() {
        Ok(h) => h,
        Err(e) => {
            println!("unable to connect to bitcoind; caused by {e}");
            return Ok(());
        }
    };
    
    println!(
        "Connected to Bitcoin Core at block {}: {:?}",
        height,
        client.get_best_block_hash()?,
    );
    println!();

    client.create_wallet(
        "test", /*disable_private_keys: */ None, /*blank: */ None,
        /*passphrase: */ None, /*avoid_reuse: */ None,
    )?;

    let core_addr = client
        .get_new_address(/*label: */ None, /*address_type: */ None)?
        .assume_checked();

    println!("Generating blocks...");
    client.generate_to_address(101, &core_addr)?;

    let core_balance = client
        .get_balance(/*minconf: */ None, /*include_watch_only: */ None)?
        .to_btc();
    println!("Core balance: {} BTC", core_balance);

    // Configure chain source emitter
    let start_height = 0u32;
    let mut emitter = Emitter::new(&client, wallet.latest_checkpoint(), start_height);

    // Sync (#1)
    sync(&mut wallet, &mut emitter)?;
    println!();

    println!("Sending 2M sat Core -> Bdk");
    client.send_to_address(
        &bdk_addr,
        Amount::from_sat(AMOUNT),
        /*comment: */ None,
        /*comment_to: */ None,
        /*subtract_fee: */ None,
        /*replaceable: */ None,
        /*confirmation_target: */ None,
        /*estimate_mode: */ None,
    )?;

    let unconfirmed = emitter.mempool()?;
    wallet.apply_unconfirmed_txs(unconfirmed.iter().map(|(tx, time)| (tx, *time)));
    wallet.commit()?;

    let balance = wallet.get_balance().untrusted_pending;
    println!("Wallet pending: {} sat", balance);

    client.generate_to_address(1, &core_addr)?;

    // Sync (#2)
    sync(&mut wallet, &mut emitter)?;

    let balance = wallet.get_balance().confirmed;
    assert_eq!(balance, AMOUNT);
    println!("Confirmed! {} sat", balance);
    println!();

    println!("Sending 1M sat Bdk -> Core");
    let mut tx_builder = wallet.build_tx();
    let amount = AMOUNT / 2;
    tx_builder.add_recipient(core_addr.script_pubkey(), amount);

    let mut psbt = tx_builder.finish()?;

    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized);

    let tx = psbt.extract_tx();
    let fee = wallet.calculate_fee(&tx)?;
    let txid = client.send_raw_transaction(&tx)?;
    println!("Txid: {}", txid);

    client.generate_to_address(1, &core_addr)?;

    // Sync (#3)
    sync(&mut wallet, &mut emitter)?;

    let core_balance = client.get_balance(None, None)?.to_btc();
    let bdk_balance = wallet.get_balance().total();
    assert!(bdk_balance < amount);
    println!("Core balance: {} BTC", core_balance);
    println!("Wallet balance: {} sat ", bdk_balance);
    println!("Network fee: {} sat", fee);

    Ok(())
}

// Calls `Emitter::next_block`, applying transactions we care about to the wallet's
// transaction graph until all events have been consumed.
fn sync(wallet: &mut Wallet, emitter: &mut Emitter<Client>) -> Result<()> {
    print!("Syncing... ");
    while let Some(event) = emitter.next_block()? {
        let block = &event.block;
        let height = event.block_height();
        wallet.apply_block_connected_to(block, height, event.connected_to())?;
    }
    wallet.commit()?;

    println!("Ok");
    Ok(())
}
