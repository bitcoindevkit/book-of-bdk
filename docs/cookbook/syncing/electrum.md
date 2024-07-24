# Sync a Wallet with Electrum

!!! tip
    This page is up-to-date with version `1.0.0-alpha.13` of bdk.

### 1. Create a new Rust project
```shell
cargo init electrumexample
cd electrumexample
```

### 2. Add required bdk dependencies to your `Cargo.toml` file
```toml
[package]
name = "electrumexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk_wallet = { version = "=1.0.0-alpha.13" }
bdk_sqlite = { version = "=0.2.0" }
bdk_electrum = { version = "=0.15.0" }
```

### 3. Create your descriptors
Refer to the [Working with Descriptors](../keys-descriptors/descriptors.md) page for information on how to generate descriptors. This page will assume you are working on signet with the following BIP86 descriptors:
```txt
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";
```

### Create and sync the wallet

```rust
use bdk_wallet::wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_electrum::{BdkElectrumClient, electrum_client};
use bdk_electrum::electrum_client::Client;

const STOP_GAP: usize = 50;
const BATCH_SIZE: usize = 5;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/1/*)";

fn main() -> () {\
    let mut wallet: Wallet = Wallet::new(
        EXTERNAL_DESCRIPTOR,
        INTERNAL_DESCRIPTOR,
        Network::Signet,
    ).unwrap();

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Sync the wallet
    let client: BdkElectrumClient<Client> = BdkElectrumClient::new(
        electrum_client::Client::new("ssl://mempool.space:60602").unwrap()
    );

    let full_scan_request = wallet.start_full_scan();
    let mut update = client
        .full_scan(full_scan_request, STOP_GAP, BATCH_SIZE, true).unwrap()
        .with_confirmation_time_height_anchor(&client).unwrap();

    let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let _ = update.graph_update.update_last_seen_unconfirmed(now);

    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sats", balance.total().to_sat());
}
```

### Notes

The `client.full_scan()` method returns an `ElectrumFullScanResult`, which _must_ be transformed into one of two `Anchors`: either a `ConfirmationHeightAnchor` or a `ConfirmationTimeHeightAnchor`. To work with a `Wallet`, the `ConfirmationTimeHeightAnchor` must be used, which you get by calling the `ElectrumFullScanResult.with_confirmation_time_height_anchor()` method.
