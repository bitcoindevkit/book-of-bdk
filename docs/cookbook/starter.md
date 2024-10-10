# Full Wallet Example

!!! tip
    This page is up-to-date with version `1.0.0-beta.5` of `bdk_wallet`.

## Create a new Rust project

```shell
cargo init bdkexample
cd bdkexample
```

## Add required dependencies to your `Cargo.toml` file

```toml
[package]
name = "bdkexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk_wallet = { version = "=1.0.0-beta.5", features = ["file_store"] }
bdk_esplora = { version = "=0.19.0", features = ["blocking"] }
```

## Create your descriptors

Refer to the [Working with Descriptors](./keys-descriptors/descriptors.md) page for information on how to generate descriptors. This page will assume you are working on signet with the following BIP86 descriptors:
```rust
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";
```

## Create a wallet, sync it, build a transaction, and broadcast it in `src/main.rs`

```rust
use bdk_wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_esplora::EsploraExt;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::esplora_client;
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResult};

const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 1;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> () {
    // Create the wallet
    let mut wallet: Wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .unwrap();

    // Reveal a new address from your external keychain
    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Sync the wallet
    let client: esplora_client::BlockingClient = Builder::new("http://signet.bitcoindevkit.net").build_blocking();
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResult<KeychainKind> = client.full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS).unwrap();

    // Apply the update from the full scan to the wallet
    wallet.apply_update(update).unwrap();

    // Query the wallet balance again
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}
```

## Build and run:

```shell
cargo build
cargo run
```