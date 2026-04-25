//! The processing side of the engine.
//! 
//! You will find the functionnal representation of the processing graph, implementations as well as the main thread representation.
 
pub mod node;
pub mod command;

use std::{collections::HashMap};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{SyncSender, sync_channel};

use crate::engine::process::{command::Command, node::{PullProcessNode, cache::OutputContext, impls::sink::Sink}};

pub struct ProcessGraph(pub HashMap<usize, Box<dyn PullProcessNode>>);

impl ProcessGraph {
    pub fn new() -> Self {
        let root: Box<dyn PullProcessNode> = Box::new(Sink::new());
        Self(HashMap::from([(0, root)]))
    }

    pub fn insert(&mut self, id: usize, node: Box<dyn PullProcessNode>) {
        self.0.insert(id, node);
    }

    pub fn remove(&mut self, id: usize) {
        self.0.remove(&id);
    }

    pub fn connect(&mut self, id: usize, port: usize, input_id: usize) {
        if let Some(target) = self.0.get_mut(&id) {
            target.connect(port, input_id);
        }

    }

    pub fn disconnect(&mut self, id: usize, port: usize) {
        if let Some(target) = self.0.get_mut(&id) {
            target.disconnect(port);
        }
    }
}

pub struct Process {
    pub root: usize,
    pub nodes: ProcessGraph,
    frame: usize,
}

impl Process {
    fn new() -> Self {
        Self { root: 0, nodes: ProcessGraph::new(), frame: 0 }
    }

    fn process<'a>(&mut self, data: &mut [f32]) {
        let mut output_ctx = OutputContext::new(data, self.frame);
        self.frame += 1;

        if let Some(mut root) = self.nodes.0.remove(&self.root) {
            root.process(&mut output_ctx, &mut self.nodes);
            self.nodes.insert(self.root, root);
        }
    }
}

/// Represents the connection to the audio output device.
/// It holds the audio stream and ensures it remains active while the struct is alive.
pub struct ProcessThread {
    _stream: cpal::Stream,
}

impl ProcessThread {

    /// Initializes the audio output by setting up a stream that continuously reads audio samples from the provided receiver.
    ///
    /// # Example
    /// ```rust, no_run
    /// use std::sync::mpsc::sync_channel;
    /// use audio_engine::output::AudioOutput;
    /// 
    /// // Create a channel for audio samples
    /// let (tx, rx) = sync_channel::<f32>(1024);
    /// 
    /// // Initialize the audio output with the receiver
    /// let output = AudioOutput::new(rx);
    /// ```
    pub fn new() -> (Self, SyncSender<Command>) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let config = device.default_output_config().expect("Failed to get default output config");

        let (tx, rx) = sync_channel::<Command>(1024);
        let mut process = Process::new();

        let stream = device.build_output_stream (
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {

                while let Ok(cmd) = rx.try_recv() {
                    cmd.send(&mut process);
                }

                process.process(data);
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        ).expect("Failed to build output stream");

        stream.play().expect("Failed to play stream");

        let thread = ProcessThread {
            _stream: stream,
        };

        (thread, tx)
    }
}