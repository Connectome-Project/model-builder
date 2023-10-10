use async_trait::async_trait;

use neo4rs::{query, Graph};
use std::sync::Arc;
use std::{error, vec};

pub struct Node {
    pub pattern: String,
}

#[async_trait]
pub trait GraphDbConnection {
    async fn create_node(&self, pattern: &str) -> Result<Node, Box<dyn std::error::Error>>;

    async fn clear_db(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_nodes_by_exact_pattern_match(
        &self,
        label: &str,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>>;

    async fn get_nodes_by_matching_pattern_against_input(
        &self,
        input: &str,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>>;
}

pub struct ConnectionType {
    client: Arc<Graph>,
}

impl ConnectionType {
    pub async fn create(
        uri: impl Into<String>,
        user: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self, Box<dyn error::Error>> {
        let client = Arc::new(Graph::new(uri, user, password).await.unwrap());

        Ok(ConnectionType { client })
    }

    pub fn clone(&self) -> Self {
        ConnectionType {
            client: self.client.clone(),
        }
    }
}

#[async_trait]
impl GraphDbConnection for ConnectionType {
    async fn get_nodes_by_exact_pattern_match(
        &self,
        pattern: &str,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        let mut res = self
            .client
            .execute(query("MATCH (p {pattern:{$name}}) return p").param("name", pattern))
            .await?;

        let mut result: Vec<Node> = vec![];
        while let Ok(Some(row)) = res.next().await {
            let p = row.get::<String>("p").unwrap();
            result.push(Node { pattern: p });
        }
        Ok(result)
    }

    async fn create_node(&self, pattern: &str) -> Result<Node, Box<dyn std::error::Error>> {
        let mut res = self
            .client
            .execute(query("MERGE (p {pattern:{$name}}) return p").param("name", pattern))
            .await?;
        let res = res.next().await.unwrap().unwrap();
        let p = res.get::<String>("p").unwrap();
        let node = Node { pattern: p };
        Ok(node)
    }

    async fn clear_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .execute(query("MATCH (n) DETACH DELETE n;"))
            .await?;
        Ok(())
    }

    async fn get_nodes_by_matching_pattern_against_input(
        &self,
        input: &str,
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        let mut res = self
            .client
            .execute(query("MATCH (n) WHERE $input=~n.pattern RETURN n").param("input", input))
            .await?;

        let mut result: Vec<Node> = vec![];
        while let Ok(Some(row)) = res.next().await {
            let p = row.get::<String>("pattern").unwrap();
            result.push(Node { pattern: p });
        }
        Ok(result)
    }
}
