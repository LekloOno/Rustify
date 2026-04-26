use std::sync::mpsc::{self, Sender, SyncSender};

use crate::{process::{command::Command,node::{PullProcessNode,impls::panning::PanningNode,}},topology::node::TopologyNode};

pub struct TopologyPanning {
    pub node: TopologyNode,
    pub command_tx: SyncSender<f32>,
}

impl TopologyPanning {
    pub fn new(tx: Sender<Command>, initial_pan: f32) -> Self {
        let (command_tx, command_rx) = mpsc::sync_channel(1);

        let panner: Box<dyn PullProcessNode> =
            Box::new(PanningNode::new(command_rx, initial_pan));

        let node = TopologyNode::new(panner, tx);

        Self { node, command_tx }
    }
}