use crate::process_node::topology::{base::ProcessNode, graph::{ProcessGraph, cache::{OutputContext, ProcessGraphCache}}};

pub struct Mixer {
    id: usize,
    /// Not ideal.. Ids referencing for now
    inputs: Vec<usize>,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            id: Self::next_id(),
            inputs: vec![],
        }
    }
}

impl ProcessNode for Mixer {
    fn id(&self) -> usize {
        todo!()
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        graph_cache: &mut dyn ProcessGraphCache,
        graph: &mut ProcessGraph
    ) {
        let mut buff = vec![0_f32; output_ctx.output.len()];
        let mut buff_ctx = OutputContext::new(
            &mut buff,
            output_ctx.frame
        );

        for input in &self.inputs {
            let Some(input_node) = graph.get_node(*input) else {
                continue;
            };

            input_node.process(&mut buff_ctx, graph_cache, graph);
        }
    }
}