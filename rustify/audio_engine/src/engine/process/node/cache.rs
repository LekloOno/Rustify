pub mod impls;

pub struct OutputContext<'a> {
    pub output: &'a mut [f32],
    pub frame: usize,
}

impl<'a> OutputContext<'a> {
    pub fn new(output: &'a mut [f32], frame: usize) -> Self {
        Self {output, frame}
    }
}

pub trait ProcessNodeCache {
    // using bool instead of Option since it fits
    // the pull flow better
    fn copy<'a>(&self, output_ctx: &mut OutputContext<'a>) -> bool;
    fn cache<'a>(&mut self, output_ctx: &OutputContext<'a>);
}