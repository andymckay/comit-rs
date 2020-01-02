use crate::{
    btsieve::{BlockByHash, LatestBlock, ReceiptByHash},
    ethereum::{
        web3::{
            self,
            transports::{EventLoopHandle, Http},
            Web3,
        },
        BlockId, BlockNumber,
    },
};
use futures::Future;
use futures_core::compat::Future01CompatExt;
use reqwest::Url;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Web3Connector {
    web3: Arc<Web3<Http>>,
    task_executor: tokio::runtime::TaskExecutor,
}

impl Web3Connector {
    pub fn new(
        node_url: Url,
        task_executor: tokio::runtime::TaskExecutor,
    ) -> Result<(Self, EventLoopHandle), web3::Error> {
        let (event_loop_handle, http_transport) = Http::new(node_url.as_str())?;
        Ok((
            Self {
                web3: Arc::new(Web3::new(http_transport)),
                task_executor,
            },
            event_loop_handle,
        ))
    }
}

#[async_trait::async_trait]
impl LatestBlock for Web3Connector {
    type Error = crate::ethereum::web3::Error;
    type Block = Option<crate::ethereum::Block<crate::ethereum::Transaction>>;
    type BlockHash = crate::ethereum::H256;

    async fn latest_block(&mut self) -> Result<Self::Block, Self::Error> {
        let web = self.web3.clone();

        web.eth()
            .block_with_txs(BlockId::Number(BlockNumber::Latest))
            .compat()
            .await
    }
}

#[async_trait::async_trait]
impl BlockByHash for Web3Connector {
    type Error = crate::ethereum::web3::Error;
    type Block = Option<crate::ethereum::Block<crate::ethereum::Transaction>>;
    type BlockHash = crate::ethereum::H256;

    async fn block_by_hash(&self, block_hash: Self::BlockHash) -> Result<Self::Block, Self::Error> {
        let web = self.web3.clone();
        web.eth()
            .block_with_txs(BlockId::Hash(block_hash))
            .compat()
            .await
    }
}

#[async_trait::async_trait]
impl ReceiptByHash for Web3Connector {
    type Receipt = Option<crate::ethereum::TransactionReceipt>;
    type TransactionHash = crate::ethereum::H256;
    type Error = crate::ethereum::web3::Error;

    async fn receipt_by_hash(
        &self,
        transaction_hash: Self::TransactionHash,
    ) -> Result<Self::Receipt, Self::Error> {
        let web = self.web3.clone();
        web.eth()
            .transaction_receipt(transaction_hash)
            .compat()
            .await
    }
}

impl tokio::executor::Executor for Web3Connector {
    fn spawn(
        &mut self,
        future: Box<dyn Future<Item = (), Error = ()> + Send>,
    ) -> Result<(), tokio::executor::SpawnError> {
        tokio::executor::Executor::spawn(&mut self.task_executor, future)
    }
}
