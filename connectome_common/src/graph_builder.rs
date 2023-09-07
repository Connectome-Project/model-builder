mod graph_builder_trait;

use crate::{arc_model::graph_change_request::GraphChangeRequest, pattern::PatternTrait};
pub use graph_builder_trait::GraphBuilderTrait;
use petgraph::stable_graph::IndexType;
use std::{fmt::Debug, fmt::Display, sync::mpsc::Receiver};

struct GraphBuilder<PatternContent, Ix, Mod>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + IndexType,
{
    pub channel: Option<Receiver<GraphChangeRequest<PatternContent, Ix>>>,
    model: Mod,
}

impl<PatternContent, Ix, Mod> GraphBuilder<PatternContent, Ix, Mod>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + IndexType,
{
    fn new(model: Mod) -> Self {
        GraphBuilder {
            channel: None,
            model,
        }
    }

    fn set_channel(&mut self, chan: Option<Receiver<GraphChangeRequest<PatternContent, Ix>>>) {
        self.channel = chan;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::thread;

    use petgraph::stable_graph::EdgeIndex;
    use petgraph::stable_graph::NodeIndex;

    use super::GraphBuilder;
    use super::GraphBuilderTrait;
    use crate::arc_model::connection::Connection;
    use crate::arc_model::connection::ConnectionInfo;
    use crate::arc_model::graph_change_request::NodeOrIdx;
    use crate::arc_model::{graph_change_request::GraphChangeRequest, ThreadSafeModel};
    use crate::arc_model::{Node, NodeType};

    #[test]
    fn test_graph_builder() -> () {
        let (send, receiver) = channel::<GraphChangeRequest<String, usize>>();
        let model: ThreadSafeModel<String, String, usize> =
            ThreadSafeModel::<String, String, usize>::new();
        let node1 = Node::new("Some".to_string(), NodeType::Generated);
        let node2 = Node::new("Other".to_string(), NodeType::Generated);
        let node1_clone = node1.clone();
        let node2_clone = node2.clone();
        let new_node1 = GraphChangeRequest::AddNode(node1);
        let new_node2 = GraphChangeRequest::AddNode(node2);

        //prep connection
        let connection = Connection::build_from_content(ConnectionInfo {
            label: "Label".to_string(),
        });
        let connection_clone = connection.clone();
        let from_node = NodeOrIdx::<String, usize>::Pattern(node1_clone.pattern.clone());
        let to_node = NodeOrIdx::Pattern(node2_clone.pattern.clone());
        let new_connection = GraphChangeRequest::AddConnection {
            from_node,
            to_node,
            connection: connection,
        };

        let _ = send.send(new_node1);
        let _ = send.send(new_node2);
        let _ = send.send(new_connection);

        let mut builder: GraphBuilder<_, _, ThreadSafeModel<String, String, usize>> =
            GraphBuilder::new(model.clone());
        builder.set_channel(Some(receiver));

        let thread_handle = thread::spawn(move || {
            builder.build_graph_from_channel();
        });

        drop(send); //kills the build thread as it will no longer wait for anything if one end of the channel is destroyed

        let err = thread_handle.join();
        if let Err(e) = err {
            panic!("{:?}", e);
        }

        let read: std::sync::RwLockReadGuard<
            '_,
            crate::arc_model::model::Model<String, String, usize>,
        > = model.data.read().unwrap();
        let inner_model = read.get_data();

        let first = inner_model.node_weight(NodeIndex::new(0)).unwrap();
        let second = inner_model.node_weight(NodeIndex::new(1)).unwrap();
        let res_connection = inner_model.edge_weight(EdgeIndex::new(0)).unwrap();

        assert_eq!(first.pattern, node1_clone.pattern);
        assert_eq!(second.pattern, node2_clone.pattern);
        assert_eq!(connection_clone, res_connection.clone())
    }
}
