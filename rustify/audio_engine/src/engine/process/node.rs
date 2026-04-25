use crate::engine::process::{ProcessGraph, node::cache::{OutputContext, ProcessNodeCache}};

pub mod cache;
pub mod impls;

pub trait ProcessNode : Send {
    fn cache_mut(&mut self) -> &mut dyn ProcessNodeCache;
    fn cache(&self) -> &dyn ProcessNodeCache;

    fn process<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        nodes: &mut ProcessGraph
    ) {
        if !self.cache().copy(output_ctx) {
            self.process_new(output_ctx, nodes);
            self.cache_mut().cache(output_ctx);
        }
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        graph: &mut ProcessGraph
    );
}

/// Defines a node that can pull from other pipeline nodes.
/// That is, a node with input sources.
/// 
/// A node that only implements pull node is essentially a root node in the pipeline graph.
/// That would typically be a sound card output.
pub trait PullNode {
    // Connection by id to avoid using interior mutability
    fn connect(&mut self, port: usize, node: usize);
    fn disconnect(&mut self, port: usize);
}

/// An alias for internal nodes that processe audio from prior nodes input sources.
pub trait PullProcessNode : PullNode + ProcessNode {}
impl<T: PullNode + ProcessNode> PullProcessNode for T {}