mod graph_db_connection;
mod input;
mod model_builder;
mod pattern;

use core::num;
use graph_db_connection::GremlinConnectionType;
use gremlin_client::{
    aio::GremlinClient, process::traversal::traversal, ConnectionOptions, Vertex,
};
use num_cpus;
use std::error;

use crate::graph_db_connection::GraphDbConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");

    let num_cores = num_cpus::get();
    println!("Number of logical cpu cores: {num_cores}");
    let mut connection_options = ConnectionOptions::builder()
        .host("localhost")
        .port(8182)
        .pool_size(num_cores as u32)
        .build();

    let mut gremlin_client = GremlinConnectionType::create(connection_options).await?;
    gremlin_client.clear_db().await?;
    gremlin_client.create_node("hello".to_string()).await?;
    gremlin_client.create_node("te").await?;

    let mut handles = Vec::new();

    for i in 0..num_cores {
        let mut client = gremlin_client.clone();
        let node_pattern = format!("{}", i);
        let handle = tokio::spawn(async move {
            let new_node = client.create_node(node_pattern.clone()).await;
            return 0;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
