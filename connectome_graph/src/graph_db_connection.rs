use async_trait::async_trait;
use gremlin_client::{
    aio::GremlinClient, process::traversal::traversal, structure::Labels, ConnectionOptions, Vertex,
};
use std::error;
use std::marker::Send;

#[async_trait]
pub trait GraphDbConnection {
    async fn get_nodes_by_label(
        &self,
        label: impl Into<Labels> + Send,
    ) -> Result<Vec<Vertex>, Box<dyn std::error::Error>>;
    async fn create_node(
        &mut self,
        node_content: impl Into<Labels> + Send,
    ) -> Result<Option<Vertex>, Box<dyn std::error::Error>>;
}

pub struct GremlinConnectionType {
    client: GremlinClient,
}

impl GremlinConnectionType {
    pub async fn create<T>(options: T) -> Result<Self, Box<dyn error::Error>>
    where
        T: Into<ConnectionOptions>,
    {
        let client = GremlinClient::connect(options).await?;
        Ok(GremlinConnectionType { client })
    }

    pub fn clone(&self) -> Self {
        GremlinConnectionType {
            client: self.client.clone(),
        }
    }
}

#[async_trait]
impl GraphDbConnection for GremlinConnectionType {
    async fn get_nodes_by_label(
        &self,
        label: impl Into<Labels> + Send,
    ) -> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
        let g = traversal().with_remote_async(self.client.clone());

        let res = g.v(()).has_label(label).to_list().await?;

        Ok(res)
    }

    async fn create_node(
        &mut self,
        node_content: impl Into<Labels> + Send,
    ) -> Result<Option<Vertex>, Box<dyn std::error::Error>> {
        let g = traversal().with_remote_async(self.client.clone());
        let res = g.add_v(node_content).next().await?;
        return Ok(res);
    }
}
