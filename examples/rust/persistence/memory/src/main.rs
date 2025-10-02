use bdk_wallet::bitcoin::Network;
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::Wallet;

// --8<-- [start:descriptors]
// Example private descriptors
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdJuLWWArdBsWjqDA3W5WoREnfdgKEcCQB1FMKfSoaFz9JHZU71HwXAqTsjHripkLM62kUQar14SDD8brsmhFKqVUPXGrZLc/86'/1'/0'/0/*)#fv8tutn2";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdJuLWWArdBsWjqDA3W5WoREnfdgKEcCQB1FMKfSoaFz9JHZU71HwXAqTsjHripkLM62kUQar14SDD8brsmhFKqVUPXGrZLc/86'/1'/0'/1/*)#ccz2p7rj";
// --8<-- [end:descriptors]

fn main() -> Result<(), anyhow::Error> {
    // --8<-- [start:create]
    let mut wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .expect("valid wallet");
    // --8<-- [end:create]

    // --8<-- [start:address]
    // Reveal a new address from your external keychain
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!(
        "Generated address {} at index {}",
        address.address, address.index
    );
    // --8<-- [end:address]

    Ok(())
}
