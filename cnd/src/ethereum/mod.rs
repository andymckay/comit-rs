#![warn(unused_extern_crates, missing_debug_implementations, rust_2018_idioms)]
#![forbid(unsafe_code)]

pub use self::{
    contract_address::*, erc20_quantity::*, erc20_token::*, ether_quantity::*, u256_ext::*,
};
pub use ::web3::types::{
    Address, Block, BlockId, BlockNumber, Bytes, Log, Transaction, TransactionReceipt,
    TransactionRequest, H160, H2048, H256, U128, U256,
};

pub mod web3 {
    pub use ::web3::{transports, Error, Web3};
}

mod contract_address;
mod erc20_quantity;
mod erc20_token;
mod ether_quantity;
mod u256_ext;

#[derive(Debug, PartialEq)]
pub struct TransactionAndReceipt {
    pub transaction: Transaction,
    pub receipt: TransactionReceipt,
}
