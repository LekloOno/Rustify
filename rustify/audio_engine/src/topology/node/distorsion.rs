use std::sync::mpsc::{self, Sender};

use crate::{process::{command::Command, node::{PullProcessNode, impls::{distorsion::Distorsion, sine_osc::SineOsc}}}, topology::node::TopologyNode};

pub struct TopologyDistorsion {
    pub node: TopologyNode,
    pub command_tx: Sender<fn(f32) -> f32>,
}

impl TopologyDistorsion {
    pub fn new(tx: Sender<Command>, initial_dist: fn(f32) -> f32) -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        let distorter: Box<dyn PullProcessNode> = Box::new(Distorsion::new(command_rx, initial_dist));
        let node = TopologyNode::new(distorter, tx);

        Self {node, command_tx }
    }
}