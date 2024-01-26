#![allow(dead_code)]
#![allow(unused_must_use)]

use bdk::chain::bitcoin::hashes::Hash;
use bdk::chain::bitcoin::{BlockHash, Transaction};
use bdk::chain::example_utils::tx_from_hex;
use bdk::chain::local_chain::{LocalChain, Update};
use bdk::chain::local_chain::CheckPoint;
use bdk::chain::{BlockId, ConfirmationTimeHeightAnchor, TxGraph};

use bdk::bitcoin::Network;
use bdk::Wallet;
use std::collections::BTreeMap;

fn main() -> () {
    // checkpoints();
    // local_chain();
    // anchors();
    // updates();
    changesets();
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

fn local_chain() -> () {
    let local_chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
            (12, Hash::hash("twelve".as_bytes())),
        ]
            .into_iter()
            .collect::<BTreeMap<u32, BlockHash>>(),
    ).unwrap();

    println!("###     Local chain     ### \n{:#?}\n", local_chain);
}

fn anchors() -> () {
    pub const RAW_TX_1: &str = "0200000000010116d6174da7183d70d0a7d4dc314d517a7d135db79ad63515028b293a76f4f9d10000000000feffffff023a21fc8350060000160014531c405e1881ef192294b8813631e258bf98ea7a1027000000000000225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c024730440220591b1a172a122da49ba79a3e79f98aaa03fd7a372f9760da18890b6a327e6010022013e82319231da6c99abf8123d7c07e13cf9bd8d76e113e18dc452e5024db156d012102318a2d558b2936c52e320decd6d92a88d7f530be91b6fe0af5caf41661e77da3ef2e0100";

    let mut graph = TxGraph::<ConfirmationTimeHeightAnchor>::default();
    let tx: Transaction = tx_from_hex(RAW_TX_1);
    println!("### --- Graph 0 --- ### \n{:#?}\n", graph);

    let confirmation_time_height_anchor4 = ConfirmationTimeHeightAnchor {
        anchor_block: BlockId {
            height: 4,
            hash: Hash::hash("fourth".as_bytes()),
        },
        confirmation_height: 2,
        confirmation_time: 123,
    };
    println!("### --- ConfirmationTimeHeightAnchor --- ### \n{:#?}\n", confirmation_time_height_anchor4);

    let confirmation_time_height_anchor2 = ConfirmationTimeHeightAnchor {
        anchor_block: BlockId {
            height: 2,
            hash: Hash::hash("second".as_bytes()),
        },
        confirmation_height: 2, // TODO: Why is this allowed to be different than the height above?
        confirmation_time: 100,
    };

    graph.insert_tx(tx.clone());

    println!("### --- Graph without anchors --- ### \n{:#?}\n", graph);

    let chain_a = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
        ]
            .into_iter()
            .collect::<BTreeMap<u32, BlockHash>>(),
    ).unwrap();

    graph.insert_anchor(
        tx.txid(),
        confirmation_time_height_anchor4
    );

    graph.insert_anchor(
        tx.txid(),
        confirmation_time_height_anchor2
    );

    println!(
        "################  Graph with anchors  ######################\n{:#?}\n",
        graph
    );

    println!(
        "################  Chain A  #################################\n{:#?}\n",
        chain_a
    );

    let block_3 = BlockId {
        height: 3,
        hash: Hash::hash("third".as_bytes()),
    };

    let txs = graph.try_list_chain_txs(&chain_a, block_3);
    println!("################  Transactions  ############################\n{:#?}\n", txs.collect::<Vec<_>>());
}

fn updates() -> () {
    let mut chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
        ]
        .into_iter()
        .collect::<BTreeMap<u32, BlockHash>>(),
    ).unwrap();

    let other_chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
            (5, Hash::hash("fifth".as_bytes())),
        ]
        .into_iter()
        .collect::<BTreeMap<u32, BlockHash>>(),
    ).unwrap();

    let update = Update {
        tip: other_chain.tip(),
        introduce_older_blocks: true,
    };

    println!("################  Chain before update  #####################\n{:#?}\n", chain);
    let changeset = chain.apply_update(update);
    println!("################  Chain after update  #####################\n{:#?}\n", chain);
}

fn changesets() -> () {
    let mut chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (1, Hash::hash("first".as_bytes())),
            (2, Hash::hash("second".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
        ]
            .into_iter()
            .collect::<BTreeMap<u32, BlockHash>>(),
    ).unwrap();

    let other_chain = LocalChain::from_blocks(
        [
            (0, Hash::hash("zero".as_bytes())),
            (3, Hash::hash("third".as_bytes())),
            (5, Hash::hash("fifth".as_bytes())),
        ]
            .into_iter()
            .collect::<BTreeMap<u32, BlockHash>>(),
    ).unwrap();

    let update = Update {
        tip: other_chain.tip(),
        introduce_older_blocks: true,
    };

    println!("################  Chain before update  #####################\n{:#?}\n", chain);
    let changeset = chain.apply_update(update);
    println!("################  Chain after update  #####################\n{:#?}\n", chain);
    println!("################  Changeset  ##############################\n{:#?}\n", changeset);
}
