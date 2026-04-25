use crate::engine::process::node::{ProcessNode, PullNode, cache::{OutputContext, ProcessNodeCache, impls::EmptyProcessNodeCache}};

pub struct Sink {
    cache: EmptyProcessNodeCache,
}

impl Sink {
    pub fn new() -> Self {
        Self {
            cache: EmptyProcessNodeCache {},
        }
    }
}

impl ProcessNode for Sink {
    fn cache_mut(&mut self) -> &mut dyn ProcessNodeCache {
        &mut self.cache
    }

    fn cache(&self) -> &dyn ProcessNodeCache {
        &self.cache
    }

    fn process_new<'a>(&mut self, _output_ctx: &mut OutputContext<'a>) {}
}

impl PullNode for Sink {
    fn connect(&mut self, port: usize, node_id: usize) {}
    fn disconnect(&mut self, port: usize) {}
}