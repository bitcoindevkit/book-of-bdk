# `Wallet::create_psbt` API

!!! note "Overview"

    * **Lead Developer:** [@ValuedMammal]
    * **Pull Request:** [#516]
    * **Feature Type:** Non-Breaking/Unstable

## Overview
`Wallet::create_psbt` is a new method for constructing a PSBT through the new `PsbtParams` API. It returns both the built `Psbt` and `Finalizer` that can be used to finalize PSBTs after signatures are added.

This API is built on the new planning stack (`bdk_tx` + `miniscript::plan`), and is paired with a related RBF feature (`replace_by_fee`).

Please note this feature is marked *unstable*. It is intended for early adopters and may change in a minor release without a semver-major bump.

To enable this experimental feature, you must opt in to both gates:

1. Enable the `bdk-tx` cargo feature on `bdk_wallet`
1. Pass `--cfg bdk_wallet_unstable` to rustc (for example via `.cargo/config.toml`)

```toml title="Cargo.toml"
[dependencies]
bdk_wallet = { version = "3.2", features = ["bdk-tx"] }
```

```toml title=".cargo/config.toml"
[build]
rustflags = ["--cfg", "bdk_wallet_unstable"]
```


## Why Do This?
The legacy transaction-construction path in `bdk_wallet` relies on older internals (`policy.rs` and legacy coin selection) that grew increasingly obsolete. The new `create_psbt` design modernizes this flow by integrating with:

- miniscript planning (`plan`) for spending-path construction
- `bdk_tx` for transaction construction primitives
- `bdk_coin_select` for coin selection logic

This separation results in a relatively safe deployment, improves maintainability, and provides a better experience while giving users direct control over spending constraints, selection policy, and advanced workflows (including foreign inputs and RBF construction). The broader design rationale is documented in [Replacing the Transaction Builder].


## New Method on `Wallet`

```rust
impl Wallet {
    /// Creates a PSBT with the given `params`
    pub fn create_psbt(
        &mut self,
        params: PsbtParams<CreateTx>,
    ) -> Result<(Psbt, Finalizer), CreatePsbtError>
    {
        // ...
    }
}
```


## Example: Create a new PSBT

The following example shows a basic `create_psbt` flow where specific UTXOs are selected, a recipient is added, and coin selection is configured with `SelectionStrategy::LowestFee`.

```rust
use bdk_wallet::{Wallet, PsbtParams, SelectionStrategy};
use bitcoin::{Amount, Address, FeeRate, OutPoint};

let mut params = PsbtParams::default();
params
    .add_utxos(&[outpoint])
    .add_recipients([(address, amount)])
    .coin_selection(SelectionStrategy::LowestFee)
    .fee_rate(FeeRate::from_sat_per_vb(5));

let (psbt, finalizer) = wallet.create_psbt(params)?;
```

## FAQs

**What are "assets"?**

Use `PsbtParams::add_assets` when:

- A spending plan requires a condition the wallet doesn't know about (e.g. a future timelock)
- To define the spending policy when multiple spending paths exist

**What if I need to spend a "foreign" UTXO?**

Use `PsbtParams::add_planned_input`. To do that you need to know how to create a miniscript `Plan` and a `bdk_tx::Input`.
  
**How to control the method of coin selection used?**

There are multiple options for managing coin selection:

- Select UTXOs manually by outpoint
- Choose one of the provided SelectionStrategy
- Define a Custom selection algorithm

**I need to sweep funds to an address but don't yet know the outgoing amount**

A Bitcoin transaction has both inputs and outputs. Among the outputs are the specified recipients and optionally an output carrying "change" back to the wallet. The value of the change is set dynamically based on the excess amount remaining after all recipients are funded and accounting for fees. In some cases it may be desirable to sweep coins to a single address where we're not particularly concerned about the amount as long as it confirms in a timely manner. This is especially true whenever increasing the fee rate causes the excess amount to diminish. In that case we don't know the output amount until after the transaction is built! How to create such a transaction?

1. Leave the recipients empty.
1. Set the change script explicitly (e.g. a descriptor at a defined index). This tells the wallet "I definitely intend to send change to this address".
1. Set `SelectionStrategy::All` if you wish to drain the wallet of all available funds.
1. If you know the inputs precisely and don't want to drain the wallet entirely, use `manually_selected_only`.

[@ValuedMammal]: https://github.com/ValuedMammal
[#516]: https://github.com/bitcoindevkit/bdk_wallet/pull/516
[Replacing the Transaction Builder]: https://hackmd.io/@bdk/r11JIeIjxl
