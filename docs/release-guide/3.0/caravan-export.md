# Caravan Wallet Format Import/Export

!!! note "Overview"

    * **Lead Developer:** [@vkprogrammer-001]
    * **Ticket:** [#5]
    * **Pull Request:** [#205]
    * **Feature Type:** Non-breaking

## Overview

BDK now supports importing and exporting wallets using the [Caravan] multisig wallet format. A new `CaravanExport` struct handles serialization to and from Caravan's JSON configuration, making it straightforward to move multisig wallet configurations between BDK and Caravan-compatible coordinators.

## Why Do This?

[Caravan] is a widely used stateless multisig coordinator. Teams building multisig workflows often need to share wallet configurations across tools. By supporting the Caravan format natively, BDK wallets can participate in Caravan-based multisig setups without requiring custom serialization code.

## Exporting a Wallet to Caravan Format

!!! warning

    While _importing_ a Caravan wallet should always work, you should only attempt to _export_ a wallet with descriptors that Caravan understands.

```rust
use bdk_wallet::wallet::export::CaravanExport;

let wallet = Wallet::create(
    "wsh(sortedmulti(2,[73756c7f/48h/0h/0h/2h]tpubDCKxNyM3bLgbEX13Mcd8mYxbVg9ajDkWXMh29hMWBurKfVmBfWAM96QVP3zaUcN51HvkZ3ar4VwP82kC8JZhhux8vFQoJintSpVBwpFvyU3/0/*,[f9f62194/48h/0h/0h/2h]tpubDDp3ZSH1yCwusRppH7zgSxq2t1VEUyXSeEp8E5aFS8m43MknUjiF1bSLo3CGWAxbDyhF1XowA5ukPzyJZjznYk3kYi6oe7QxtX2euvKWsk4/0/*))",
    "wsh(sortedmulti(2,[73756c7f/48h/0h/0h/2h]tpubDCKxNyM3bLgbEX13Mcd8mYxbVg9ajDkWXMh29hMWBurKfVmBfWAM96QVP3zaUcN51HvkZ3ar4VwP82kC8JZhhux8vFQoJintSpVBwpFvyU3/1/*,[f9f62194/48h/0h/0h/2h]tpubDDp3ZSH1yCwusRppH7zgSxq2t1VEUyXSeEp8E5aFS8m43MknUjiF1bSLo3CGWAxbDyhF1XowA5ukPzyJZjznYk3kYi6oe7QxtX2euvKWsk4/1/*))",
)
    .network(Network::Testnet)
    .create_wallet_no_persist()?;

let export = CaravanExport::export_wallet(&wallet, "my-multisig")?;
let json = export.to_string();
println!("{}", json);
```

The resulting JSON can be loaded directly into Caravan to reconstruct the wallet configuration.

## Importing a Caravan Config into BDK

```rust
use bdk_wallet::wallet::export::CaravanExport;

let json = std::fs::read_to_string("caravan-export.json")?;
let import = CaravanExport::from_str(&json)?;
let (descriptor, change_descriptor) = import.to_descriptors()?;

let wallet = Wallet::create(descriptor, change_descriptor)
    .network(Network::Bitcoin)
    .create_wallet_no_persist()?;
```

[@vkprogrammer-001]: https://github.com/vkprogrammer-001
[#5]: https://github.com/bitcoindevkit/bdk_wallet/issues/5
[#205]: https://github.com/bitcoindevkit/bdk_wallet/pull/205
[Caravan]: https://github.com/caravan-bitcoin/caravan
