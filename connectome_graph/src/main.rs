mod graph_db_connection;
mod input;
mod model_builder;
mod pattern;

use graph_db_connection::ConnectionType;

use num_cpus;
use std::error;

use crate::graph_db_connection::GraphDbConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");

    let num_cores = num_cpus::get();
    println!("Number of logical cpu cores: {num_cores}");

    let mut gremlin_client = ConnectionType::create("http://localhost:7687", "", "").await?;
    gremlin_client.clear_db().await?;
    gremlin_client.create_node("hello").await?;
    gremlin_client.create_node("te").await?;

    let mut handles = Vec::new();

    for i in 0..num_cores {
        let client = gremlin_client.clone();
        let node_pattern = format!("{}", i);
        let handle = tokio::spawn(async move {
            let _ = client.create_node(&node_pattern).await;
            return 0;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
