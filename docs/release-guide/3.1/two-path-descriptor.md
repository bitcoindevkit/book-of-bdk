# `LoadParams::two_path_descriptor` Method

!!! note "Overview"

    * **Lead Developer:** [@yukibtc]
    * **Pull Request:** [#418]
    * **Feature Type:** Non-Breaking

## Overview

`LoadParams::two_path_descriptor` is a new convenience method that validates both the external and internal wallet keychains against a single two-path descriptor when loading a wallet from persistence. It is the counterpart to `Wallet::create_from_two_path_descriptor` on the load side.

## Why Do This?

BDK has supported creating wallets from a single two-path descriptor (one that derives both the external and change keychains) since it was introduced. However, when loading an existing wallet back from persistence, users had to supply the two paths separately using `LoadParams::descriptor(KeychainKind::External, ...)` and `LoadParams::descriptor(KeychainKind::Internal, ...)`.

`two_path_descriptor` makes the load path as ergonomic as the create path: pass in the same descriptor you used at creation time and BDK validates both keychains automatically.

## New Method on `LoadParams`

```rust
pub fn two_path_descriptor<D>(self, expected_descriptor: D) -> Self
where
    D: IntoWalletDescriptor + Send + Clone + 'static,
```

The method derives the external path (index `0`) and the internal/change path (index `1`) from the provided descriptor, then validates each against the corresponding keychain stored in persistence.

!!! note

    If the expected descriptor contains private keys and you want BDK to register them as signers, you must also call `.extract_keys()` on `LoadParams`.

## Example: Creating and Reloading a Wallet

```rust
use bdk_wallet::{Wallet, KeychainKind};
use bdk_wallet::bitcoin::Network;
use bdk_chain::rusqlite;

let two_path_descriptor = "wpkh([73c5da0a/84'/1'/0']tpubDCKxNyM3bLgbEX13Mcd8mYxbVg9ajDkWXMh29hMWBurKfVmBfWAM96QVP3zaUcN51HvkZ3ar4VwP82kC8JZhhux8vFQoJintSpVBwpFvyU3/<0;1>/*)";

let mut db = rusqlite::Connection::open("wallet.db")?;

// Create the wallet on first run
let mut wallet = Wallet::create_from_two_path_descriptor(two_path_descriptor)
    .network(Network::Testnet4)
    .create_wallet(&mut db)?;

wallet.reveal_addresses_to(KeychainKind::External, 5);
wallet.persist(&mut db)?;

// On subsequent runs, load and validate with the same descriptor
let loaded = Wallet::load()
    .two_path_descriptor(two_path_descriptor)
    .check_network(Network::Testnet4)
    .load_wallet(&mut db)?
    .expect("wallet must exist");

assert_eq!(loaded.derivation_index(KeychainKind::External), Some(5));
```

[@yukibtc]: https://github.com/yukibtc
[#418]: https://github.com/bitcoindevkit/bdk_wallet/pull/418
