mod graph_db_connection;
use graph_db_connection::GremlinConnectionType;
use std::error;

use gremlin_client::{
    aio::GremlinClient, process::traversal::traversal, ConnectionOptions, Vertex,
};

use crate::graph_db_connection::GraphDbConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");

    // let client = GremlinClient::connect("localhost").await?;
    // let g = traversal().with_remote_async(client);

    // let res = g.v(()).to_list().await?;

    // println!("Res: {:?}", res);
    // Ok(())

    let mut sd = GremlinConnectionType::create("localhost").await?;
    sd.create_node("hello".to_string()).await?;
    sd.create_node("te").await?;
    Ok(())
}
