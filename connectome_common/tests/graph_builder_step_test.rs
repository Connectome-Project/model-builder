// extern crate connectome_common;

use std::{
    fs::File,
    io::{BufReader, Lines},
    path::PathBuf,
    sync::mpsc::channel,
    thread,
};

use connectome_common::{
    assemble_relative_path, read_lines, GraphBuilder, GraphBuilderTrait, GraphChangeRequest,
    ModelBuilder, ModelBuilderTrait, ModelBuilderType, ThreadSafeModel, TrainingConfig,
};

#[test]
fn test_graph_builder_step() {
    let (sender, receiver) = channel::<GraphChangeRequest<String, usize>>();
    let model: ThreadSafeModel<String, String, usize> =
        ThreadSafeModel::<String, String, usize>::new();

    //create graph builder
    let mut graph_builder =
        GraphBuilder::<String, usize, ThreadSafeModel<String, String, usize>>::new(model.clone());
    graph_builder.set_channel(Some(receiver));

    let graph_builder_handle = thread::spawn(move || {
        graph_builder.build_graph_from_channel();
    });

    //create model builder
    let combined_path: PathBuf = assemble_relative_path("tests\\assets\\example.txt");
    println!("\n\n {} \n\n", combined_path.to_str().unwrap());
    let lines: Lines<BufReader<File>> = read_lines(combined_path).unwrap();
    let config = TrainingConfig {};
    let mut model_builder = ModelBuilder::new(
        ModelBuilderType::Builder,
        config,
        model.clone(),
        lines,
        sender.clone(),
    );

    let model_builder_handle = thread::spawn(move || {
        model_builder.perform_action();
    });

    model_builder_handle.join().unwrap();
    drop(sender);

    graph_builder_handle.join().unwrap();

    let reader = model.data.read().unwrap();

    let edge_count = reader.get_data().edge_count();
    let node_count = reader.get_data().node_count();

    assert_eq!(edge_count, 6);
    assert_eq!(node_count, 8);
}
