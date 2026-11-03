# Signing Without the Wallet: Replacing `SignOptions`

!!! note "Overview"

    * **Applies to:** `bdk_wallet` 4.0.0-alpha.0
    * **Feature Type:** Migration Guide

## Overview

Since [3.2](../3.2/signing-deprecation.md), `Wallet` is being phased out as a key store. The recommended flow is to keep your keys outside the wallet, sign the PSBT yourself, and finalize as a separate step.

`Wallet::sign` and `Wallet::sign_with_signers` both take a `SignOptions` struct that tweaks how signing and finalization behave. The PSBT signers you would use instead take no such options. This guide goes through each `SignOptions` field and shows how to get the same behavior without the wallet signer, in two ways:

- **With rust-bitcoin:** `bitcoin::Psbt::sign`, the PSBT type `Wallet::build_tx` returns. This works today.
- **With rust-psbt:** the [rust-psbt](https://git.rust-bitcoin.org/rust-bitcoin/rust-psbt) crate (`psbt-v2` on crates.io), whose new signer checks more for you.

!!! warning "The rust-psbt sections use unreleased code"

    The rust-psbt signer described here is on rust-psbt's `master` branch, at commit [`973e4e0`](https://git.rust-bitcoin.org/rust-bitcoin/rust-psbt/commit/973e4e079c6abfbe6f51ecf256ea772023f7aa05). It is **not** in `psbt-v2` 0.3.0, the latest release, whose signer behaves like rust-bitcoin's. Expect the API to change before the next release. Each rust-psbt section stands on its own, so you can skip them all and still follow this guide.

In short, here is what you give up by moving away from `Wallet::sign`, and what you need to do about it:

- **Two safety checks you need to run yourself:** `trust_witness_utxo` and `allow_all_sighashes`. Both protect you from a malicious PSBT creator. If you only sign PSBTs your own wallet built, they cost you almost nothing. If you sign PSBTs from a coordinator or co-signer, don't skip them. rust-psbt's strict signer covers the first one for you.
- **Two options that need a small helper:** `sign_with_tap_internal_key` and `tap_leaves_options`, which choose which Taproot spend paths you sign for.
- **Three options that need nothing:** `try_finalize` (finalize as a separate step), `assume_height` (never needed, and could produce invalid transactions), and `allow_grinding` (a saving of at most 0.25 vbytes per input).

The code in this guide is checked by two programs:

- The rust-bitcoin sections: the [`sign_options` example](https://github.com/bitcoindevkit/bdk_wallet/blob/master/examples/sign_options.rs) in the `bdk_wallet` repository. Run it with `cargo run --example sign_options`.
- The rust-psbt sections: a separate program pinned to the commit above.

Both check every claim made here, including that the deprecated wallet signer and the replacement code agree.

## Setup

### With rust-bitcoin

This replaces `wallet.sign(&mut psbt, SignOptions::default())`. The wallet is created from public descriptors only, and the keys live in a `KeyMapWrapper` that you own:

```rust
use bdk_wallet::miniscript::Descriptor;
use bdk_wallet::miniscript::descriptor::KeyMapWrapper;

// Keep the keys; give the wallet the public descriptors.
let (desc, mut keymap) = Descriptor::parse_descriptor(&secp, private_desc)?;
let (change_desc, change_keymap) = Descriptor::parse_descriptor(&secp, private_change_desc)?;
keymap.extend(change_keymap);
let signer = KeyMapWrapper::from(keymap);

let mut wallet = Wallet::create(desc.to_string(), change_desc.to_string())
    .network(Network::Regtest)
    .create_wallet_no_persist()?;

// ... sync the wallet and build a PSBT with `wallet.build_tx()` ...

// 1. Sign. `Psbt::sign` only adds signatures; it never finalizes.
psbt.sign(&signer, wallet.secp_ctx())
    .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;

// 2. Finalize.
let outcome = wallet.try_finalize_psbt(&mut psbt)?;
assert!(outcome.is_finalized());

let tx = psbt.extract_tx()?;
```

### With rust-psbt

Add rust-psbt, pinned to the commit this guide was checked against. The `miniscript` feature enables its finalizer:

```toml
[dependencies]
psbt-v2 = { git = "https://git.rust-bitcoin.org/rust-bitcoin/rust-psbt", rev = "973e4e079c6abfbe6f51ecf256ea772023f7aa05", features = ["miniscript"] }
```

You need three small helpers.

**A key provider.** rust-psbt has its own `GetKey` trait. It is implemented for `Xpriv` and for maps of public to private keys, but not for miniscript's `KeyMap`, which is how BDK users usually hold their keys. This adapter forwards each request to `KeyMapWrapper`:

```rust
use bdk_wallet::bitcoin::psbt;
use bdk_wallet::bitcoin::secp256k1::{Secp256k1, Signing};
use bdk_wallet::bitcoin::PrivateKey;
use bdk_wallet::miniscript::descriptor::KeyMapWrapper;
use psbt_v2::{GetKey, KeyRequest};

/// Lets rust-psbt sign with a miniscript `KeyMap`.
struct KeyMapSigner(KeyMapWrapper);

impl GetKey for KeyMapSigner {
    type Error = psbt::GetKeyError;

    fn get_key<C: Signing>(
        &self,
        request: &KeyRequest,
        secp: &Secp256k1<C>,
    ) -> Result<Option<PrivateKey>, Self::Error> {
        let request = match request {
            KeyRequest::Pubkey(pk) => psbt::KeyRequest::Pubkey(*pk),
            KeyRequest::Bip32(source) => psbt::KeyRequest::Bip32(source.clone()),
            KeyRequest::XOnlyPubkey(xonly) => psbt::KeyRequest::XOnlyPubkey(*xonly),
            _ => return Ok(None),
        };
        psbt::GetKey::get_key(&self.0, request, secp)
    }
}
```

**Conversions.** `Wallet::build_tx` returns a rust-bitcoin `Psbt`, and `Wallet::try_finalize_psbt` takes one. You convert through the PSBT v0 serialization:

```rust
use psbt_v2::{Psbt, PsbtV0};

/// rust-bitcoin's `Psbt` (what `Wallet::build_tx` returns) to rust-psbt.
fn to_rust_psbt(psbt: &psbt::Psbt) -> anyhow::Result<Psbt> {
    Ok(PsbtV0::deserialize(&psbt.serialize())?.into_psbt())
}

/// rust-psbt back to rust-bitcoin's `Psbt` (what `Wallet::try_finalize_psbt` takes).
fn to_rust_bitcoin(psbt: Psbt) -> anyhow::Result<psbt::Psbt> {
    Ok(psbt::Psbt::deserialize(&PsbtV0::from_psbt(psbt)?.serialize())?)
}
```

**Signing.** rust-psbt's `Signer` produces signatures in a session, then writes them to the PSBT with `apply`. `sign_all` is the strict signer:

```rust
use psbt_v2::{sign_all, Signer};

/// Sign every input with the strict signer.
fn sign_strict(psbt: Psbt, signer: &KeyMapSigner) -> anyhow::Result<Psbt> {
    let secp = Secp256k1::new();
    let mut session = Signer::new(psbt)?;
    let sigs = session
        .session(sign_all(signer, &secp))
        .map_err(|errors| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;
    Ok(session.apply(sigs)?)
}
```

With those, the flow is:

```rust
let signer = KeyMapSigner(KeyMapWrapper::from(keymap));

// ... build a PSBT with `wallet.build_tx()` ...

let signed = sign_strict(to_rust_psbt(&psbt)?, &signer)?;
let mut psbt = to_rust_bitcoin(signed)?;
assert!(wallet.try_finalize_psbt(&mut psbt)?.is_finalized());
let tx = psbt.extract_tx()?;
```

## Quick Reference

What each signer does out of the box, compared to `SignOptions`' default:

| `SignOptions` field          | Default in BDK | rust-bitcoin `Psbt::sign`                                  | rust-psbt `sign_all` (unreleased)                            |
| ---------------------------- | -------------- | ---------------------------------------------------------- | ------------------------------------------------------------ |
| `try_finalize`               | `true`         | Never finalizes: [finalize separately](#try_finalize)      | Never finalizes: [finalize separately](#try_finalize)        |
| `trust_witness_utxo`         | `false`        | Acts like `true`: [add a check](#trust_witness_utxo)       | **Acts like `false`**; `sign_all_unchecked` acts like `true` |
| `allow_all_sighashes`        | `false`        | Acts like `true`: [add a check](#allow_all_sighashes)      | Acts like `true`: [add a check](#allow_all_sighashes)        |
| `sign_with_tap_internal_key` | `true`         | Acts like `true`\*: [helper](#sign_with_tap_internal_key)  | Acts like `true`\*: [helper](#sign_with_tap_internal_key)    |
| `tap_leaves_options`         | `All`          | Acts like `All`: [helper](#tap_leaves_options)             | Acts like `All`: [helper](#tap_leaves_options)               |
| `assume_height`              | `None`         | Not needed: [drop it](#assume_height)                      | Not needed, but see [a lock time caveat](#assume_height)     |
| `allow_grinding`             | `true`         | Acts like `false`: [usually fine](#allow_grinding)         | Acts like `false`: [usually fine](#allow_grinding)           |

\* Except when the internal key is also used in a leaf. See [When the internal key is also in a leaf](#when-the-internal-key-is-also-in-a-leaf).

## `try_finalize`

**What it did:** when `true` (the default), `Wallet::sign` tried to finalize the PSBT right after adding signatures. It returned `true` if every input was finalized. When `false`, it only added signatures.

**Without the wallet signer:** neither signer finalizes, so `try_finalize: false` is what you get by default. A signed but not finalized PSBT is what you pass on to the next signer in a multisig setup. To get `try_finalize: true`, finalize as a separate step.

### With rust-bitcoin

You have two choices:

```rust
use bdk_wallet::miniscript::psbt::PsbtExt;

psbt.sign(&signer, wallet.secp_ctx())
    .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;

// Option 1: with the wallet. Reports the outcome for each input.
let outcome = wallet.try_finalize_psbt(&mut psbt)?;
assert!(outcome.is_finalized());

// Option 2: without the wallet, using miniscript.
psbt.finalize_mut(wallet.secp_ctx())
    .map_err(|errors| anyhow::anyhow!("failed to finalize PSBT: {errors:?}"))?;
```

The example checks that both finalizers produce the same witness.

!!! tip

    `Wallet::try_finalize_psbt` returns a `FinalizePsbtOutcome` that tells you why each input could not be finalized. `Wallet::sign` only returned `false`. If you build PSBTs with the unstable `Wallet::create_psbt`, use the `Finalizer` it returns instead.

### With rust-psbt

Either convert back and use the wallet, or use rust-psbt's own `Finalizer`:

```rust
use psbt_v2::Finalizer;

let signed = sign_strict(to_rust_psbt(&psbt)?, &signer)?;

// Option 1: back to rust-bitcoin, finalize with the wallet.
let mut psbt = to_rust_bitcoin(signed.clone())?;
assert!(wallet.try_finalize_psbt(&mut psbt)?.is_finalized());

// Option 2: rust-psbt's finalizer. It returns a new, finalized PSBT.
let finalized = Finalizer::new(signed)?.finalize(&secp)?;
```

The check program confirms that both produce the same witness.

!!! warning

    At this commit, rust-psbt's `Finalizer` cannot finalize spends that use an absolute timelock (`after`) on a PSBT converted from rust-bitcoin, unless you apply the workaround in [`assume_height`](#assume_height). The wallet's finalizer is not affected.

## `trust_witness_utxo`

**What it did:** when `false` (the default), `Wallet::sign` refused to sign unless every input had a `non_witness_utxo` (the full previous transaction) that matched the outpoint being spent. If a `witness_utxo` was also present, the two had to agree. It returned `SignerError::MissingNonWitnessUtxo` or `SignerError::InvalidNonWitnessUtxo` otherwise. The check was skipped if every input spent a Taproot output.

**Why it matters:** a SegWit v0 signature commits to the amount of the input it signs, but not to the amounts of the other inputs. A malicious PSBT creator can lie about input amounts in `witness_utxo` across two signing rounds and trick you into paying a much larger fee than you agreed to. This is the [SegWit fee vulnerability](https://blog.trezor.io/details-of-firmware-updates-for-trezor-one-version-1-9-1-and-trezor-model-t-version-2-3-1-1eba8f60f2dd). The full previous transaction can't be faked, because its hash must match the txid in the outpoint. Taproot fixes this at the protocol level: BIP 341 signatures commit to the amounts and scripts of *all* inputs, so if any of them is a lie, the signature is invalid.

!!! note

    PSBTs built by `Wallet::build_tx` include `non_witness_utxo` for non-Taproot inputs by default, so they pass this check. They fail only if you called `TxBuilder::only_witness_utxo`, or if you added a foreign UTXO without its previous transaction.

### With rust-bitcoin

`Psbt::sign` does no such check. It reads the amount from `witness_utxo` when one is present, and only falls back to `non_witness_utxo` when there is none. It never checks that the two agree. So `Psbt::sign` always behaves like `trust_witness_utxo: true`.

!!! warning

    If you sign PSBTs created by someone else (a coordinator, a co-signer, a hardware wallet companion app), `Psbt::sign` will happily sign a PSBT whose `witness_utxo` lies about the amount, even if the PSBT also has a correct `non_witness_utxo`. Run the check below before you sign.

To keep the default behavior, run this check before calling `Psbt::sign`:

```rust
use bdk_wallet::bitcoin::psbt::{Input, Psbt};
use bdk_wallet::bitcoin::{OutPoint, TxOut};

#[derive(Debug, PartialEq)]
enum UtxoCheckError {
    MissingNonWitnessUtxo,
    InvalidNonWitnessUtxo,
}

/// Reproduce the check `Wallet::sign` runs when `SignOptions::trust_witness_utxo` is `false`.
fn check_non_witness_utxos(psbt: &Psbt) -> Result<(), UtxoCheckError> {
    let inputs = || psbt.inputs.iter().zip(&psbt.unsigned_tx.input);

    let all_taproot = inputs().all(|(input, txin)| {
        validated_prevout(input, txin.previous_output)
            .or(input.witness_utxo.as_ref())
            .is_some_and(|txout| txout.script_pubkey.is_p2tr())
    });
    if all_taproot {
        return Ok(());
    }

    for (input, txin) in inputs() {
        if input.non_witness_utxo.is_none() {
            return Err(UtxoCheckError::MissingNonWitnessUtxo);
        }
        let prevout = validated_prevout(input, txin.previous_output)
            .ok_or(UtxoCheckError::InvalidNonWitnessUtxo)?;
        if input.witness_utxo.as_ref().is_some_and(|w| w != prevout) {
            return Err(UtxoCheckError::InvalidNonWitnessUtxo);
        }
    }
    Ok(())
}

/// The output being spent, taken from `non_witness_utxo` only if it is the right transaction.
fn validated_prevout(input: &Input, outpoint: OutPoint) -> Option<&TxOut> {
    let tx = input.non_witness_utxo.as_ref()?;
    if tx.compute_txid() != outpoint.txid {
        return None;
    }
    tx.output.get(outpoint.vout as usize)
}
```

Then sign only if it passes:

```rust
check_non_witness_utxos(&psbt).map_err(|e| anyhow::anyhow!("refusing to sign: {e:?}"))?;
psbt.sign(&signer, wallet.secp_ctx())
    .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;
```

To get `trust_witness_utxo: true`, skip the check. Only do this if you trust whoever created the PSBT, or if every input is Taproot.

The example checks three cases against both this function and the deprecated `Wallet::sign`:

1. A PSBT built by the wallet passes.
2. A PSBT with its `non_witness_utxo` removed is rejected with `MissingNonWitnessUtxo`.
3. A PSBT whose `witness_utxo` claims the wrong amount is rejected with `InvalidNonWitnessUtxo`. `Psbt::sign` signs this PSBT anyway, and its signature differs from the one over the honest PSBT, which shows that it committed to the fake amount.

### With rust-psbt

This is built in. rust-psbt has two signers, and they map directly onto the option:

| rust-psbt                      | Acts like                   |
| ------------------------------ | --------------------------- |
| `sign_all` (strict)            | `trust_witness_utxo: false` |
| `sign_all_unchecked`           | `trust_witness_utxo: true`  |

```rust
use psbt_v2::{sign_all, sign_all_unchecked, Signer};

let mut session = Signer::new(psbt)?;

// trust_witness_utxo: false
let sigs = session.session(sign_all(&signer, &secp));

// trust_witness_utxo: true
let sigs = session.session(sign_all_unchecked(&signer, &secp));
```

The strict signer refuses a SegWit v0 input that has only a `witness_utxo`, with `SignError::FundingUtxo(FundingUtxoError::UnverifiableUtxo)`. Witness-only Taproot inputs are accepted, so a Taproot PSBT built by `Wallet::build_tx` (which has no `non_witness_utxo`) signs fine.

The unchecked signer is safer than `trust_witness_utxo: true` was. It still refuses a PSBT whose `witness_utxo` and `non_witness_utxo` disagree, with `FundingUtxoError::MismatchingUtxos`. It only waives the requirement to have the full previous transaction.

The strict signer also checks more than `Wallet::sign` did. It refuses inputs whose redeem or witness script doesn't match the output being spent, whose Taproot internal key doesn't produce the output key, or whose existing signatures use a different sighash type.

The check program confirms:

1. The strict signer signs a PSBT built by the wallet.
2. It refuses the witness-only PSBT, which the unchecked signer signs.
3. Both refuse the PSBT with the fake amount.
4. The strict signer signs a witness-only Taproot PSBT.

## `allow_all_sighashes`

**What it did:** when `false` (the default), `Wallet::sign` refused to sign the whole PSBT if any input asked for a sighash type other than `SIGHASH_ALL` (or `SIGHASH_DEFAULT` for Taproot). It returned `SignerError::NonStandardSighash`. An input with no `sighash_type` set was fine, since signers then use `SIGHASH_ALL`. When `true`, it signed with whatever sighash type each input asked for.

**Why it matters:** the sighash type decides which parts of the transaction your signature commits to. `SIGHASH_ALL` commits to every input and every output, so nobody can change the transaction after you sign. The other types commit to less:

- `SIGHASH_NONE` commits to no outputs. Anyone who sees your signature can send the funds wherever they like.
- `SIGHASH_SINGLE` commits only to the output at the same index as your input.
- `ANYONECANPAY` (combined with any of the above) commits only to your own input, so others can add inputs.

These have real uses, such as crowdfunding or some offer protocols, but a PSBT from someone else should never be able to pick one without you noticing.

!!! note

    PSBTs built by `Wallet::build_tx` leave `sighash_type` unset unless you call `TxBuilder::sighash`, so they pass this check.

### With rust-bitcoin

`Psbt::sign` signs with the sighash type the PSBT asks for, as long as it is one of the standard values. It refuses only invalid values. So `Psbt::sign` always behaves like `allow_all_sighashes: true`.

!!! warning

    The sighash type is set by whoever created the PSBT. If you sign PSBTs created by someone else, `Psbt::sign` will sign a `SIGHASH_NONE` input without complaint. Run the check below before you sign.

To keep the default behavior, run this check before calling `Psbt::sign`:

```rust
use bdk_wallet::bitcoin::psbt::Psbt;
use bdk_wallet::bitcoin::sighash::{EcdsaSighashType, TapSighashType};

#[derive(Debug, PartialEq)]
struct NonStandardSighash;

/// Reproduce the check `Wallet::sign` runs when `SignOptions::allow_all_sighashes` is `false`.
fn check_sighashes(psbt: &Psbt) -> Result<(), NonStandardSighash> {
    let all_standard = psbt.inputs.iter().all(|input| {
        input.sighash_type.is_none()
            || input.sighash_type == Some(EcdsaSighashType::All.into())
            || input.sighash_type == Some(TapSighashType::All.into())
            || input.sighash_type == Some(TapSighashType::Default.into())
    });
    if all_standard {
        Ok(())
    } else {
        Err(NonStandardSighash)
    }
}
```

Then sign only if it passes:

```rust
check_sighashes(&psbt).map_err(|_| anyhow::anyhow!("refusing to sign: non-standard sighash"))?;
psbt.sign(&signer, wallet.secp_ctx())
    .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;
```

To get `allow_all_sighashes: true`, skip the check. If you only want to allow some types, for example `SIGHASH_ALL | ANYONECANPAY`, add them to the list in `check_sighashes`.

The example checks these cases against both this function and the deprecated `Wallet::sign`:

1. A PSBT built by the wallet (no `sighash_type`) passes, and so does one with `SIGHASH_ALL` set explicitly.
2. A PSBT asking for `SIGHASH_NONE` is rejected. `Wallet::sign` rejects it with `NonStandardSighash` under the default options and signs it with `allow_all_sighashes: true`.
3. `Psbt::sign` signs the `SIGHASH_NONE` PSBT, and the signature it produces is a `SIGHASH_NONE` signature.

### With rust-psbt

Not built in. Even the strict signer signs with any valid sighash type the PSBT asks for, so it acts like `allow_all_sighashes: true`. Run the same check on rust-psbt's PSBT type before signing. The only difference is that rust-psbt has its own `PsbtSighashType`:

```rust
use bdk_wallet::bitcoin::{EcdsaSighashType, TapSighashType};
use psbt_v2::{Psbt, PsbtSighashType};

#[derive(Debug, PartialEq)]
struct NonStandardSighash;

/// Reproduce the check `Wallet::sign` runs when `SignOptions::allow_all_sighashes` is `false`.
fn check_sighashes(psbt: &Psbt) -> Result<(), NonStandardSighash> {
    let all_standard = psbt.inputs.iter().all(|input| {
        input.sighash_type.is_none()
            || input.sighash_type == Some(PsbtSighashType::from(EcdsaSighashType::All))
            || input.sighash_type == Some(PsbtSighashType::from(TapSighashType::All))
            || input.sighash_type == Some(PsbtSighashType::from(TapSighashType::Default))
    });
    if all_standard {
        Ok(())
    } else {
        Err(NonStandardSighash)
    }
}
```

The check program confirms that this check passes a PSBT built by the wallet and rejects one asking for `SIGHASH_NONE`. The strict signer signs that PSBT anyway, producing a `SIGHASH_NONE` signature.

## `sign_with_tap_internal_key`

**What it did:** when `true` (the default), `Wallet::sign` signed the Taproot key path whenever it had the internal key. When `false`, it skipped the key path and only signed script paths.

**Why you'd turn it off:** the internal key is sometimes not meant to be used on its own, for example when it is a MuSig2 aggregate key whose signature has to be produced in a separate protocol. Or you may want to force a script-path spend.

Both signers sign the key path whenever they have the internal key, so out of the box they act like `sign_with_tap_internal_key: true`. Getting `false` takes a small helper, which also covers [`tap_leaves_options`](#tap_leaves_options).

### With rust-bitcoin

`Psbt::sign` decides what to sign from two PSBT fields:

- It signs the key path if it has the key for the input's `tap_internal_key`.
- It signs every leaf listed in `tap_key_origins` for a key it has.

To restrict what it signs, sign a copy of the PSBT with those fields trimmed, then copy the new signatures back into the original:

```rust
use bdk_wallet::bitcoin::psbt::{GetKey, Psbt};
use bdk_wallet::bitcoin::secp256k1::{All, Secp256k1};
use bdk_wallet::bitcoin::taproot::TapLeafHash;

/// Sign `psbt`, but only on the Taproot spend paths we choose.
fn sign_taproot_paths<K: GetKey>(
    psbt: &mut Psbt,
    signer: &K,
    secp: &Secp256k1<All>,
    sign_key_path: bool,
    sign_leaf: impl Fn(&TapLeafHash) -> bool,
) -> anyhow::Result<()> {
    let mut scratch = psbt.clone();
    for input in &mut scratch.inputs {
        if !sign_key_path {
            input.tap_internal_key = None;
        }
        for (leaf_hashes, _) in input.tap_key_origins.values_mut() {
            leaf_hashes.retain(|lh| sign_leaf(lh));
        }
    }

    scratch
        .sign(signer, secp)
        .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;

    for (input, signed) in psbt.inputs.iter_mut().zip(scratch.inputs) {
        input.partial_sigs.extend(signed.partial_sigs);
        input.tap_key_sig = input.tap_key_sig.or(signed.tap_key_sig);
        input.tap_script_sigs.extend(signed.tap_script_sigs);
    }
    Ok(())
}
```

The original PSBT only ever gains signatures. Its `tap_internal_key` and `tap_key_origins` stay intact, which the finalizer and other signers need. Signing a copy also means that if signing fails halfway, the original is left as it was.

To get `sign_with_tap_internal_key: false`:

```rust
sign_taproot_paths(&mut psbt, &signer, wallet.secp_ctx(), false, |_| true)?;
```

The example signs a PSBT for `tr(A,{pk(B),pk(C)})` while holding all three keys, and checks that:

1. Plain `Psbt::sign` produces a key-path signature and both leaf signatures.
2. `sign_taproot_paths` with `sign_key_path: false` produces no key-path signature, and the same two leaf signatures as the deprecated `Wallet::sign` with `sign_with_tap_internal_key: false`.
3. The PSBT still finalizes, through a script path.

### With rust-psbt

rust-psbt produces signatures before writing them, and tags each one as key-path or script-path. Filtering those would be the natural way to do this, but at this commit the types involved (`TypedSigs` and `TapKeyMaterial`) are not exported, so your code can't name them. Instead, sign everything, then remove the signatures you didn't want:

```rust
use bdk_wallet::bitcoin::TapLeafHash;
use psbt_v2::Psbt;

/// Sign every input, then keep only the new Taproot signatures for the paths we want.
fn sign_taproot_paths(
    psbt: Psbt,
    signer: &KeyMapSigner,
    keep_key_path: bool,
    keep_leaf: impl Fn(&TapLeafHash) -> bool,
) -> anyhow::Result<Psbt> {
    let original = psbt.clone();
    let mut signed = sign_strict(psbt, signer)?;
    for (input, original) in signed.inputs.iter_mut().zip(&original.inputs) {
        if !keep_key_path {
            input.tap_key_sig = original.tap_key_sig;
        }
        input
            .tap_script_sigs
            .retain(|key, _| original.tap_script_sigs.contains_key(key) || keep_leaf(&key.1));
    }
    Ok(signed)
}
```

Signatures that were already in the PSBT before you signed are always kept. To get `sign_with_tap_internal_key: false`:

```rust
let signed = sign_taproot_paths(to_rust_psbt(&psbt)?, &signer, false, |_| true)?;
```

The check program runs the same case as the rust-bitcoin example. It confirms that the result has no key-path signature, the same leaf signatures as the deprecated `Wallet::sign`, and still finalizes through a script path.

### When the Internal Key Is Also in a Leaf

There is one case where neither signer acts like `sign_with_tap_internal_key: true`: when the internal key is also used in one of the leaves, as in `tr(A,{pk(A),pk(B)})`. Both only sign the key path for a key whose `tap_key_origins` entry has no leaf hashes (their reading of BIP 371), so here they skip it. `Wallet::sign` signed it.

Both programs confirm this: the signers produce the two leaf signatures but no key-path signature, while the deprecated `Wallet::sign` produces all three. Most wallets never use their internal key in a leaf, so most users won't hit this. If you do need the key-path signature too:

**With rust-bitcoin**, sign in two passes:

```rust
// Pass 1: no leaves, so `Psbt::sign` sees the internal key as key-path only.
sign_taproot_paths(&mut psbt, &signer, secp, true, |_| false)?;
// Pass 2: the leaves.
sign_taproot_paths(&mut psbt, &signer, secp, true, |_| true)?;
```

**With rust-psbt**, sign a copy where the internal key has no leaf hashes, and take its key-path signature:

```rust
let mut key_path_only = psbt.clone();
for input in &mut key_path_only.inputs {
    let internal_key = input.tap_internal_key.unwrap();
    if let Some((leaf_hashes, _)) = input.tap_key_origins.get_mut(&internal_key) {
        leaf_hashes.clear();
    }
}
let key_path_only = sign_strict(key_path_only, &signer)?;

let mut signed = sign_strict(psbt, &signer)?;
for (input, other) in signed.inputs.iter_mut().zip(key_path_only.inputs) {
    input.tap_key_sig = input.tap_key_sig.or(other.tap_key_sig);
}
```

## `tap_leaves_options`

**What it did:** it chose which script-path leaves `Wallet::sign` signed for. The key path was not affected.

| `TapLeavesOptions` | Signed leaves                                 |
| ------------------ | --------------------------------------------- |
| `All` (default)    | Every leaf it had a key for                   |
| `Include(leaves)`  | Only the listed leaves                        |
| `Exclude(leaves)`  | Every leaf except the listed ones             |
| `None`             | No leaves                                     |

Both signers sign every leaf they have a key for, so out of the box they act like `All`. Each of the other variants becomes a filter passed to the `sign_taproot_paths` helper from [`sign_with_tap_internal_key`](#sign_with_tap_internal_key).

To find a leaf's hash, compute it from the scripts in the PSBT:

```rust
let leaves: Vec<TapLeafHash> = psbt.inputs[0]
    .tap_scripts
    .values()
    .map(|(script, version)| TapLeafHash::from_script(script, *version))
    .collect();
```

### With rust-bitcoin

```rust
// Include(leaves)
sign_taproot_paths(&mut psbt, &signer, secp, true, |lh| leaves.contains(lh))?;

// Exclude(leaves)
sign_taproot_paths(&mut psbt, &signer, secp, true, |lh| !leaves.contains(lh))?;

// None
sign_taproot_paths(&mut psbt, &signer, secp, true, |_| false)?;
```

To combine this with `sign_with_tap_internal_key: false`, pass `false` as the fourth argument.

The example runs all four variants on the same `tr(A,{pk(B),pk(C)})` PSBT. For each one, it checks that `sign_taproot_paths` signs exactly the expected leaves, and the same leaves as the deprecated `Wallet::sign`.

### With rust-psbt

```rust
// Include(leaves)
let signed = sign_taproot_paths(to_rust_psbt(&psbt)?, &signer, true, |lh| leaves.contains(lh))?;

// Exclude(leaves)
let signed = sign_taproot_paths(to_rust_psbt(&psbt)?, &signer, true, |lh| !leaves.contains(lh))?;

// None
let signed = sign_taproot_paths(to_rust_psbt(&psbt)?, &signer, true, |_| false)?;
```

To combine this with `sign_with_tap_internal_key: false`, pass `false` as the third argument.

The check program runs all four variants, and each one keeps the same leaf signatures as the deprecated `Wallet::sign`.

## `assume_height`

**What it claimed to do:** the documentation says `assume_height` lets the wallet "assume a specific height has been reached" when finalizing, so that it can use timelocks that only become valid in the future. It was read only by `Wallet::finalize_psbt`, which `Wallet::sign` called when `try_finalize` was `true`.

**What it actually did:** much less, and the one thing it changed is something you don't want. The example checks this with an absolute timelock (`and(pk(A),after(100000))`) and a relative one (`and(pk(A),older(6))`), with the wallet's tip at height 2000:

- **For PSBTs built by `Wallet::build_tx`, it has no effect.** `build_tx` already sets `nLockTime` or `nSequence` to what the spending path needs. The finalizer accepts a timelock if *either* the transaction's own fields *or* the assumed height satisfy it, so these PSBTs finalize whatever you pass. `None`, `Some(0)`, `Some(2000)` and `Some(100000)` all give the same result, even though the tip is far below the lock.
- **For a PSBT whose own fields don't satisfy the lock, it forces finalization anyway.** Say a PSBT spends the `after(100000)` path but has `nLockTime` set to 1500. `finalize_psbt` refuses it by default. With `assume_height: Some(100000)`, it finalizes it. But `OP_CHECKLOCKTIMEVERIFY` and `OP_CHECKSEQUENCEVERIFY` check the transaction's own fields, not the chain height, so the result can never be valid. The example confirms this with miniscript's interpreter (`PsbtExt::extract`), which rejects the forced transaction.

So you don't need a replacement. If you set `assume_height` to finalize PSBTs that didn't set their own lock fields, fix how those PSBTs are built instead, for example with `TxBuilder::nlocktime` or `TxBuilder::set_exact_sequence`. Finalizing them with an assumed height only ever produced invalid transactions.

!!! note

    A finalized transaction with a future timelock is valid but not yet *final*: nodes won't relay or mine it until the chain reaches the lock. That is a question for when you broadcast, not for when you finalize. Keep the finalized transaction and broadcast it once the lock has passed.

### With rust-bitcoin

`Wallet::try_finalize_psbt` and miniscript's `PsbtExt::finalize_mut` both read timelocks from the transaction itself:

- They finalize a correctly built timelocked PSBT right away, at any chain height.
- They refuse a PSBT whose fields don't satisfy its lock, which is what you want.

```rust
// Works for timelocked spends too; no assumed height needed.
let outcome = wallet.try_finalize_psbt(&mut psbt)?;
assert!(outcome.is_finalized());
```

### With rust-psbt

You can convert back and use the wallet as above. rust-psbt's own `Finalizer` also reads timelocks from the PSBT, with one caveat at this commit:

- **Relative timelocks (`older`)** work as is. They are read from each input's `nSequence`, which the conversion keeps.
- **Absolute timelocks (`after`)** don't finalize on a PSBT converted from rust-bitcoin. A v0 PSBT stores its lock in the transaction's `nLockTime`, which the conversion turns into PSBT v2's global *fallback* lock time. The finalizer only checks `after` against each input's *required* lock time (`PSBT_IN_REQUIRED_HEIGHT_LOCKTIME`), so it never sees it.

Until that's fixed, copy the lock time into the inputs after converting. This doesn't change the transaction or its txid, so you can do it before or after signing:

```rust
use bdk_wallet::bitcoin::absolute;
use psbt_v2::{Psbt, RequiredHeightLockTime};

/// Copy the transaction's height-based `nLockTime` into each input's `min_height`.
fn copy_lock_time_to_inputs(psbt: &mut Psbt) -> anyhow::Result<()> {
    if let absolute::LockTime::Blocks(height) = psbt.determine_lock_time()? {
        if height.to_consensus_u32() > 0 {
            for input in &mut psbt.inputs {
                input.min_height = Some(RequiredHeightLockTime::try_from(height)?);
            }
        }
    }
    Ok(())
}
```

```rust
let mut psbt = to_rust_psbt(&psbt)?;
copy_lock_time_to_inputs(&mut psbt)?;
let finalized = Finalizer::new(sign_strict(psbt, &signer)?)?.finalize(&secp)?;
```

The check program confirms, at height 2000:

1. Without the workaround, the `after(100000)` PSBT fails to finalize, while the wallet finalizes it.
2. With it, the PSBT finalizes and its txid is unchanged.
3. With `nLockTime` set too low, the PSBT is still refused.
4. The `older(6)` PSBT finalizes without the workaround, and is refused when its `nSequence` doesn't satisfy the lock.

Only height-based locks are handled here. Time-based locks would need `min_time` instead.

## `allow_grinding`

**What it did:** when `true` (the default), `Wallet::sign` made "low-R" ECDSA signatures. ECDSA signing picks a nonce whose point `R` becomes part of the signature. About half the time, `R` has its top bit set, and its DER encoding needs an extra byte. Grinding retries with a different nonce until it gets a low `R`. That keeps every signature at 71 bytes (with the sighash byte) instead of 72. When `false`, it signed once and kept whatever came out.

**Does it matter?** Usually not:

- **Fees:** a high-R signature makes the transaction 1 witness byte larger, which is 0.25 vbytes. BDK estimates fees assuming the largest possible signature, so you never pay *less* than the fee rate you asked for. The example checks this for 32 transactions, signed both ways. Without grinding, you just miss out on a saving of up to 0.25 vbytes per input.
- **Taproot:** Schnorr signatures are always 64 bytes, so grinding doesn't apply. Taproot inputs are unaffected either way.

### With rust-bitcoin

`Psbt::sign` does not grind, and has no option to. So it always acts like `allow_grinding: false`. In the example, roughly half of the 32 signatures from `Psbt::sign` come out high-R (the exact count varies between runs, because `build_tx` shuffles outputs).

**If you want low-R signatures anyway**, for example to match another signer exactly, sign the ECDSA inputs yourself:

```rust
use bdk_wallet::bitcoin::psbt::{GetKey, KeyRequest, Psbt};
use bdk_wallet::bitcoin::secp256k1::{All, Secp256k1};
use bdk_wallet::bitcoin::sighash::SighashCache;
use bdk_wallet::bitcoin::{PublicKey, ecdsa};

/// Sign every ECDSA input with a low-R signature, like `Wallet::sign` did by default.
fn sign_ecdsa_low_r<K: GetKey>(
    psbt: &mut Psbt,
    signer: &K,
    secp: &Secp256k1<All>,
) -> anyhow::Result<()> {
    let tx = psbt.unsigned_tx.clone();
    let mut cache = SighashCache::new(&tx);

    for i in 0..psbt.inputs.len() {
        let keys: Vec<_> = psbt.inputs[i]
            .bip32_derivation
            .clone()
            .into_iter()
            .collect();
        for (pk, key_source) in keys {
            let request = |r| signer.get_key(r, secp).ok().flatten();
            let Some(sk) = request(KeyRequest::Bip32(key_source))
                .or_else(|| request(KeyRequest::Pubkey(PublicKey::new(pk))))
            else {
                continue;
            };
            let (msg, sighash_type) = psbt.sighash_ecdsa(i, &mut cache)?;
            let signature = ecdsa::Signature {
                signature: secp.sign_ecdsa_low_r(&msg, &sk.inner),
                sighash_type,
            };
            psbt.inputs[i]
                .partial_sigs
                .insert(sk.public_key(secp), signature);
        }
    }
    Ok(())
}
```

This looks up keys the same way `Psbt::sign` does, by key origin first and then by public key. It only handles legacy and SegWit v0 inputs. For a PSBT that also has Taproot inputs, call `Psbt::sign` first, then this helper. It replaces the ECDSA signatures with low-R ones and leaves the Taproot signatures alone.

ECDSA signing is deterministic (RFC 6979), so the example can compare signatures exactly. For all 32 transactions, `Psbt::sign` produces the same signature as the deprecated `Wallet::sign` with `allow_grinding: false`. `sign_ecdsa_low_r` produces the same signature as `Wallet::sign` with `allow_grinding: true`.

### With rust-psbt

rust-psbt doesn't grind either. Its ECDSA signatures are byte-for-byte identical to rust-bitcoin's, so it acts like `allow_grinding: false`. The check program confirms this for 16 transactions, against both `Psbt::sign` and the deprecated `Wallet::sign` with `allow_grinding: false`.

rust-psbt doesn't expose the sighash computation its signer uses. If you want low-R signatures, convert to rust-bitcoin and use the `sign_ecdsa_low_r` helper above.
