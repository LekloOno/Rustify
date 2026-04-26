use std::sync::mpsc::{self, Sender, SyncSender};

use crate::{process::{command::Command, node::{PullProcessNode, impls::gainer::GainNode}}, topology::node::TopologyNode};

pub struct TopologyGain {
    pub node: TopologyNode,
    pub command_tx: SyncSender<f32>,
}

impl TopologyGain {
    pub fn new(tx: Sender<Command>, initial_gain: f32) -> Self {
        let (command_tx, command_rx) = mpsc::sync_channel(1);

        let distorter: Box<dyn PullProcessNode> = Box::new(GainNode::new(command_rx, initial_gain));
        let node = TopologyNode::new(distorter, tx);

        Self {node, command_tx }
    }
}