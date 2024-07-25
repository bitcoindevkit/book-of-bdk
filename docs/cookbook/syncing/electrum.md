# Sync a Wallet with Electrum

!!! tip
    This page is up-to-date with version `1.0.0-alpha.13` of bdk.

### Add required bdk dependencies to your `Cargo.toml` file
```toml
[dependencies]
bdk_wallet = { version = "=1.0.0-beta.1" }
bdk_electrum = { version = "=0.16.0" }
```

### Create and sync the wallet

```rust
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_electrum::{BdkElectrumClient, electrum_client};
use bdk_electrum::electrum_client::Client;

const STOP_GAP: usize = 50;
const BATCH_SIZE: usize = 5;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/1/*)";

fn main() -> () {
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Create the Electrum client
    let client: BdkElectrumClient<Client> = BdkElectrumClient::new(
        electrum_client::Client::new("ssl://mempool.space:60602").unwrap()
    );

    // Perform the initial full scan on the wallet
    let full_scan_request = wallet.start_full_scan();
    let mut update = client.full_scan(full_scan_request, STOP_GAP, BATCH_SIZE, true).unwrap();

    let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let _ = update.graph_update.update_last_seen_unconfirmed(now);

    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}
```
