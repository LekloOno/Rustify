use std::sync::mpsc::{self, SyncSender, Sender};

use crate::{process::{command::Command, node::{PullProcessNode, impls::sine_osc::SineOscNode}}, topology::node::TopologyNode};

pub struct TopologySineOsc {
    pub node: TopologyNode,
    pub command_tx: SyncSender<f32>,
}

impl TopologySineOsc {
    pub fn new(tx: Sender<Command>, initial_freq: f32, sample_rate: f32) -> Self {
        let (command_tx, command_rx) = mpsc::sync_channel(1);

        let decoder: Box<dyn PullProcessNode> = Box::new(SineOscNode::new(command_rx, initial_freq, sample_rate));
        let node = TopologyNode::new(decoder, tx);

        Self {node, command_tx }
    }
}