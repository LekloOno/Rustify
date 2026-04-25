use std::sync::{atomic::{AtomicUsize, Ordering}, mpsc::SyncSender};

use crate::engine::process::{command::Command, node::PullProcessNode};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct TopologyNode {
    id: usize,
    tx: SyncSender<Command>,
}

impl TopologyNode {
    pub fn new(node: Box<dyn PullProcessNode>, tx: SyncSender<Command>) -> Self {
        let id = Self::next_id();
        let _ = tx.send(Command::CreateNode { id, node });
        Self { id, tx }
    }

    pub fn delete(self) {
        let _ = self.tx.send(Command::DeleteNode { id: self.id });
    }

    pub fn connect(&self, port: usize, input: &TopologyNode) {
        let _ = self.tx.send(Command::ConnectNodes {
            id: self.id,
            port,
            input_id: input.id
        });
    }

    pub fn disconnect(&self, port: usize) {
        let _ = self.tx.send(Command::DisconnectNode { id: self.id, port });
    }

    pub fn set_root(&self) {
        let _ = self.tx.send(Command::SetRootNode { id: self.id });
    }

    fn next_id() -> usize where Self: Sized {
        GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}