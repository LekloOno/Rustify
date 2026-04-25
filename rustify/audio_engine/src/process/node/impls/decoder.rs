use std::sync::mpsc::Receiver;

use crate::process::node::{ProcessNode, PullNode, cache::impls::VecProcessNodeCache};

pub struct DecoderNode {
    rx: Receiver<f32>,
    cache: VecProcessNodeCache,
}

impl DecoderNode {
    pub fn new(rx: Receiver<f32>) -> Self {
        Self {
            rx,
            cache: VecProcessNodeCache::new(50),
        }
    }
}

impl ProcessNode for DecoderNode {
    fn cache_mut(&mut self) -> &mut dyn crate::process::node::cache::ProcessNodeCache {
        &mut self.cache
    }

    fn cache(&self) -> &dyn crate::process::node::cache::ProcessNodeCache {
        &self.cache
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut crate::process::node::cache::OutputContext<'a>,
        _graph: &mut crate::process::ProcessGraph
    ) {
        output_ctx.output.fill_with(||
            self.rx
                .try_recv()
                .unwrap_or(0.0)
        );
    }
}

impl PullNode for DecoderNode {
    fn connect(&mut self, _port: usize, _node: usize) {}
    fn disconnect(&mut self, _port: usize) {}
}