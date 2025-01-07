# Sync a Wallet with Bitcoin Core RPC

### 1. Start a regtest bitcoin daemon
For this example you'll need to run a bitcoin core daemon locally in regtest mode. Here are some of the commands you'll need:
```shell
# In a shell dedicated to the bitcoin daemon 
bitcoind --chain=regtest

# In a new shell dedicated to the bitcoin-cli
bitcoin-cli --chain=regtest getblockchaininfo

bitcoin-cli --chain=regtest createwallet mywallet
bitcoin-cli --chain=regtest loadwallet mywallet
bitcoin-cli --chain=regtest getnewaddress

# Mine 101 blocks
bitcoin-cli --chain=regtest generatetoaddress 101 <address>

# Send to address
bitcoin-cli --chain=regtest sendtoaddress <address> <amount>
```

### 2. Create a new Rust project
```shell
cargo init rpcexample
cd rpcexample
```

### 3. Add required bdk dependencies to your `Cargo.toml` file

```toml title="Cargo.toml"
--8<-- "examples/rust/syncing/rpc/Cargo.toml"
```

### 4. Create your descriptors
Refer to the [Working with Descriptors](../keys-descriptors/descriptors.md) page for information on how to generate descriptors. This page will assume you are working on Regtest with the following BIP86 descriptors:
```txt
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";
```

### 5. Create and sync wallet

```rust title="main.rs"
--8<-- "examples/rust/syncing/rpc/src/main.rs"
```

Once you have synced the wallet once, mine a few new blocks using the bitcoin-cli and send coins to the address provided by the wallet and printed in the console. Upon running the example code again, your wallet will sync up the latest blocks and update its balance.
