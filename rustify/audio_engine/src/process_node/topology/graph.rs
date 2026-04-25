use std::collections::HashMap;

pub mod cache;
pub mod error;

pub use error::Result;
pub use error::Error;

use crate::process_node::topology::base::ProcessNode;
use crate::process_node::topology::base::PullProcessNode;


pub struct ProcessGraph {
    nodes: HashMap<usize, Box<dyn PullProcessNode>>,
}

impl ProcessGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn get_node(&mut self, id: usize) -> Option<&mut Box<dyn PullProcessNode>> {
        self.nodes.get_mut(&id)
    }

    pub fn add_node(&mut self, input: Box<dyn PullProcessNode>) {
        self.nodes.insert(input.id(), input);
    }

    pub fn remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
    }

    pub fn connect_to_node(&mut self, target_id: usize, port: usize, node_id: usize) -> Result<()> {
        let target_node = self
            .get_node(target_id)
            .ok_or(Error::NodeNotFound { id: target_id })?;

        target_node.connect(port, node_id);
        Ok(())
    }

    pub fn disconnect_node(&mut self, target_id: usize, port: usize) -> Result<()> {
        let target_node = self
            .get_node(target_id)
            .ok_or(Error::NodeNotFound { id: target_id })?;

        target_node.disconnect(port);
        Ok(())
    }
}


