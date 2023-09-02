use crate::{arc_model::graph_change_request::GraphChangeRequest, pattern::PatternTrait};
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

    fn build_graph(&mut self) {
        // match self.channel.recv() {
        //     Ok(val) => {
        //         // match val{
        //         //   GraphChangeRequest::AddNode(node)=>{

        //         //   }
        //         //   }
        //         // }
        //     }
        //     Err(e) => {
        //         println!("{}", e);
        //     }
        // }
    }
}
