pub mod ethereum_helper;

use cnd::{
    btsieve::ethereum::{matching_transaction, TransactionPattern},
    ethereum::{Block, Transaction, TransactionAndReceipt, TransactionReceipt},
};
use ethereum_helper::EthereumConnectorMock;
use futures_core::{FutureExt, TryFutureExt};
use tokio::prelude::Future;

#[test]
fn find_transaction_in_old_block() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let block1_with_transaction: Block<Transaction> = include_json_test_data!(
        "./test_data/ethereum/find_transaction_in_old_block/block1_with_transaction.json"
    );
    let transaction: Transaction = include_json_test_data!(
        "./test_data/ethereum/find_transaction_in_old_block/transaction.json"
    );
    let receipt: TransactionReceipt =
        include_json_test_data!("./test_data/ethereum/find_transaction_in_old_block/receipt.json");
    let connector = EthereumConnectorMock::new(
        vec![
            include_json_test_data!(
                "./test_data/ethereum/find_transaction_in_old_block/block4.json"
            ),
            include_json_test_data!(
                "./test_data/ethereum/find_transaction_in_old_block/block5.json"
            ),
        ],
        vec![
            block1_with_transaction.clone(),
            include_json_test_data!(
                "./test_data/ethereum/find_transaction_in_old_block/block2.json"
            ),
            include_json_test_data!(
                "./test_data/ethereum/find_transaction_in_old_block/block3.json"
            ),
            include_json_test_data!(
                "./test_data/ethereum/find_transaction_in_old_block/block4.json"
            ),
            include_json_test_data!(
                "./test_data/ethereum/find_transaction_in_old_block/block5.json"
            ),
        ],
        vec![(transaction.hash, receipt.clone())],
        runtime.executor(),
    );

    let expected_transaction_and_receipt: TransactionAndReceipt = async {
        matching_transaction(
            connector,
            TransactionPattern {
                from_address: None,
                to_address: Some(transaction.to.unwrap()),
                is_contract_creation: None,
                transaction_data: None,
                transaction_data_length: None,
                events: None,
            },
            Some(block1_with_transaction.timestamp.low_u32()),
        )
        .await
    }
        .unit_error()
        .boxed()
        .compat()
        .wait()
        .unwrap();

    assert_eq!(expected_transaction_and_receipt, TransactionAndReceipt {
        transaction,
        receipt
    });
}
