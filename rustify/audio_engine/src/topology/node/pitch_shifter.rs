use std::sync::mpsc::{self, Sender, SyncSender};

use crate::{process::{command::Command, node::{PullProcessNode, impls::pitch_shifter::PitchNode}}, topology::node::TopologyNode};

pub struct TopologyPitch {
    pub node: TopologyNode,
    pub command_tx: SyncSender<f32>,
}

impl TopologyPitch {
    pub fn new(tx: Sender<Command>, initial_pitch: f32) -> Self {
        let (command_tx, command_rx) = mpsc::sync_channel(1);

        let distorter: Box<dyn PullProcessNode> = Box::new(PitchNode::new(command_rx, initial_pitch));
        let node = TopologyNode::new(distorter, tx);

        Self {node, command_tx }
    }
}