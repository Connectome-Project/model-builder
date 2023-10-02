use async_trait::async_trait;
use gremlin_client::process::traversal;
use gremlin_client::GValue;
use gremlin_client::GremlinResult;
use gremlin_client::{
    aio::GResultSet, aio::GremlinClient, structure::Labels, ConnectionOptions, Vertex,
};
use tokio_stream::StreamExt;

use regex::Regex;
use std::marker::Send;
use std::{error, vec};

#[async_trait]
pub trait GraphDbConnection {
    async fn get_nodes_by_label(
        &self,
        label: impl Into<Labels> + Send,
    ) -> Result<Vec<Vertex>, Box<dyn std::error::Error>>;

    async fn get_nodes_starting_by_label(&self, label: impl Into<Labels> + Send);

    async fn get_nodes_by_label_regexp(
        &self,
        regex: Regex,
    ) -> Result<Vec<Vertex>, Box<dyn std::error::Error>>;

    async fn create_node(
        &mut self,
        node_content: impl Into<Labels> + Send,
    ) -> Result<Option<Vertex>, Box<dyn std::error::Error>>;

    async fn clear_db(&mut self) -> Result<(), Box<dyn std::error::Error>>;
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
        let g = traversal::traversal().with_remote_async(self.client.clone());

        let res = g.v(()).has_label(label).to_list().await?;

        Ok(res)
    }

    async fn get_nodes_starting_by_label(
        &self,
        label: impl Into<Labels> + Send,
    ) -> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
        let client = self.client.clone();
        let g = traversal::traversal().with_remote_async(client);
        let res = g.with_remote_async(client).v().map(g.)
        Ok(res)
    }

    async fn create_node(
        &mut self,
        node_content: impl Into<Labels> + Send,
    ) -> Result<Option<Vertex>, Box<dyn std::error::Error>> {
        let g = traversal::traversal().with_remote_async(self.client.clone());
        let res = g.add_v(node_content).next().await?;
        return Ok(res);
    }

    async fn clear_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.execute("g.V().drop()", &[]).await?;
        Ok(())
    }

    async fn get_nodes_by_label_regexp(
        &self,
        regex: Regex,
    ) -> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
        let mut res = self
            .client
            .execute(
                "g.V().hasLabel(regex(param))",
                &[("param", &regex.as_str())],
            )
            .await?
            .filter_map(Result::ok)
            .map(|f: GValue| f.take::<Vertex>())
            .collect::<Result<Vec<Vertex>, _>>()
            .await?;

        Ok(res)
    }
}
