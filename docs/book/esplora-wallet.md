# Wallet with Esplora

!!! tip
    This page is up-to-date with version `1.0.0-alpha.3` of bdk.

### Create a new Rust project
```shell
cargo init esploraexample
cd esploraexample
```
   
### Add required bdk dependencies to your Cargo.toml file
```toml
[package]
name = "electrumexample"
version = "0.1.0"
edition = "2021"

[dependencies]
bdk = { version = "1.0.0-alpha.3" }
bdk_file_store = { version = "0.3.0" }
bdk_esplora = { version = "0.7.0" }
```

### Create your wallet
Refer to the [Working with Descriptors](./descriptors.md) page for information on how to generate descriptors. This page will assume you are working on testnet with the following BIP86 descriptor:
```txt
tr(tprv8ZgxMBicQKsPewab4KfjNu6p9Q5XAPokRpK9zrPGoJS7H6CqnxuKJX6zPBDj2Q43tfmVBRTpQMBSg8AhqBDdNEsBC14kMXiZj2tPWv5wHAE/86'/1'/0'/0/*)#30pfz5ly
```
