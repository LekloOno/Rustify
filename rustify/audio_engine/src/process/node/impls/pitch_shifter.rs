use std::sync::mpsc::Receiver;

use crate::process::node::{ProcessNode, PullNode, cache::{OutputContext, impls::VecProcessNodeCache}};

pub struct PitchNode {
    rx: Receiver<f32>,
    source: Option<usize>,
    pitch: f32,
    cache: VecProcessNodeCache,
}

impl PitchNode {
    pub fn new(rx: Receiver<f32>, pitch: f32) -> Self {
        Self {
            rx,
            source: None,
            pitch,
            cache: VecProcessNodeCache::new(50),
        }
    }
}

impl ProcessNode for PitchNode {
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
        if let Ok(pitch) = self.rx.try_recv() {
            self.pitch = pitch;
        }

        let Some(source_id) = self.source else {
            output_ctx.output.fill(0.0);
            return;
        };

        let Some(mut source) = graph.0.remove(&source_id) else {
            return;
        };

        let out_len = output_ctx.output.len();
        let in_len = ((out_len as f32) * self.pitch).round() as usize;

        let mut pitched_buff = vec![0_f32; in_len];
        let mut pitched_ctx = OutputContext::new(&mut pitched_buff, output_ctx.frame);

        source.process(&mut pitched_ctx, graph);
        graph.insert(source_id, source);

        for (i, sample) in output_ctx.output.iter_mut().enumerate() {
            let src_pos = (i as f32) * (in_len as f32 / out_len as f32);
            let src_index = src_pos.round() as usize;

            *sample = pitched_buff[src_index.min(in_len - 1)];
        }
    }
}

impl PullNode for PitchNode {
    fn connect(&mut self, _port: usize, node: usize) {
        self.source = Some(node);
    }
    fn disconnect(&mut self, _port: usize) {
        self.source = None;
    }
}