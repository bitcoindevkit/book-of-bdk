#![allow(dead_code)]
#![allow(unused_must_use)]

use bdk::chain::bitcoin::hashes::Hash;
use bdk::chain::bitcoin::{BlockHash, Transaction};
use bdk::chain::example_utils::tx_from_hex;
use bdk::chain::local_chain::LocalChain;
use bdk::chain::local_chain::{ChangeSet, CheckPoint};
use bdk::chain::{BlockId, ConfirmationTimeHeightAnchor, TxGraph};

use bdk::bitcoin::Network;
use bdk::Wallet;
use std::collections::BTreeMap;

fn main() -> () {
    checkpoints();
    // part1();
    // part2();
    // part3();
    // part4();
    // part5();
}

fn checkpoints() -> () {
    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdRvpdnGWLRrcEkQzdxBanKRFLucEZ2NopN8KFB4ir8hzht33JKFj4WmKwdW4qCbePqHK8gm1cDU6BBTkmGjUhpFWjyr7M1Z/84'/1'/0'/0/*)";

    let mut wallet = Wallet::new_or_load(external_descriptor, None, (), Network::Testnet).unwrap();

    let genesis_block_checkpoint: CheckPoint = wallet.latest_checkpoint();
    println!(
        "Genesis block checkpoint: \n{:#?}\n",
        genesis_block_checkpoint
    );

    let local_chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
            (12, Hash::hash("twelve".as_bytes())),
            (17, Hash::hash("seventeen".as_bytes())),
        ]
        .into_iter()
        .collect::<BTreeMap<u32, BlockHash>>(),
    )
    .unwrap();

    println!("Local chain checkpoints: \n{:#?}\n", local_chain.tip());
}

fn part5() -> () {
    pub const RAW_TX_1: &str = "0200000000010116d6174da7183d70d0a7d4dc314d517a7d135db79ad63515028b293a76f4f9d10000000000feffffff023a21fc8350060000160014531c405e1881ef192294b8813631e258bf98ea7a1027000000000000225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c024730440220591b1a172a122da49ba79a3e79f98aaa03fd7a372f9760da18890b6a327e6010022013e82319231da6c99abf8123d7c07e13cf9bd8d76e113e18dc452e5024db156d012102318a2d558b2936c52e320decd6d92a88d7f530be91b6fe0af5caf41661e77da3ef2e0100";
    let tx: Transaction = tx_from_hex(RAW_TX_1);

    let mut graph = TxGraph::<ConfirmationTimeHeightAnchor>::default();
    graph.insert_tx(tx.clone());

    let confirmation_time_height_anchor2 = ConfirmationTimeHeightAnchor {
        anchor_block: BlockId {
            height: 2,
            hash: Hash::hash("second".as_bytes()),
        },
        confirmation_height: 2,
        confirmation_time: 100,
    };

    let chain_a = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
        ]
        .into_iter()
        .collect::<BTreeMap<u32, BlockHash>>(),
    )
    .unwrap();

    graph.insert_anchor(tx.txid(), confirmation_time_height_anchor2);

    let block_4 = BlockId {
        height: 4,
        hash: Hash::hash("fourth".as_bytes()),
    };

    // let txs = graph.try_list_chain_txs(&chain_a, block_3);
    let txs = graph.list_chain_txs(&chain_a, block_4);
    println!("Transactions: {:#?}\n", txs.collect::<Vec<_>>());
}

fn part4() -> () {
    let mut graph = TxGraph::<()>::default();
    println!("Empty graph: {:#?}\n", graph);

    pub const RAW_TX_1: &str = "0200000000010116d6174da7183d70d0a7d4dc314d517a7d135db79ad63515028b293a76f4f9d10000000000feffffff023a21fc8350060000160014531c405e1881ef192294b8813631e258bf98ea7a1027000000000000225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c024730440220591b1a172a122da49ba79a3e79f98aaa03fd7a372f9760da18890b6a327e6010022013e82319231da6c99abf8123d7c07e13cf9bd8d76e113e18dc452e5024db156d012102318a2d558b2936c52e320decd6d92a88d7f530be91b6fe0af5caf41661e77da3ef2e0100";
    let tx: Transaction = tx_from_hex(RAW_TX_1);

    let changeset_1 = graph.insert_tx(tx.clone());
    println!("New graph: {:#?}\n", graph);
    println!("Changeset: {:#?}\n", changeset_1);
}

fn part1() -> () {
    let chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("first".as_bytes())),
            (1, Hash::hash("second".as_bytes())),
        ]
        .into_iter()
        .collect::<BTreeMap<u32, BlockHash>>(),
    );

    println!("Chain: {:#?}", chain.unwrap());
}

fn part2() -> () {
    let (chain, changeset): (LocalChain, ChangeSet) =
        LocalChain::from_genesis_hash(Hash::hash("genesis".as_bytes()));

    println!("Chain: {:#?}", chain);
    println!("Changeset: {:#?}", changeset)
}

fn part3() -> () {
    pub const RAW_TX_1: &str = "0200000000010116d6174da7183d70d0a7d4dc314d517a7d135db79ad63515028b293a76f4f9d10000000000feffffff023a21fc8350060000160014531c405e1881ef192294b8813631e258bf98ea7a1027000000000000225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c024730440220591b1a172a122da49ba79a3e79f98aaa03fd7a372f9760da18890b6a327e6010022013e82319231da6c99abf8123d7c07e13cf9bd8d76e113e18dc452e5024db156d012102318a2d558b2936c52e320decd6d92a88d7f530be91b6fe0af5caf41661e77da3ef2e0100";

    // TxGraph is generic over its Anchor type.
    let mut graph = TxGraph::<ConfirmationTimeHeightAnchor>::default();
    let tx: Transaction = tx_from_hex(RAW_TX_1);
    // println!("Graph 0: {:#?}\n", graph);

    let confirmation_time_height_anchor = ConfirmationTimeHeightAnchor {
        anchor_block: BlockId {
            height: 4,
            hash: Hash::hash("fourth".as_bytes()),
        },
        confirmation_height: 4, // TODO: Why is this allowed to be different than the height above?
        confirmation_time: 123,
    };
    // println!("ConfirmationTimeHeightAnchor: {:#?}\n", confirmation_time_height_anchor);

    let confirmation_time_height_anchor2 = ConfirmationTimeHeightAnchor {
        anchor_block: BlockId {
            height: 2,
            hash: Hash::hash("second".as_bytes()),
        },
        confirmation_height: 2, // TODO: Why is this allowed to be different than the height above?
        confirmation_time: 100,
    };

    graph.insert_tx(tx.clone());

    // println!("============ Graph without anchors ============: {:#?}\n", graph);

    let chain_a = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
        ]
        .into_iter()
        .collect::<BTreeMap<u32, BlockHash>>(),
    )
    .unwrap();

    graph.insert_anchor(tx.txid(), confirmation_time_height_anchor);

    graph.insert_anchor(tx.txid(), confirmation_time_height_anchor2);

    // println!("============ Graph with anchors ============: {:#?}\n", graph);

    // println!("Chain A: {:#?}\n", chain_a);

    let block_3 = BlockId {
        height: 3,
        hash: Hash::hash("third".as_bytes()),
    };
    let block_4 = BlockId {
        height: 4,
        hash: Hash::hash("fourth".as_bytes()),
    };

    // let txs = graph.try_list_chain_txs(&chain_a, block_3);
    let txs = graph.list_chain_txs(&chain_a, block_4);
    println!("Transactions: {:#?}\n", txs.collect::<Vec<_>>());

    // println!("Missing heights: {:#?}\n", graph.missing_heights().collect());
}
