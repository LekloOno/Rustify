/// Main library file for the audio engine module.
/// It defines the AudioEngine struct and its associated methods, as well as the AudioCommand enum for controlling playback.

pub mod decoder;
pub mod output;
pub mod process_node;

use std::sync::mpsc::Sender;

use crate::process_node::topology::base::ProcessNode;

pub enum EngineCommand {
    AddNode(Box<dyn ProcessNode>),
    ConnectNode {
        target: usize,
        port: usize,
        node: usize,
    },
}


/// Represents the audio engine that manages audio playback, decoding, and output.
/// It runs an audio thread that listens for commands and handles audio processing.
#[derive(Clone)]
pub struct AudioEngine {
    command_tx: Sender<EngineCommand>,
}

impl AudioEngine {

}