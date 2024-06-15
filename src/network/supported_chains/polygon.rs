use anyhow::Result;
use async_trait::async_trait;

use crate::network::{config::NetworkConfig, utils::subgraph_query_block_from_timestamp};

pub struct PolygonNetwork {
    client: reqwest::Client,
}

impl PolygonNetwork {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NetworkConfig for PolygonNetwork {
    fn get_subgraph_url(&self) -> String {
        return String::from("https://api.thegraph.com/subgraphs/name/ord786/matic-blocks");
    }

    async fn get_block_from_timestamp(&self, timestamp: u64) -> Result<u64> {
        return subgraph_query_block_from_timestamp(
            &self.client,
            self.get_subgraph_url(),
            timestamp,
        )
        .await;
    }

    fn get_latest_block(&self) {
        todo!()
    }
}