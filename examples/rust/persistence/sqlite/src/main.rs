use bdk_wallet::bitcoin::Network;
use bdk_wallet::rusqlite;
use bdk_wallet::KeychainKind;
use bdk_wallet::Wallet;

// --8<-- [start:descriptors]
// const EXTERNAL_DESCRIPTOR: &str = "[your external descriptor here ...]";
// const INTERNAL_DESCRIPTOR: &str = "[your internal descriptor here ...]";
// Example private descriptors
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdJuLWWArdBsWjqDA3W5WoREnfdgKEcCQB1FMKfSoaFz9JHZU71HwXAqTsjHripkLM62kUQar14SDD8brsmhFKqVUPXGrZLc/86'/1'/0'/0/*)#fv8tutn2";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdJuLWWArdBsWjqDA3W5WoREnfdgKEcCQB1FMKfSoaFz9JHZU71HwXAqTsjHripkLM62kUQar14SDD8brsmhFKqVUPXGrZLc/86'/1'/0'/1/*)#ccz2p7rj";
// --8<-- [end:descriptors]

// The codeblocks in https://bookofbdk.com pull their code from these examples. Since we do not want an indent on the 
// codeblocks on the website, we also remove the indents here.

fn main() -> Result<(), anyhow::Error> {
// --8<-- [start:load]
let network = Network::Signet;
let file_path = "test_wallet.sqlite3";
let mut conn = rusqlite::Connection::open(file_path)?;

let wallet_opt = Wallet::load()
    .descriptor(KeychainKind::External, Some(EXTERNAL_DESCRIPTOR))
    .descriptor(KeychainKind::Internal, Some(INTERNAL_DESCRIPTOR))
    .extract_keys() // only needed if using private key descriptors
    .check_network(network)
    .load_wallet(&mut conn)?;
// --8<-- [end:load]

// --8<-- [start:create]
let mut wallet = match wallet_opt {
    Some(wallet) => {
        println!("Loaded existing wallet database.");
        wallet
    }
    None => {
        println!("Creating new wallet database.");
        Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
            .network(network)
            .create_wallet(&mut conn)?
    }
};
// --8<-- [end:create]

// --8<-- [start:address]
// Reveal a new address from your external keychain
let address = wallet.reveal_next_address(KeychainKind::External);
wallet.persist(&mut conn)?;
// Only share new address with user after successfully persisting wallet
println!(
    "Generated address {} at index {}",
    address.address, address.index
);
// --8<-- [end:address]

    Ok(())
}
