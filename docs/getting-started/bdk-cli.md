# The BDK Command Line Wallet

## Purpose

The `bdk-cli` binary crate is designed as an experimental playground for users to dive into the `BDK` libraries with ease. 

Its documentation is available [here](https://docs.rs/crate/bdk-cli/latest), and you can explore the source code [here](https://github.com/bitcoindevkit/bdk-cli).

## Features

`bdk-cli` can be compiled with different features to suit your experimental needs.

#### 1. Database Options

- `sqlite`: Sets the wallet database to a sqlite3 db. `sqlite` is a default feature.

#### 2. Blockchain Options

- `rpc`: Connects the wallet to bitcoin core via RPC.
- `electrum`: Connects the wallet to an electrum server.
- `esplora`: Connects the wallet to an esplora server.
- `cbf`: Connects the wallet to kyoto - a node and client for compact block filter based syncing.
    
#### 3. Extra Utility Tools

- `repl`: use bdk-cli as a REPL shell (useful for quick manual testing of wallet operations).
`repl` is also a default feature.
- `compiler`: opens up bdk-cli policy compiler commands.
- `verify`: uses bitcoinconsensus to verify transactions at every sync call of the wallet.

These features are non-exhaustive as more features will be added.

## Installation

You have the option to build `bdk-cli` from its source code or install directly from `crates.io`.

To build from the [source](https://github.com/bitcoindevkit/bdk-cli):

```shell
git clone git@github.com:bitcoindevkit/bdk-cli.git
cd bdk-cli/
cargo install --path . --features electrum
bdk-cli help # to verify it worked
```

To install from `crates.io`, run this command:

```shell
cargo install bdk-cli --features electrum  # add any additional features as needed
```

Note that if no blockchain client feature is enabled, online wallet commands will be disabled.

## Usage

To get usage information for the bdk-cli binary use the below command which returns a list of available wallet options and commands:

```shell
cargo run
```

Follow the instruction in [descriptors](../cookbook/keys-descriptors/descriptors.md) to generate descriptors, selecting your preferred network and descriptor type.

Assuming you exported your external and internal descriptors as `EXT_DESC` and `INT_DESC`, we can generate a new address by running the following command:

```shell
cargo run -- --network <selectednetwork> wallet -e $EXT_DESC -i $INT_DESC -d sqlite new_address
```

To sync the above wallet to a Bitcoin Core node (assuming a regtest node at 127.0.0.1:18443, username and password of bitcoin:bitcoin), run the following command:

```shell
cargo run --features rpc wallet -e $EXT_DESC -i $INT_DESC -d sqlite -c rpc -u 127.0.0.1:18443 -a bitcoin:bitcoin sync
```

## Batch payments on regtest

Run this example with Bash, `jq`, and a fresh Bitcoin Core regtest node with
RPC enabled. Use test-only RPC credentials and keep the generated wallet on
regtest. The example uses `bdk-cli` 4.0.0 with the `sqlite` and `rpc`
features.

```bash
cargo install bdk-cli --version 4.0.0 --locked --no-default-features --features sqlite,rpc
```

Create a temporary BDK wallet directory and configure the wallet to use the
regtest node. Set the RPC values to match your node before running the commands.

```bash
rpc_host=${RPC_HOST:-127.0.0.1}
rpc_user=${RPC_USER:-bitcoin}
rpc_password=${RPC_PASSWORD:-bitcoin}
bdk_dir=$(mktemp -d)

core() {
  bitcoin-cli -regtest -rpcconnect="$rpc_host" -rpcport=18443 \
    -rpcuser="$rpc_user" -rpcpassword="$rpc_password" "$@"
}

wallet() {
  bdk-cli --network regtest --datadir "$bdk_dir" wallet --wallet batch "$@"
}

descriptors=$(bdk-cli --network regtest descriptor --type wpkh)
wallet config --database-type sqlite --client-type rpc \
  --url "http://$rpc_host:18443" --basic-auth "$rpc_user:$rpc_password" \
  --ext-descriptor "$(jq -r '.private_descriptors.external' <<< "$descriptors")" \
  --int-descriptor "$(jq -r '.private_descriptors.internal' <<< "$descriptors")"
```

Mine 101 blocks to a wallet address so the first coinbase output is mature,
then scan the chain. Create two destination addresses in a separate Bitcoin
Core wallet.

```bash
funding_address=$(wallet new_address | jq -r '.address')
core generatetoaddress 101 "$funding_address" >/dev/null
wallet full_scan

core createwallet batch-recipients
recipient1=$(core -rpcwallet=batch-recipients getnewaddress)
recipient2=$(core -rpcwallet=batch-recipients getnewaddress)
```

Pass one `--to ADDRESS:AMOUNT` argument per recipient. Amounts are in satoshis.
Both payments go into the same partially signed Bitcoin transaction (PSBT).

```bash
psbt=$(wallet create_tx --to "$recipient1:20000" --to "$recipient2:30000" \
  | jq -r '.psbt')
core decodepsbt "$psbt" | jq '.tx.vout[] | {address: .scriptPubKey.address, value}'
```

Bitcoin Core displays the amounts in BTC. Look for `0.00020000 BTC` and
`0.00030000 BTC` at the respective recipient addresses. The transaction may also
contain a change output. The PSBT can be inspected without signing or
broadcasting it.
