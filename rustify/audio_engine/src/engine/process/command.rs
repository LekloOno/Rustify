use crate::engine::process::{Process, node::PullProcessNode};

pub enum Command {
    /// Requests to create a new node, with the provided topology id.
    CreateNode {id: usize, node: Box<dyn PullProcessNode>},
    /// Requests to delete the node by the provided topology id.
    DeleteNode {id: usize},
    /// Requests to connect the provided input node to a port of the target node.
    ConnectNodes {id: usize, port: usize, input_id: usize},
    /// Requests to disconnect the provided port of the target node.
    DisconnectNode {id: usize, port: usize},
    /// Set the node by the provided id as root of the pipeline.
    SetRootNode {id: usize},
}

impl Command {
    /// Sends and consumes the command
    pub fn send(self, process: &mut Process) {
        match self {
            Command::CreateNode { id, node }
                => process.nodes.insert(id, node),
            Command::DeleteNode { id }
                => process.nodes.remove(id),
            Command::ConnectNodes { id, port, input_id }
                => process.nodes.connect(id, port, input_id),
            Command::DisconnectNode { id, port }
                => process.nodes.disconnect(id, port),
            Command::SetRootNode { id }
                => process.root = id,
        };
    }
}