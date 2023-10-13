mod graph_db_connection;
mod input;
mod model_builder;
mod pattern;
mod read_file;
use std::{error, sync::Arc};

use crate::{
    graph_db_connection::GraphDbConnection,
    read_file::{assemble_relative_path, read_lines},
};
use graph_db_connection::ConnectionType;
use num_cpus;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");

    let num_cores = num_cpus::get();
    println!("Number of logical cpu cores: {num_cores}");

    let mut client = ConnectionType::create("http://localhost:7687", "", "").await?;
    let file_path = assemble_relative_path("../files_to_read/first.txt");
    let file = read_lines(file_path).unwrap();

    client.clear_db().await?;
    client.create_node("hello").await?;
    client.create_node("te").await?;

    let mut handles = Vec::new();
    let semaphore = Arc::new(Semaphore::new(num_cores.clone()));

    for optional_line in file {
        if let Ok(line) = optional_line {
            let permit = semaphore.clone().acquire_owned().await?;
            let chars_to_build: std::iter::Peekable<std::vec::IntoIter<String>> = line
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .peekable();

            let local_client = client.clone();
            let handle = tokio::spawn(async move {
                let _res = model_builder::build_model(local_client, chars_to_build).await;
                // let _ = client.create_node(&chars_to_build).await;
                drop(permit);
                // return 0;
            });
            handles.push(handle);
        }
    }

    Ok(())
}
