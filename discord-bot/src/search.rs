use anyhow::Result;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::Index;

pub async fn create_search_client(endpoint: String) -> Result<Index> {
    let index = Client::new(endpoint, "masterKey").index("resources");
    index.set_filterable_attributes(["group_id", "subject", "tags"]).await.unwrap();

    Ok(index)
}
