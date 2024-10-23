# Wallet Recovery from Seeds

BDK wallets require the use of descriptors, but recovery phrases (also called seed phrases) are a common and popular backup solution. Creating descriptors from a recovery phrase is a common workflow and BDK makes this easy with its _descriptor templates_, which are offered for common descriptors ([BIP 44/49/84/86](https://docs.rs/bdk_wallet/latest/bdk_wallet/descriptor/template/index.html)).

!!! note "Feature Flags"

    The current example requires the feature `keys-bip39` for `bdk_wallet`.
    You can add it with:
    
    ```bash
    cargo add bdk_wallet --features="keys-bip39"
    ```

### Example

```rust
--8<-- "examples/rust/wallet-recovery/src/main.rs"
```
