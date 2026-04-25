//! The processing side of the engine.
//! 
//! You will find the functionnal representation of the processing graph, implementations as well as the main thread representation.
 
pub mod node;
pub mod command;

use std::{process::Command, sync::{Arc, Mutex}};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::HeapCons; 
use ringbuf::traits::Consumer;
use std::sync::mpsc::{SyncSender, sync_channel};

use crate::engine::process::node::{ProcessNode, cache::OutputContext, impls::sink::Sink};

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

        let mut frame = 0;

        let (tx, rx) = sync_channel::<Command>(1024);
        let mut root = Sink::new();

        let stream = device.build_output_stream (
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                frame += 1;
                let mut ctx = OutputContext::new(data, frame);
                
                root.process(&mut ctx);
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