use std::sync::mpsc::Receiver;

use crate::process::node::{ProcessNode, PullNode, cache::impls::VecProcessNodeCache};

pub struct GainNode {
    rx: Receiver<f32>,
    source: Option<usize>,
    gain: f32,
    cache: VecProcessNodeCache,
}

impl GainNode {
    pub fn new(rx: Receiver<f32>, gain: f32) -> Self {
        Self {
            rx,
            source: None,
            gain,
            cache: VecProcessNodeCache::new(50),
        }
    }
}

impl ProcessNode for GainNode {
    fn cache_mut(&mut self) -> &mut dyn crate::process::node::cache::ProcessNodeCache {
        &mut self.cache
    }

    fn cache(&self) -> &dyn crate::process::node::cache::ProcessNodeCache {
        &self.cache
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut crate::process::node::cache::OutputContext<'a>,
        graph: &mut crate::process::ProcessGraph
    ) {
        if let Ok(gain) = self.rx.try_recv() {
            self.gain = gain;
        }

        let Some(source_id) = self.source else {
            output_ctx.output.fill(0.0);
            return;
        };

        let Some(mut source) = graph.0.remove(&source_id) else {
            return;
        };

        source.process(output_ctx, graph);
        graph.insert(source_id, source);

        for sample in output_ctx.output.iter_mut() {
            *sample *= self.gain;
        }
    }
}

impl PullNode for GainNode {
    fn connect(&mut self, _port: usize, node: usize) {
        self.source = Some(node);
    }
    fn disconnect(&mut self, _port: usize) {
        self.source = None;
    }
}