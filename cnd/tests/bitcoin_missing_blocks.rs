pub mod bitcoin_helper;

use bitcoin::Address;
use bitcoin_helper::BitcoinConnectorMock;
use chrono::offset::Utc;
use cnd::{
    btsieve::{bitcoin::TransactionPattern, MatchingTransactions},
    first_or_else::StreamExt,
};
use std::str::FromStr;
use tokio::prelude::Future;

#[test]
fn find_transaction_in_missing_block() {
    let connector = BitcoinConnectorMock::new(
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block/block1.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block/block3.hex"),
        ],
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block/block1.hex"),
            include_hex!(
                "./test_data/bitcoin/find_transaction_in_missing_block/block2_with_transaction.hex"
            ),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block/block3.hex"),
        ],
    );

    let expected_transaction: bitcoin::Transaction = connector
        .matching_transactions(
            TransactionPattern {
                to_address: Some(
                    Address::from_str(
                        include_str!("test_data/bitcoin/find_transaction_in_missing_block/address")
                            .trim(),
                    )
                    .unwrap(),
                ),
                from_outpoint: None,
                unlock_script: None,
            },
            Utc::now().naive_local(),
        )
        .first_or_else(|| panic!())
        .wait()
        .unwrap();

    assert_eq!(
        expected_transaction,
        include_hex!("./test_data/bitcoin/find_transaction_in_missing_block/transaction.hex")
    );
}

#[test]
fn find_transaction_in_missing_block_with_big_gap() {
    let connector = BitcoinConnectorMock::new(
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block1.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block8.hex"),
        ],
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block1.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block2_with_transaction.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block3.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block4.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block5.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block6.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block7.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/block8.hex"),
        ],
    );

    let expected_transaction: bitcoin::Transaction = connector
        .matching_transactions(
            TransactionPattern {
                to_address: Some(
                    Address::from_str(
                        include_str!(
                            "test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/address"
                    )
                        .trim(),
                    )
                    .unwrap(),
                ),
                from_outpoint: None,
                unlock_script: None,
            },
            Utc::now().naive_local(),
        )
        .first_or_else(|| panic!())
        .wait()
        .unwrap();

    assert_eq!(
        expected_transaction,
        include_hex!(
            "./test_data/bitcoin/find_transaction_in_missing_block_with_big_gap/transaction.hex"
        )
    );
}

#[test]
fn find_transaction_if_blockchain_reorganisation() {
    let connector = BitcoinConnectorMock::new(
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/block1.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/block1b_stale.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/block2_with_transaction.hex"),
        ],
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/block1.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/block2_with_transaction.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/block1b_stale.hex"),
        ],
    );

    let expected_transaction: bitcoin::Transaction = connector
        .matching_transactions(
            TransactionPattern {
                to_address: Some(
                    Address::from_str(
                        include_str!(
                            "test_data/bitcoin/find_transaction_if_blockchain_reorganisation/address"
                    )
                        .trim(),
                    )
                    .unwrap(),
                ),
                from_outpoint: None,
                unlock_script: None,
            },
            Utc::now().naive_local(),
        )
        .first_or_else(|| panic!())
        .wait()
        .unwrap();

    assert_eq!(
        expected_transaction,
        include_hex!(
            "./test_data/bitcoin/find_transaction_if_blockchain_reorganisation/transaction.hex"
        )
    );
}

#[test]
fn find_transaction_if_blockchain_reorganisation_with_long_chain() {
    let connector = BitcoinConnectorMock::new(
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block4.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block4b_stale.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block5_with_transaction.hex")
        ],
        vec![
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block1.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block2.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block3.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block4.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block5_with_transaction.hex"),
            include_hex!("./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/block4b_stale.hex"),
        ],
    );

    let expected_transaction: bitcoin::Transaction = connector
        .matching_transactions(TransactionPattern {
            to_address: Some(
                Address::from_str(
                    include_str!(
                        "test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/address"
                    ).trim()
                    ,
                )
                .unwrap(),
            ),
            from_outpoint: None,
            unlock_script: None,
        },
                               Utc::now().naive_local(),
        )
        .first_or_else(|| panic!())
        .wait()
        .unwrap();

    assert_eq!(
        expected_transaction,
        include_hex!(
        "./test_data/bitcoin/find_transaction_if_blockchain_reorganisation_with_long_chain/transaction.hex"
    )
    );
}
