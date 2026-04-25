use crate::process::node::{ProcessNode, PullNode, cache::{OutputContext, impls::VecProcessNodeCache}};

pub struct MixerNode {
    cache: VecProcessNodeCache,
    inputs: [Option<usize>; 10],
}

impl MixerNode {
    pub fn new() -> Self {
        Self {
            cache: VecProcessNodeCache::new(50),
            inputs: [None; 10],
        }
    }
}

impl ProcessNode for MixerNode {
    fn cache_mut(&mut self) -> &mut dyn crate::process::node::cache::ProcessNodeCache {
        &mut self.cache
    }

    fn cache(&self) -> &dyn crate::process::node::cache::ProcessNodeCache {
        &self.cache
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        graph: &mut crate::process::ProcessGraph
    ) {
        let mut buff = vec![0_f32; output_ctx.output.len()];
        let mut buff_ctx = OutputContext::new(&mut buff, output_ctx.frame);

        output_ctx.output.fill(0.0);
        for input_id in &mut self.inputs {
            let Some(input_id) = *input_id else {
                continue;
            };

            let Some(mut input) = graph.0.remove(&input_id) else {
                continue;
            };
            
            input.process(&mut buff_ctx, graph);


            for (out, b) in output_ctx.output.iter_mut().zip(buff_ctx.output.iter()) {
                *out += *b;
            }

            // clear buffer for next iteration
            buff_ctx.output.fill(0.0);

            graph.0.insert(input_id, input);
        }
    }
}

impl PullNode for MixerNode {
    fn connect(&mut self, port: usize, node: usize) {
        self.inputs[port] = Some(node);
    }

    fn disconnect(&mut self, port: usize) {
        self.inputs[port] = None;
    }
}