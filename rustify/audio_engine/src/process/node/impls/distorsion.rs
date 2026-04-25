use std::sync::mpsc::Receiver;

use crate::process::node::{ProcessNode, PullNode, cache::impls::VecProcessNodeCache};

pub struct Distorsion {
    rx: Receiver<fn(f32) -> f32>,
    source: Option<usize>,
    func: fn(f32) -> f32,
    cache: VecProcessNodeCache,
}

impl Distorsion {
    pub fn new(rx: Receiver<fn(f32) -> f32>, func: fn(f32) -> f32) -> Self {
        Self {
            rx,
            source: None,
            func,
            cache: VecProcessNodeCache::new(50),
        }
    }
}

impl ProcessNode for Distorsion {
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
        if let Ok(func) = self.rx.try_recv() {
            self.func = func;
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
            *sample = (self.func)(*sample);
        }
    }
}

impl PullNode for Distorsion {
    fn connect(&mut self, _port: usize, node: usize) {
        self.source = Some(node);
    }
    fn disconnect(&mut self, _port: usize) {
        self.source = None;
    }
}


pub fn overdrive(x: f32) -> f32 {
    let drive = 2.0;
    let x = x * drive;

    if x > 1.0 {
        1.0
    } else if x < -0.5 {
        -0.5
    } else {
        x - 0.2 * x * x
    }
}

pub fn cubic_distortion(x: f32) -> f32 {
    x - (1.0 / 3.0) * x * x * x
}

pub fn hard_clip(x: f32) -> f32 {
    let t = 0.8;
    if x > t { t } else if x < -t { -t } else { x }
}

pub fn soft_clip(x: f32) -> f32 {
    x.tanh()
}

pub fn stoopid(x: f32) -> f32 {
    (x + 1.0).round() - 1.0
}