use std::collections::HashMap;

pub struct OutputContext<'a> {
    pub output: &'a mut [f32],
    pub frame: usize,
}

impl<'a> OutputContext<'a> {
    pub fn new(output: &'a mut [f32], frame: usize) -> Self {
        Self {output, frame}
    }
}

pub struct HashProcessGraphCache {
    map: HashMap<usize, (usize, Vec<f32>)>,
}

impl HashProcessGraphCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    } 
}

pub trait ProcessGraphCache {
    fn copy<'a>(&self, id: usize, output_ctx: &mut OutputContext<'a>) -> bool;
    fn cache<'a>(&mut self, id: usize, output_ctx: &OutputContext<'a>);
}



impl ProcessGraphCache for HashProcessGraphCache {
    fn copy<'a>(&self, id: usize, output_ctx: &mut OutputContext<'a>) -> bool {
        let Some(cached) = self.map.get(&id) else {
            return false;
        };

        if cached.0 != output_ctx.frame {
            return false;
        }

        if cached.1.len() != output_ctx.output.len() {
            return false;
        }

        output_ctx.output.copy_from_slice(&cached.1);
        return true;
    }
    
    fn cache<'a>(&mut self, id: usize, output_ctx: &OutputContext<'a>) {
        self.map.insert(id, (output_ctx.frame, output_ctx.output.to_vec()));
    }
}