use std::sync::mpsc::Receiver;

use crate::process::{ProcessGraph, node::{ProcessNode, PullNode, cache::{OutputContext, ProcessNodeCache, impls::VecProcessNodeCache}}};

pub struct PanningNode {
    rx: Receiver<f32>,
    source: Option<usize>,
    pan: f32,
    cache: VecProcessNodeCache,
}

impl PanningNode {
    pub fn new(rx: Receiver<f32>, pan: f32) -> Self {
        Self {
            rx,
            source: None,
            pan,
            cache: VecProcessNodeCache::new(50),
        }
    }
}

impl ProcessNode for PanningNode {
    fn cache_mut(&mut self) -> &mut dyn ProcessNodeCache {
        &mut self.cache
    }

    fn cache(&self) -> &dyn ProcessNodeCache {
        &self.cache
    }

    fn process_new<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        graph: &mut ProcessGraph
    ) {
        if let Ok(p) = self.rx.try_recv() {
            self.pan = p.clamp(0.0, 1.0);
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

        let angle = self.pan * std::f32::consts::FRAC_PI_2;
        let left_gain = angle.cos();
        let right_gain = angle.sin();

        for frame in output_ctx.output.chunks_exact_mut(2) {
            let mono = frame[0]; // assume mono input stored in left slot

            frame[0] = mono * left_gain;
            frame[1] = mono * right_gain;
        }
    }
}

impl PullNode for PanningNode {
    fn connect(&mut self, _port: usize, node: usize) {
        self.source = Some(node);
    }
    fn disconnect(&mut self, _port: usize) {
        self.source = None;
    }
}