use crate::engine::process::node::cache::{OutputContext, ProcessNodeCache};

pub struct VecProcessNodeCache {
    cache: Vec<f32>,
    frame: usize,
}

impl VecProcessNodeCache {
    pub fn new(init_size: usize) -> Self {
        Self {
            cache: vec![0_f32; init_size],
            frame: 0,
        }
    }
}

impl ProcessNodeCache for VecProcessNodeCache {
    // using bool instead of Option since it fits
    // the pull flow better

    fn copy<'a>(&self, output_ctx: &mut OutputContext<'a>) -> bool {
        if self.frame != output_ctx.frame {
            return false;
        }

        if self.cache.len() != output_ctx.output.len() {
            return false;
        }

        output_ctx.output.copy_from_slice(&self.cache);
        return true;
    }
    
    fn cache<'a>(&mut self, output_ctx: &OutputContext<'a>) {
        self.frame = output_ctx.frame;
        self.cache = output_ctx.output.to_vec();
    }
}

pub struct EmptyProcessNodeCache;

impl ProcessNodeCache for EmptyProcessNodeCache {
    fn copy<'a>(&self, _output_ctx: &mut OutputContext<'a>) -> bool {
        false
    }

    fn cache<'a>(&mut self, _output_ctx: &OutputContext<'a>) {}
}