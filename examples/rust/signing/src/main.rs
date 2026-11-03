// This code is a companion to the "Replacing SignOptions" page at
// https://bookofbdk.com/release-guide/4.0-alpha.0/sign-options
//
// For each field of `SignOptions`, it shows how to get the same behavior when signing with
// rust-bitcoin's `Psbt::sign` instead of the deprecated `Wallet::sign`, and checks the claims the
// guide makes along the way. Every check prints one line. The first one that fails stops the
// program with an error.
//
// The wallets are funded with `bdk_wallet::test_utils`, so this runs offline: each wallet starts
// with a single confirmed 50_000 sat UTXO, and a chain tip at height 2000. Instead of
// broadcasting, transactions are checked with miniscript's interpreter (`PsbtExt::extract`), which
// runs the scripts against the signatures and fails on anything that would be rejected on chain.

use std::str::FromStr;

use bdk_wallet::bitcoin::psbt::{GetKey, Input, Psbt, PsbtSighashType};
use bdk_wallet::bitcoin::secp256k1::{All, Secp256k1};
use bdk_wallet::bitcoin::sighash::{EcdsaSighashType, SighashCache, TapSighashType};
use bdk_wallet::bitcoin::taproot::TapLeafHash;
use bdk_wallet::bitcoin::{absolute, Address, Amount, Network, OutPoint, Sequence, TxOut};
use bdk_wallet::miniscript::descriptor::{DescriptorPublicKey, KeyMap, KeyMapWrapper};
use bdk_wallet::miniscript::psbt::PsbtExt;
use bdk_wallet::miniscript::Descriptor;
use bdk_wallet::signer::{SignerError, TapLeavesOptions};
use bdk_wallet::test_utils::{
    feerate_unchecked, get_test_single_sig_cltv, get_test_single_sig_csv,
    get_test_wpkh_and_change_desc, new_wallet_and_funding_update,
};
use bdk_wallet::{SignOptions, Wallet};

const SEND_TO: &str = "bcrt1q3qtze4ys45tgdvguj66zrk4fu6hq3a3v9pfly5";

/// `tr(A,{pk(B),pk(C)})`: a Taproot descriptor where we own the internal key and both leaf keys.
const TR_DESC: &str = "tr(cNJmN3fH9DDbDt131fQNkVakkpzawJBSeybCUNmP1BovpmGQ45xG,{pk(cPZzKuNmpuUjD1e8jUU4PVzy2b5LngbSip8mBsxf4e7rSFZVb4Uh),pk(cNaQCDwmmh4dS9LzCgVtyy1e1xjCJ21GUDHe9K98nzb689JvinGV)})";

/// `tr(A,{pk(A),pk(C)})`: the internal key is also used in the first leaf.
const TR_REPEATED_DESC: &str = "tr(cNJmN3fH9DDbDt131fQNkVakkpzawJBSeybCUNmP1BovpmGQ45xG,{pk(cNJmN3fH9DDbDt131fQNkVakkpzawJBSeybCUNmP1BovpmGQ45xG),pk(cNaQCDwmmh4dS9LzCgVtyy1e1xjCJ21GUDHe9K98nzb689JvinGV)})";

fn main() -> anyhow::Result<()> {
    setup()?;
    try_finalize()?;
    trust_witness_utxo()?;
    allow_all_sighashes()?;
    sign_with_tap_internal_key()?;
    repeated_internal_key()?;
    tap_leaves_options()?;
    assume_height()?;
    allow_grinding()?;
    println!("\nAll checks passed.");
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// Small utilities
// —————————————————————————————————————————————————————————————————————————————————————————————————

/// Print a claim from the guide, or stop with an error if it doesn't hold.
fn check(claim: &str, holds: bool) -> anyhow::Result<()> {
    if holds {
        println!("  ✓ {claim}");
        Ok(())
    } else {
        anyhow::bail!("claim does not hold: {claim}")
    }
}

/// Create a watch-only wallet from private descriptors, and keep the keys for ourselves.
///
/// This is the setup the guide recommends: the wallet only ever sees the public descriptors.
fn watch_only_wallet(
    desc: &str,
    change_desc: Option<&str>,
) -> anyhow::Result<(Wallet, KeyMapWrapper)> {
    // --8<-- [start:setup]
    let secp = Secp256k1::new();

    // Keep the keys; give the wallet the public descriptors.
    let (desc, mut keymap): (Descriptor<DescriptorPublicKey>, KeyMap) =
        Descriptor::parse_descriptor(&secp, desc)?;
    let change_desc = match change_desc {
        Some(change_desc) => {
            let (change_desc, change_keymap) = Descriptor::parse_descriptor(&secp, change_desc)?;
            keymap.extend(change_keymap);
            Some(change_desc.to_string())
        }
        None => None,
    };
    let signer = KeyMapWrapper::from(keymap);
    // --8<-- [end:setup]

    // `new_wallet_and_funding_update` calls `Wallet::create` (or `Wallet::create_single`) on
    // regtest, and returns an update that stands in for a sync with a blockchain client.
    let (mut wallet, _, update) =
        new_wallet_and_funding_update(&desc.to_string(), change_desc.as_deref());
    wallet.apply_update(update)?;
    Ok((wallet, signer))
}

/// The same wallet, created from the private descriptors so the deprecated `Wallet::sign` has the
/// keys. Only used to compare against.
fn wallet_with_keys(desc: &str, change_desc: Option<&str>) -> anyhow::Result<Wallet> {
    let (mut wallet, _, update) = new_wallet_and_funding_update(desc, change_desc);
    wallet.apply_update(update)?;
    Ok(wallet)
}

/// The deprecated wallet signer, which this guide replaces.
#[allow(deprecated)]
fn legacy_sign(
    wallet: &Wallet,
    psbt: &mut Psbt,
    options: SignOptions,
) -> Result<bool, SignerError> {
    wallet.sign(psbt, options)
}

/// The deprecated wallet signer with `try_finalize: false`, on a copy of the PSBT.
fn legacy_sign_only(wallet: &Wallet, psbt: &Psbt, options: SignOptions) -> anyhow::Result<Psbt> {
    let mut psbt = psbt.clone();
    let options = SignOptions {
        try_finalize: false,
        ..options
    };
    legacy_sign(wallet, &mut psbt, options)?;
    Ok(psbt)
}

fn new_psbt(wallet: &mut Wallet) -> anyhow::Result<Psbt> {
    let addr = Address::from_str(SEND_TO)?.require_network(Network::Regtest)?;
    let mut builder = wallet.build_tx();
    builder.add_recipient(addr.script_pubkey(), Amount::from_sat(10_000));
    Ok(builder.finish()?)
}

fn sign<K: GetKey>(psbt: &mut Psbt, signer: &K, secp: &Secp256k1<All>) -> anyhow::Result<()> {
    psbt.sign(signer, secp)
        .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// Setup: the flow that replaces `wallet.sign(&mut psbt, SignOptions::default())`
// —————————————————————————————————————————————————————————————————————————————————————————————————

fn setup() -> anyhow::Result<()> {
    println!("\nSetup");
    let (desc, change_desc) = get_test_wpkh_and_change_desc();
    let (mut wallet, signer) = watch_only_wallet(desc, Some(change_desc))?;
    let unsigned = new_psbt(&mut wallet)?;

    let mut watch_only = unsigned.clone();
    let signed = legacy_sign(&wallet, &mut watch_only, SignOptions::default())?;
    check(
        "the wallet has no keys: Wallet::sign on it adds no signatures",
        !signed && watch_only.inputs[0].partial_sigs.is_empty(),
    )?;

    let mut psbt = unsigned.clone();
    // --8<-- [start:baseline]
    // 1. Sign. `Psbt::sign` only adds signatures; it never finalizes.
    psbt.sign(&signer, wallet.secp_ctx())
        .map_err(|(_, errors)| anyhow::anyhow!("failed to sign PSBT: {errors:?}"))?;

    // 2. Finalize.
    let outcome = wallet.try_finalize_psbt(&mut psbt)?;
    assert!(outcome.is_finalized());

    let tx = psbt.extract_tx()?;
    // --8<-- [end:baseline]
    check(
        "Psbt::sign + Wallet::try_finalize_psbt gives a fully signed transaction",
        tx.input.iter().all(|txin| !txin.witness.is_empty()),
    )?;

    // Sign and finalize the same PSBT again, to run the result through the interpreter.
    let mut again = unsigned.clone();
    sign(&mut again, &signer, wallet.secp_ctx())?;
    assert!(wallet.try_finalize_psbt(&mut again)?.is_finalized());
    check(
        "the transaction passes miniscript's interpreter",
        again.extract(wallet.secp_ctx()).is_ok_and(|t| t == tx),
    )?;

    // Same transaction as the deprecated signer, once grinding is out of the picture (see
    // `allow_grinding` below for why that one differs).
    let private_wallet = wallet_with_keys(desc, Some(change_desc))?;
    let mut legacy = unsigned.clone();
    let options = SignOptions {
        allow_grinding: false,
        ..Default::default()
    };
    let finalized = legacy_sign(&private_wallet, &mut legacy, options)?;
    check(
        "it produces the exact transaction Wallet::sign produces (with allow_grinding: false)",
        finalized && legacy.extract_tx()? == tx,
    )?;
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// try_finalize
// —————————————————————————————————————————————————————————————————————————————————————————————————

fn try_finalize() -> anyhow::Result<()> {
    println!("\ntry_finalize");
    let (desc, change_desc) = get_test_wpkh_and_change_desc();
    let (mut wallet, signer) = watch_only_wallet(desc, Some(change_desc))?;
    let private_wallet = wallet_with_keys(desc, Some(change_desc))?;
    let unsigned = new_psbt(&mut wallet)?;

    // `try_finalize: false` is what `Psbt::sign` does out of the box.
    let mut psbt = unsigned.clone();
    sign(&mut psbt, &signer, wallet.secp_ctx())?;
    check(
        "Psbt::sign adds signatures but never finalizes (acts like try_finalize: false)",
        !psbt.inputs[0].partial_sigs.is_empty()
            && psbt.inputs[0].final_script_witness.is_none()
            && psbt.inputs[0].final_script_sig.is_none(),
    )?;

    let options = SignOptions {
        allow_grinding: false,
        ..Default::default()
    };
    let legacy = legacy_sign_only(&private_wallet, &unsigned, options)?;
    check(
        "it leaves the PSBT just like Wallet::sign with try_finalize: false",
        legacy == psbt,
    )?;

    // `try_finalize: true`, option 1: with the wallet.
    let mut with_wallet = psbt.clone();
    let outcome = wallet.try_finalize_psbt(&mut with_wallet)?;
    check(
        "Wallet::try_finalize_psbt finalizes it",
        outcome.is_finalized(),
    )?;

    // `try_finalize: true`, option 2: without the wallet, with miniscript.
    let mut with_miniscript = psbt.clone();
    with_miniscript
        .finalize_mut(wallet.secp_ctx())
        .map_err(|errors| anyhow::anyhow!("failed to finalize PSBT: {errors:?}"))?;
    check(
        "miniscript's PsbtExt::finalize_mut produces the same witness",
        with_wallet.inputs[0].final_script_witness
            == with_miniscript.inputs[0].final_script_witness,
    )?;
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// trust_witness_utxo
// —————————————————————————————————————————————————————————————————————————————————————————————————

// --8<-- [start:check-non-witness-utxos]
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
// --8<-- [end:check-non-witness-utxos]

fn trust_witness_utxo() -> anyhow::Result<()> {
    println!("\ntrust_witness_utxo");
    let (desc, change_desc) = get_test_wpkh_and_change_desc();
    let (mut wallet, signer) = watch_only_wallet(desc, Some(change_desc))?;
    let private_wallet = wallet_with_keys(desc, Some(change_desc))?;
    let secp = wallet.secp_ctx().clone();

    // 1. A PSBT built by the wallet.
    let psbt = new_psbt(&mut wallet)?;
    check(
        "a SegWit v0 PSBT from build_tx includes non_witness_utxo",
        psbt.inputs[0].non_witness_utxo.is_some(),
    )?;
    check(
        "it passes check_non_witness_utxos",
        check_non_witness_utxos(&psbt) == Ok(()),
    )?;

    // 2. A PSBT with only `witness_utxo`.
    let mut witness_only = psbt.clone();
    witness_only.inputs[0].non_witness_utxo = None;
    check(
        "a PSBT without non_witness_utxo is rejected with MissingNonWitnessUtxo",
        check_non_witness_utxos(&witness_only) == Err(UtxoCheckError::MissingNonWitnessUtxo),
    )?;
    let res = legacy_sign(
        &private_wallet,
        &mut witness_only.clone(),
        SignOptions::default(),
    );
    check(
        "Wallet::sign rejects it the same way",
        matches!(res, Err(SignerError::MissingNonWitnessUtxo)),
    )?;

    let addr = Address::from_str(SEND_TO)?.require_network(Network::Regtest)?;
    let mut builder = wallet.build_tx();
    builder
        .add_recipient(addr.script_pubkey(), Amount::from_sat(10_000))
        .only_witness_utxo();
    let built_witness_only = builder.finish()?;
    check(
        "a PSBT built with TxBuilder::only_witness_utxo is rejected too",
        check_non_witness_utxos(&built_witness_only) == Err(UtxoCheckError::MissingNonWitnessUtxo),
    )?;

    // 3. A PSBT whose `witness_utxo` lies about the amount being spent.
    let mut lying = psbt.clone();
    lying.inputs[0].witness_utxo.as_mut().unwrap().value = Amount::from_sat(1_000_000);
    check(
        "a PSBT whose witness_utxo lies about the amount is rejected with InvalidNonWitnessUtxo",
        check_non_witness_utxos(&lying) == Err(UtxoCheckError::InvalidNonWitnessUtxo),
    )?;
    let res = legacy_sign(&private_wallet, &mut lying.clone(), SignOptions::default());
    check(
        "Wallet::sign rejects it the same way",
        matches!(res, Err(SignerError::InvalidNonWitnessUtxo)),
    )?;

    // `trust_witness_utxo: true` is what `Psbt::sign` does out of the box.
    let mut honest = psbt.clone();
    sign(&mut honest, &signer, &secp)?;
    sign(&mut witness_only, &signer, &secp)?;
    sign(&mut lying, &signer, &secp)?;
    check(
        "Psbt::sign signs both of them anyway (acts like trust_witness_utxo: true)",
        !witness_only.inputs[0].partial_sigs.is_empty() && !lying.inputs[0].partial_sigs.is_empty(),
    )?;

    // The lying signature committed to the fake amount: it does not verify against the real one.
    let (msg, _) = honest.sighash_ecdsa(0, &mut SighashCache::new(&honest.unsigned_tx))?;
    let verifies = |p: &Psbt| {
        let (pk, sig) = p.inputs[0].partial_sigs.iter().next().unwrap();
        secp.verify_ecdsa(&msg, &sig.signature, &pk.inner).is_ok()
    };
    check(
        "the honest signature verifies against the real amount",
        verifies(&honest),
    )?;
    check(
        "the signature over the lying PSBT does not: it committed to the fake amount",
        !verifies(&lying),
    )?;
    check(
        "(the lying PSBT still carried the correct non_witness_utxo: Psbt::sign ignored it)",
        lying.inputs[0].non_witness_utxo == psbt.inputs[0].non_witness_utxo,
    )?;

    // 4. Taproot: the check is skipped when every input is Taproot.
    let (mut tr_wallet, tr_signer) = watch_only_wallet(TR_DESC, None)?;
    let tr_private_wallet = wallet_with_keys(TR_DESC, None)?;
    let mut tr_psbt = new_psbt(&mut tr_wallet)?;
    check(
        "a Taproot PSBT from build_tx has no non_witness_utxo",
        tr_psbt.inputs[0].non_witness_utxo.is_none(),
    )?;
    check(
        "it passes check_non_witness_utxos",
        check_non_witness_utxos(&tr_psbt) == Ok(()),
    )?;
    let res = legacy_sign(
        &tr_private_wallet,
        &mut tr_psbt.clone(),
        SignOptions::default(),
    );
    check("Wallet::sign signs it too", matches!(res, Ok(true)))?;
    sign(&mut tr_psbt, &tr_signer, tr_wallet.secp_ctx())?;
    check(
        "and so does Psbt::sign",
        tr_psbt.inputs[0].tap_key_sig.is_some(),
    )?;
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// allow_all_sighashes
// —————————————————————————————————————————————————————————————————————————————————————————————————

// --8<-- [start:check-sighashes]
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
// --8<-- [end:check-sighashes]

fn allow_all_sighashes() -> anyhow::Result<()> {
    println!("\nallow_all_sighashes");
    let (desc, change_desc) = get_test_wpkh_and_change_desc();
    let (mut wallet, signer) = watch_only_wallet(desc, Some(change_desc))?;
    let private_wallet = wallet_with_keys(desc, Some(change_desc))?;

    let psbt = new_psbt(&mut wallet)?;
    check(
        "a PSBT from build_tx leaves sighash_type unset, and passes check_sighashes",
        psbt.inputs[0].sighash_type.is_none() && check_sighashes(&psbt) == Ok(()),
    )?;
    let mut explicit_all = psbt.clone();
    explicit_all.inputs[0].sighash_type = Some(EcdsaSighashType::All.into());
    check(
        "so does one with SIGHASH_ALL set explicitly",
        check_sighashes(&explicit_all) == Ok(()),
    )?;

    // Non-standard sighash types.
    let with_sighash = |ty: EcdsaSighashType| {
        let mut psbt = psbt.clone();
        psbt.inputs[0].sighash_type = Some(ty.into());
        psbt
    };
    for ty in [
        EcdsaSighashType::None,
        EcdsaSighashType::Single,
        EcdsaSighashType::AllPlusAnyoneCanPay,
    ] {
        let mut psbt = with_sighash(ty);
        check(
            &format!("a PSBT asking for {ty} is rejected"),
            check_sighashes(&psbt) == Err(NonStandardSighash),
        )?;
        let res = legacy_sign(&private_wallet, &mut psbt.clone(), SignOptions::default());
        check(
            "  Wallet::sign rejects it with NonStandardSighash",
            matches!(res, Err(SignerError::NonStandardSighash)),
        )?;
        let options = SignOptions {
            allow_all_sighashes: true,
            ..Default::default()
        };
        check(
            "  Wallet::sign with allow_all_sighashes: true signs it",
            legacy_sign(&private_wallet, &mut psbt.clone(), options).is_ok(),
        )?;
        sign(&mut psbt, &signer, wallet.secp_ctx())?;
        let sig = psbt.inputs[0].partial_sigs.values().next().unwrap();
        check(
            &format!(
                "  Psbt::sign signs it with a {ty} signature (acts like allow_all_sighashes: true)"
            ),
            sig.sighash_type == ty,
        )?;
    }

    // `TxBuilder::sighash` is how a PSBT from build_tx ends up with one.
    let addr = Address::from_str(SEND_TO)?.require_network(Network::Regtest)?;
    let mut builder = wallet.build_tx();
    builder
        .add_recipient(addr.script_pubkey(), Amount::from_sat(10_000))
        .sighash(EcdsaSighashType::None.into());
    let built = builder.finish()?;
    check(
        "a PSBT built with TxBuilder::sighash(SIGHASH_NONE) is rejected",
        check_sighashes(&built) == Err(NonStandardSighash),
    )?;

    // Psbt::sign refuses only values that are not valid sighash types.
    let mut invalid = psbt.clone();
    invalid.inputs[0].sighash_type = Some(PsbtSighashType::from_u32(0x55));
    check(
        "Psbt::sign refuses a sighash_type that isn't a valid value",
        invalid.sign(&signer, wallet.secp_ctx()).is_err(),
    )?;

    // Taproot: SIGHASH_DEFAULT and SIGHASH_ALL both pass.
    let (mut tr_wallet, _) = watch_only_wallet(TR_DESC, None)?;
    let tr_private_wallet = wallet_with_keys(TR_DESC, None)?;
    let tr_psbt = new_psbt(&mut tr_wallet)?;
    for ty in [TapSighashType::Default, TapSighashType::All] {
        let mut psbt = tr_psbt.clone();
        psbt.inputs[0].sighash_type = Some(ty.into());
        check(
            &format!("a Taproot PSBT asking for {ty} passes, and Wallet::sign signs it"),
            check_sighashes(&psbt) == Ok(())
                && legacy_sign(&tr_private_wallet, &mut psbt, SignOptions::default()).is_ok(),
        )?;
    }
    let mut tr_none = tr_psbt.clone();
    tr_none.inputs[0].sighash_type = Some(TapSighashType::None.into());
    check(
        "a Taproot PSBT asking for SIGHASH_NONE is rejected by both",
        check_sighashes(&tr_none) == Err(NonStandardSighash)
            && matches!(
                legacy_sign(&tr_private_wallet, &mut tr_none, SignOptions::default()),
                Err(SignerError::NonStandardSighash)
            ),
    )?;
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// sign_with_tap_internal_key and tap_leaves_options
// —————————————————————————————————————————————————————————————————————————————————————————————————

// --8<-- [start:sign-taproot-paths]
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
// --8<-- [end:sign-taproot-paths]

/// The leaf hashes of the first input, sorted.
fn leaf_hashes(psbt: &Psbt) -> Vec<TapLeafHash> {
    let mut leaves: Vec<TapLeafHash> = psbt.inputs[0]
        .tap_scripts
        .values()
        .map(|(script, version)| TapLeafHash::from_script(script, *version))
        .collect();
    leaves.sort();
    leaves
}

/// The leaves the first input has script-path signatures for, sorted.
fn signed_leaves(psbt: &Psbt) -> Vec<TapLeafHash> {
    let mut leaves: Vec<TapLeafHash> = psbt.inputs[0]
        .tap_script_sigs
        .keys()
        .map(|(_, lh)| *lh)
        .collect();
    leaves.sort();
    leaves
}

fn sign_with_tap_internal_key() -> anyhow::Result<()> {
    println!(
        "\nsign_with_tap_internal_key  (descriptor: tr(A,{{pk(B),pk(C)}}), we hold A, B and C)"
    );
    let (mut wallet, signer) = watch_only_wallet(TR_DESC, None)?;
    let private_wallet = wallet_with_keys(TR_DESC, None)?;
    let secp = wallet.secp_ctx().clone();
    let psbt = new_psbt(&mut wallet)?;

    let mut plain = psbt.clone();
    sign(&mut plain, &signer, &secp)?;
    check(
        "Psbt::sign signs the key path and both leaves (acts like sign_with_tap_internal_key: true)",
        plain.inputs[0].tap_key_sig.is_some() && plain.inputs[0].tap_script_sigs.len() == 2,
    )?;
    let legacy = legacy_sign_only(&private_wallet, &psbt, SignOptions::default())?;
    check(
        "Wallet::sign with the default options signs the same paths",
        legacy.inputs[0].tap_key_sig.is_some() && signed_leaves(&legacy) == signed_leaves(&plain),
    )?;

    // --8<-- [start:no-key-path]
    let mut no_key_path = psbt.clone();
    sign_taproot_paths(&mut no_key_path, &signer, wallet.secp_ctx(), false, |_| {
        true
    })?;
    // --8<-- [end:no-key-path]
    check(
        "sign_taproot_paths with sign_key_path: false skips the key path, signs both leaves",
        no_key_path.inputs[0].tap_key_sig.is_none()
            && no_key_path.inputs[0].tap_script_sigs.len() == 2,
    )?;

    let options = SignOptions {
        sign_with_tap_internal_key: false,
        ..Default::default()
    };
    let legacy = legacy_sign_only(&private_wallet, &psbt, options)?;
    check(
        "Wallet::sign with sign_with_tap_internal_key: false signs the same paths",
        legacy.inputs[0].tap_key_sig.is_none()
            && signed_leaves(&legacy) == signed_leaves(&no_key_path),
    )?;
    check(
        "the PSBT only gained signatures: tap_internal_key and tap_key_origins are intact",
        no_key_path.inputs[0].tap_internal_key == psbt.inputs[0].tap_internal_key
            && no_key_path.inputs[0].tap_key_origins == psbt.inputs[0].tap_key_origins,
    )?;

    let outcome = wallet.try_finalize_psbt(&mut no_key_path)?;
    let witness_len = no_key_path.inputs[0]
        .final_script_witness
        .as_ref()
        .map(|w| w.len());
    check(
        "it finalizes through a script path (witness: signature, leaf script, control block)",
        outcome.is_finalized() && witness_len == Some(3),
    )?;
    check(
        "the script-path transaction passes miniscript's interpreter",
        no_key_path.extract(&secp).is_ok(),
    )?;
    Ok(())
}

fn repeated_internal_key() -> anyhow::Result<()> {
    println!("\nWhen the internal key is also in a leaf  (descriptor: tr(A,{{pk(A),pk(C)}}))");
    let (mut wallet, signer) = watch_only_wallet(TR_REPEATED_DESC, None)?;
    let private_wallet = wallet_with_keys(TR_REPEATED_DESC, None)?;
    let secp = wallet.secp_ctx().clone();
    let psbt = new_psbt(&mut wallet)?;

    let mut plain = psbt.clone();
    sign(&mut plain, &signer, &secp)?;
    check(
        "Psbt::sign signs both leaves but skips the key path",
        plain.inputs[0].tap_key_sig.is_none() && plain.inputs[0].tap_script_sigs.len() == 2,
    )?;

    let legacy = legacy_sign_only(&private_wallet, &psbt, SignOptions::default())?;
    check(
        "Wallet::sign signs all three",
        legacy.inputs[0].tap_key_sig.is_some() && legacy.inputs[0].tap_script_sigs.len() == 2,
    )?;

    // --8<-- [start:two-pass]
    let mut psbt = psbt.clone();
    // Pass 1: no leaves, so `Psbt::sign` sees the internal key as key-path only.
    sign_taproot_paths(&mut psbt, &signer, &secp, true, |_| false)?;
    // Pass 2: the leaves.
    sign_taproot_paths(&mut psbt, &signer, &secp, true, |_| true)?;
    // --8<-- [end:two-pass]
    check(
        "signing in two passes gets all three, with the same keys and leaves as Wallet::sign",
        psbt.inputs[0].tap_key_sig.is_some()
            && psbt.inputs[0]
                .tap_script_sigs
                .keys()
                .eq(legacy.inputs[0].tap_script_sigs.keys()),
    )?;
    Ok(())
}

fn tap_leaves_options() -> anyhow::Result<()> {
    println!("\ntap_leaves_options  (descriptor: tr(A,{{pk(B),pk(C)}}))");
    let (mut wallet, signer) = watch_only_wallet(TR_DESC, None)?;
    let private_wallet = wallet_with_keys(TR_DESC, None)?;
    let secp = wallet.secp_ctx().clone();
    let psbt = new_psbt(&mut wallet)?;

    let leaves = leaf_hashes(&psbt);
    assert_eq!(leaves.len(), 2);
    let chosen = vec![leaves[0]];

    type LeafFilter<'a> = Box<dyn Fn(&TapLeafHash) -> bool + 'a>;
    let cases: Vec<(TapLeavesOptions, LeafFilter, Vec<TapLeafHash>)> = vec![
        (TapLeavesOptions::All, Box::new(|_| true), leaves.clone()),
        (
            TapLeavesOptions::Include(chosen.clone()),
            Box::new(|lh| chosen.contains(lh)),
            vec![leaves[0]],
        ),
        (
            TapLeavesOptions::Exclude(chosen.clone()),
            Box::new(|lh| !chosen.contains(lh)),
            vec![leaves[1]],
        ),
        (TapLeavesOptions::None, Box::new(|_| false), vec![]),
    ];

    for (option, sign_leaf, want) in cases {
        let name = match option {
            TapLeavesOptions::All => "All",
            TapLeavesOptions::Include(_) => "Include([leaf 0])",
            TapLeavesOptions::Exclude(_) => "Exclude([leaf 0])",
            TapLeavesOptions::None => "None",
        };
        let mut signed = psbt.clone();
        sign_taproot_paths(&mut signed, &signer, &secp, true, sign_leaf)?;

        let options = SignOptions {
            tap_leaves_options: option,
            ..Default::default()
        };
        let legacy = legacy_sign_only(&private_wallet, &psbt, options)?;
        check(
            &format!(
                "{name}: signs {} leaf/leaves, the same as Wallet::sign, and the key path is unaffected",
                want.len()
            ),
            signed_leaves(&signed) == want
                && signed_leaves(&legacy) == want
                && signed.inputs[0].tap_key_sig.is_some()
                && legacy.inputs[0].tap_key_sig.is_some(),
        )?;
    }

    let mut both = psbt.clone();
    sign_taproot_paths(&mut both, &signer, &secp, false, |lh| chosen.contains(lh))?;
    let options = SignOptions {
        sign_with_tap_internal_key: false,
        tap_leaves_options: TapLeavesOptions::Include(chosen.clone()),
        ..Default::default()
    };
    let legacy = legacy_sign_only(&private_wallet, &psbt, options)?;
    check(
        "combined with sign_key_path: false, it matches both options set on Wallet::sign",
        both.inputs[0].tap_key_sig.is_none()
            && legacy.inputs[0].tap_key_sig.is_none()
            && signed_leaves(&both) == chosen
            && signed_leaves(&legacy) == chosen,
    )?;
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// assume_height
// —————————————————————————————————————————————————————————————————————————————————————————————————

fn assume_height() -> anyhow::Result<()> {
    println!("\nassume_height  (wallet tip at height 2000)");
    let cases = [
        ("and(pk(A),after(100000))", get_test_single_sig_cltv(), true),
        ("and(pk(A),older(6))", get_test_single_sig_csv(), false),
    ];

    for (policy, desc, absolute_lock) in cases {
        println!("  {policy}:");
        let (mut wallet, signer) = watch_only_wallet(desc, None)?;
        assert_eq!(wallet.latest_checkpoint().height(), 2000);
        let secp = wallet.secp_ctx().clone();
        let mut psbt = new_psbt(&mut wallet)?;
        sign(&mut psbt, &signer, &secp)?;

        if absolute_lock {
            check(
                "  build_tx sets nLockTime to what the script needs",
                psbt.unsigned_tx.lock_time == absolute::LockTime::from_height(100_000)?,
            )?;
        } else {
            check(
                "  build_tx sets nSequence to what the script needs",
                psbt.unsigned_tx.input[0].sequence == Sequence(6),
            )?;
        }

        let mut all_finalize = true;
        for assume_height in [None, Some(0), Some(2000), Some(100_000)] {
            let options = SignOptions {
                assume_height,
                ..Default::default()
            };
            all_finalize &= wallet.finalize_psbt(&mut psbt.clone(), options)?;
        }
        check(
            "  Wallet::finalize_psbt finalizes it with assume_height None, 0, 2000 and 100000 alike",
            all_finalize,
        )?;

        let mut finalized = psbt.clone();
        check(
            "  Wallet::try_finalize_psbt finalizes it at any height, no assumed height needed",
            wallet.try_finalize_psbt(&mut finalized)?.is_finalized(),
        )?;
        check(
            "  the result passes miniscript's interpreter",
            finalized.extract(&secp).is_ok(),
        )?;

        // A PSBT whose own lock fields do NOT satisfy the script.
        let mut too_early = psbt.clone();
        if absolute_lock {
            too_early.unsigned_tx.lock_time = absolute::LockTime::from_height(1500)?;
        } else {
            too_early.unsigned_tx.input[0].sequence = Sequence::ENABLE_RBF_NO_LOCKTIME;
        }
        check(
            "  with lock fields that don't satisfy the script, finalize_psbt refuses it by default",
            !wallet.finalize_psbt(&mut too_early.clone(), SignOptions::default())?,
        )?;
        let options = SignOptions {
            assume_height: Some(100_000),
            ..Default::default()
        };
        let mut forced = too_early.clone();
        check(
            "  assume_height: Some(100000) forces it to finalize",
            wallet.finalize_psbt(&mut forced, options)?,
        )?;
        check(
            "  into a transaction that fails miniscript's interpreter",
            forced.extract(&secp).is_err(),
        )?;
        check(
            "  Wallet::try_finalize_psbt refuses it",
            !wallet
                .try_finalize_psbt(&mut too_early.clone())?
                .is_finalized(),
        )?;
        check(
            "  so does miniscript's PsbtExt::finalize_mut",
            too_early.finalize_mut(&secp).is_err(),
        )?;
    }
    Ok(())
}

// —————————————————————————————————————————————————————————————————————————————————————————————————
// allow_grinding
// —————————————————————————————————————————————————————————————————————————————————————————————————

// --8<-- [start:sign-ecdsa-low-r]
/// Sign every ECDSA input with a low-R signature, like `Wallet::sign` did by default.
fn sign_ecdsa_low_r<K: GetKey>(
    psbt: &mut Psbt,
    signer: &K,
    secp: &Secp256k1<All>,
) -> anyhow::Result<()> {
    use bdk_wallet::bitcoin::psbt::KeyRequest;
    use bdk_wallet::bitcoin::{ecdsa, PublicKey};

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
// --8<-- [end:sign-ecdsa-low-r]

fn allow_grinding() -> anyhow::Result<()> {
    const ROUNDS: u64 = 32;
    println!("\nallow_grinding  ({ROUNDS} transactions, signed both ways)");
    let (desc, change_desc) = get_test_wpkh_and_change_desc();
    let (mut wallet, signer) = watch_only_wallet(desc, Some(change_desc))?;
    let private_wallet = wallet_with_keys(desc, Some(change_desc))?;
    let secp = wallet.secp_ctx().clone();
    let addr = Address::from_str(SEND_TO)?.require_network(Network::Regtest)?;
    let target = feerate_unchecked(2.0);

    let sig_len = |p: &Psbt| {
        p.inputs[0]
            .partial_sigs
            .values()
            .next()
            .unwrap()
            .to_vec()
            .len()
    };
    let (mut high_r, mut plain_matches, mut ground_matches) = (0, 0, 0);
    let (mut ground_all_low_r, mut plain_lengths_ok, mut fee_rate_met) = (true, true, true);

    for i in 0..ROUNDS {
        let mut builder = wallet.build_tx();
        builder
            .add_recipient(addr.script_pubkey(), Amount::from_sat(10_000 + i))
            .fee_rate(target);
        let psbt = builder.finish()?;

        // `allow_grinding: false` is what `Psbt::sign` does out of the box.
        let mut plain = psbt.clone();
        sign(&mut plain, &signer, &secp)?;

        // `allow_grinding: true`, with the helper above.
        let mut ground = psbt.clone();
        sign_ecdsa_low_r(&mut ground, &signer, &secp)?;

        // ECDSA signing is deterministic (RFC 6979), so the signatures can be compared exactly.
        for (allow_grinding, ours, matches) in [
            (false, &plain, &mut plain_matches),
            (true, &ground, &mut ground_matches),
        ] {
            let options = SignOptions {
                allow_grinding,
                ..Default::default()
            };
            let legacy = legacy_sign_only(&private_wallet, &psbt, options)?;
            if legacy.inputs[0].partial_sigs == ours.inputs[0].partial_sigs {
                *matches += 1;
            }
        }

        // Signature lengths include the sighash byte.
        ground_all_low_r &= sig_len(&ground) <= 71;
        plain_lengths_ok &= sig_len(&plain) <= 72;
        if sig_len(&plain) == 72 {
            high_r += 1;
        }

        for mut signed in [plain, ground] {
            assert!(wallet.try_finalize_psbt(&mut signed)?.is_finalized());
            let tx = signed.extract_tx()?;
            fee_rate_met &= wallet.calculate_fee_rate(&tx)? >= target;
        }
    }

    check(
        &format!(
            "Psbt::sign matches Wallet::sign with allow_grinding: false ({plain_matches}/{ROUNDS})"
        ),
        plain_matches == ROUNDS,
    )?;
    check(
        &format!("sign_ecdsa_low_r matches Wallet::sign with allow_grinding: true ({ground_matches}/{ROUNDS})"),
        ground_matches == ROUNDS,
    )?;
    check(
        &format!(
            "Psbt::sign does not grind: {high_r}/{ROUNDS} of its signatures are high-R (72 bytes)"
        ),
        high_r > 0 && plain_lengths_ok,
    )?;
    check(
        "every sign_ecdsa_low_r signature is 71 bytes or less",
        ground_all_low_r,
    )?;
    check(
        "the final fee rate meets the 2 sat/vb target either way",
        fee_rate_met,
    )?;

    // Taproot: Schnorr signatures are always 64 bytes, so there is nothing to grind.
    let (mut tr_wallet, tr_signer) = watch_only_wallet(TR_DESC, None)?;
    let mut tr_psbt = new_psbt(&mut tr_wallet)?;
    sign(&mut tr_psbt, &tr_signer, &secp)?;
    let before = tr_psbt.clone();
    sign_ecdsa_low_r(&mut tr_psbt, &tr_signer, &secp)?;
    check(
        "on a Taproot PSBT, sign_ecdsa_low_r leaves the Psbt::sign signatures alone",
        tr_psbt == before
            && before.inputs[0]
                .tap_key_sig
                .unwrap()
                .signature
                .as_ref()
                .len()
                == 64,
    )?;
    Ok(())
}
