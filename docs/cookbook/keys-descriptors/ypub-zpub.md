# Why BDK Does Not Support ypub and zpub

BDK is a [BIP-380](https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki) descriptor wallet library. It takes **xpub** / **tpub** (and the matching private **xprv** / **tprv**) keys and puts the script type in the descriptor text. It does not parse Electrum-style **ypub** or **zpub** strings.

!!! danger
    The keys on this page are for illustration only. Do not use them with real funds.

## Two different ways to encode “how to pay me”

An extended public key has two jobs people often glue together:

1. **The key material** — a BIP-32 node you can derive children from.
2. **The script type** — P2WPKH, nested SegWit, Taproot, and so on.

[BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) **xpub** only does job 1. The version bytes say “mainnet public” or “testnet public”. They do not say which address type to build.

[SLIP-132](https://github.com/satoshilabs/slips/blob/master/slip-0132.md) **ypub** / **zpub** (and related **Ypub** / **Zpub**) overload the version bytes so the string also implies a script type and, in wallet software, a derivation path. That scheme is **not** a BIP. `rust-bitcoin` does not treat those version bytes as first-class keys, and BDK does not either.

Descriptors do both jobs in text, without hiding either one:

- Script type is the outer function: `pkh()`, `sh(wpkh())`, `wpkh()`, `tr()`, …
- Key origin and derivation are inside the key expression: `[fingerprint/84h/0h/0h]xpub…/0/*`

That is unambiguous. A reviewer can read the string and know the script, the origin, and the wildcard path.

## Equivalents (when you must translate)

If another wallet shows you a SLIP-132 key, this is the descriptor BDK expects instead. The xpub inside is the same BIP-32 node; only the wrapper changes.

| SLIP-132 key | Implied script | Descriptor BDK understands |
| --- | --- | --- |
| `xpub` (BIP-32) | usually P2PKH if used “bare” | `pkh([fpr/44h/0h/0h]xpub…/0/*)` |
| `ypub` | nested SegWit P2SH-P2WPKH | `sh(wpkh([fpr/49h/0h/0h]xpub…/0/*))` |
| `zpub` | native SegWit P2WPKH | `wpkh([fpr/84h/0h/0h]xpub…/0/*)` |

Use `1h` in place of `0h` on testnet/signet coin type, and `tpub` instead of `xpub`. Change addresses are the same descriptor with `/1/*` on the last path step.

BDK’s [descriptor templates](./descriptors.md) (`Bip49`, `Bip84`, `Bip86`, …) already emit this form. You do not need a zpub to get `bc1q` addresses.

## Why mixing the two is wrong

SLIP-132 and descriptors both claim “I know the script type.” If you paste a zpub *into* a descriptor, the two claims can disagree.

A zpub means **native SegWit v0**. This is well-formed BIP-380:

```text
wpkh([deadbeef/84h/0h/0h]xpub…/0/*)
```

This is not. It asks for Taproot outputs while the key version bytes (if they were honored) would mean P2WPKH:

```text
tr([deadbeef/86h/0h/0h]zpub…/0/*)
```

Hardware wallets and recovery UIs have shipped that mix (for example a SLIP-132 zpub nested inside a descriptor string). BDK will not try to guess which side wins. Disable the SLIP-132 toggle on the other device and export an **xpub plus descriptor**, or convert using the table above.

The same collision exists for `ypub` inside `wpkh()` or `tr()`.

## Importing a wallet that only gives you a zpub

Sometimes you still have to talk to software that only copies a zpub. Treat that as a recovery corner case, not the format you display to users:

1. Decode the SLIP-132 version bytes to a raw BIP-32 xpub (the payload is the same; only the four version bytes differ).
2. Wrap that xpub in the matching descriptor from the table, including origin if you have it.
3. Pass the descriptor string to `Wallet::create` / `Wallet::load`.

Keep zpub/ypub behind a warning if your product must accept them at all. Prefer copying the full descriptor so the script type never detaches from the key.

## Further reading

- [BIP-380 Output Script Descriptors](https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki)
- [Creating Keys and Descriptors](./descriptors.md)
- [SLIP-132](https://github.com/satoshilabs/slips/blob/master/slip-0132.md) (why other wallets show ypub/zpub)
- rust-bitcoin discussion of SLIP-132 version bytes: [rust-bitcoin#2190](https://github.com/rust-bitcoin/rust-bitcoin/issues/2190)
