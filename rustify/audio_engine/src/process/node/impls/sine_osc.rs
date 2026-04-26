use std::sync::mpsc::Receiver;

use crate::process::node::{ProcessNode, PullNode, cache::impls::VecProcessNodeCache};

pub struct SineOscNode {
    rx: Receiver<f32>,
    phase: f32,
    freq: f32,
    sample_rate: f32,
    cache: VecProcessNodeCache,
}

impl SineOscNode {
    pub fn new(rx: Receiver<f32>, freq: f32, sample_rate: f32) -> Self {
        Self {
            rx,
            phase: 0.0,
            freq,
            sample_rate,
            cache: VecProcessNodeCache::new(50),
        }
    }

    fn next_sample(&mut self) -> f32 {
        let sample = (2.0 * std::f32::consts::PI * self.phase).sin();
        self.phase += self.freq / self.sample_rate;
        self.phase %= 1.0;

        sample
    }
}

impl ProcessNode for SineOscNode {
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
        if let Ok(freq) = self.rx.try_recv() {
            self.freq = freq;
        }

        output_ctx.output.fill_with(||
            self.next_sample()
        );
    }
}

impl PullNode for SineOscNode {
    fn connect(&mut self, _port: usize, _node: usize) {}
    fn disconnect(&mut self, _port: usize) {}
}