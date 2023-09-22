use std::error;

use gremlin_client::{aio::GremlinClient, process::traversal::traversal, Vertex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");

    let client = GremlinClient::connect("localhost").await?;

    let g = traversal().with_remote_async(client);

    let res = g.v(()).to_list().await?;

    println!("Res: {:?}", res);
    Ok(())
}
