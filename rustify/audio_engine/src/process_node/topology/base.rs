use std::sync::atomic::{AtomicUsize, Ordering};

use crate::process_node::topology::graph::{ProcessGraph, cache::{OutputContext, ProcessGraphCache}};

// Bad practice, will fix later
static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Defines a process Node.
/// 
/// A Node that only implements process node is essentially a leaf node in the pipeline graph.
/// That would typically an input source like a decoder or a generator.
/// 
/// The caching behavior is made an external mutation, as multiple threads
/// could concurrently build such caches independantly.
/// 
/// These caches are specific to some stream frames, not to the general graph,
/// and its state should thus be held by such streams.
/// 
/// Yet, this lead to the question of synchronisation. As is, it would be quite
/// unatural to share cached data accross streams. Multiple streams would always
/// re-pull data across the data flow.
/// We do not explicitly support such scenario where multiple streams want to synchronously pull
/// data and partially share it. It might be externalised later.
pub trait ProcessNode : Send {
    fn process<'a>(
        &mut self, 
        output_ctx: &mut OutputContext<'a>, 
        graph_cache: &mut dyn ProcessGraphCache,
        graph: &mut ProcessGraph,
    ) {
        if !graph_cache.copy(self.id(), output_ctx) {
            self.process_new(output_ctx, graph_cache, graph);
            graph_cache.cache(self.id(), output_ctx);
        }
    }

    /// Bad practice, will fix later
    fn id(&self) -> usize;

    fn next_id() -> usize where Self: Sized {
        GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        graph_cache: &mut dyn ProcessGraphCache,
        graph: &mut ProcessGraph,
    );
}

/// Defines a node that can pull from other pipeline nodes.
/// That is, a node with input sources.
/// 
/// A node that only implements pull node is essentially a root node in the pipeline graph.
/// That would typically be a sound card output.
pub trait PullNode {
    /// `port`
    /// # Arguments
    fn connect(&mut self, port: usize, node_id: usize);
    fn disconnect(&mut self, port: usize);
}

/// An alias for internal nodes that processe audio from prior nodes input sources.
pub trait PullProcessNode : PullNode + ProcessNode {}
impl<T: PullNode + ProcessNode> PullProcessNode for T {}