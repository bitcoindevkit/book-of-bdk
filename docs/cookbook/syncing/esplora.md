# Sync a Wallet with Esplora

!!! tip
    This page is up-to-date with version `1.0.0-alpha.13` of bdk.

### Create a new Rust project
```shell
cargo init esploraexample
cd esploraexample
```

### Add required bdk dependencies to your Cargo.toml file
```toml
[package]
name = "esploraexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk = { version = "1.0.0-alpha.13" }
bdk_esplora = { version = "0.15.0", features = ["blocking"] }
```

### 3. Create your descriptors

Refer to the [Working with Descriptors](../keys-descriptors/descriptors.md) page for information on how to generate descriptors. This page will assume you are working on signet with the following BIP86 descriptors:
```txt
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";
```

### 4. Create and sync the wallet

```rust
use bdk_wallet::wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> () {
    let mut wallet: Wallet = Wallet::new(
        EXTERNAL_DESCRIPTOR,
        INTERNAL_DESCRIPTOR,
        Network::Signet,
    ).unwrap();

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Syncing the wallet
    let client = Builder::new("http://signet.bitcoindevkit.net").build_blocking();
    let full_scan_request = wallet.start_full_scan();
    let mut update = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();
    let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let _ = update.graph_update.update_last_seen_unconfirmed(now);
    wallet.apply_update(update).unwrap();

    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}
```
