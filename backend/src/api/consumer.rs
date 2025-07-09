use super::{
    database::{convert_cdc_item, get_cdc_id},
    meilisearch::{add_items, delete_item},
};
use async_trait::async_trait;
use meilisearch_sdk::client::Client;
use scylla_cdc::consumer::{CDCRow, Consumer, ConsumerFactory, OperationType};
use std::sync::Arc;

pub struct MeiliConsumer {
    pub meili_client: Arc<Client>,
    pub meili_index: String,
    pub scylla_id_name: String,
}

impl MeiliConsumer {
    pub async fn new(
        meili_client: Arc<Client>,
        meili_index: String,
        scylla_id_name: String,
    ) -> Self {
        Self {
            meili_client,
            meili_index,
            scylla_id_name,
        }
    }
}

#[async_trait]
impl Consumer for MeiliConsumer {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        match data.operation {
            OperationType::RowInsert => {
                add_items(
                    self.meili_client.clone(),
                    &self.meili_index,
                    &[convert_cdc_item(data)],
                    &self.scylla_id_name,
                )
                .await?;
            }
            OperationType::RowDelete
            | OperationType::PartitionDelete
            | OperationType::RowRangeDelInclLeft
            | OperationType::RowRangeDelExclLeft
            | OperationType::RowRangeDelInclRight
            | OperationType::RowRangeDelExclRight => {
                delete_item(
                    self.meili_client.clone(),
                    &self.meili_index,
                    get_cdc_id(&data, &self.scylla_id_name),
                )
                .await?;
            }
            _ => {}
        }
        Ok(())
    }
}

pub struct MeiliConsumerFactory {
    pub meili_client: Arc<Client>,
    pub meili_index: String,
    pub scylla_id_name: String,
}

#[async_trait]
impl ConsumerFactory for MeiliConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(
            MeiliConsumer::new(
                self.meili_client.clone(),
                self.meili_index.clone(),
                self.scylla_id_name.clone(),
            )
            .await,
        )
    }
}
