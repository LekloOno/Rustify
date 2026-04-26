use std::sync::mpsc::{self, Sender, SyncSender};

use crate::{process::{command::Command, node::{PullProcessNode, impls::{distorsion::DistorsionNode, sine_osc::SineOscNode}}}, topology::node::TopologyNode};

pub struct TopologyDistorsion {
    pub node: TopologyNode,
    pub command_tx: SyncSender<fn(f32) -> f32>,
}

impl TopologyDistorsion {
    pub fn new(tx: Sender<Command>, initial_dist: fn(f32) -> f32) -> Self {
        let (command_tx, command_rx) = mpsc::sync_channel(1);

        let distorter: Box<dyn PullProcessNode> = Box::new(DistorsionNode::new(command_rx, initial_dist));
        let node = TopologyNode::new(distorter, tx);

        Self {node, command_tx }
    }
}