# ChangeSet Version Compatibility Policy

!!! note "Overview"

    * **Lead Developer:** [@ValuedMammal]
    * **Ticket:** [#234]
    * **Pull Request:** [#391]
    * **Feature Type:** Non-breaking

## Overview

BDK 3.0 formalizes a version compatibility policy for `ChangeSet`, the data structure used to persist wallet state. The policy defines which upgrade and downgrade paths are supported and how breaking changes should be handled across major versions.

## Why Do This?

`ChangeSet` is the serialized representation of everything a BDK wallet needs to reconstruct its state. As BDK evolves, fields are added and removed, and without a clear policy, users are left guessing which version combinations are safe when migrating stored data or building applications that must support multiple BDK versions simultaneously.

## The Policy

The version compatibility rules are now documented directly in the `ChangeSet` source:

- **Changes to `ChangeSet` correspond to a major version bump.** Structural changes to persisted data are considered breaking and will not happen in minor or patch releases.

- **Version N can read data written by version N-1.** One major version of forward migration is guaranteed, but compatibility does not extend to N-2 or earlier.

- **New fields must implement `Default`.** When deserializing a `ChangeSet` written by an older version, any missing fields are filled in with their default value. This is what makes the N reads N-1 guarantee work in practice.

- **Version N-1 can deserialize version N data by ignoring unknown fields**, with the caveat that features introduced in version N will not be available.

- **A 3-version deprecation cycle** applies to fields being removed. A field will be:
    1. Version N: Marked deprecated
    2. Version N+1: Present but unused
    3. Version N+2: Removed

## Practical Impact for 3.0

As part of this PR, the newly added `locked_outpoints` field (introduced in [#259]) received the `#[serde(default)]` attribute:

```rust
#[serde(default)]
pub locked_outpoints: locked_outpoints::ChangeSet,
```

This means a wallet database written by BDK 2.x can be deserialized by BDK 3.0 without error — the `locked_outpoints` field will simply be empty by default.

[@ValuedMammal]: https://github.com/ValuedMammal
[#234]: https://github.com/bitcoindevkit/bdk_wallet/issues/234
[#391]: https://github.com/bitcoindevkit/bdk_wallet/pull/391
[#259]: https://github.com/bitcoindevkit/bdk_wallet/pull/259
