/// Handles audio output using the cpal crate.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::Receiver;

/// Represents the connection to the audio output device.
/// It holds the audio stream and ensures it remains active while the struct is alive.
pub struct AudioOutput {
    _stream: cpal::Stream,
}

impl AudioOutput {

    /// Initializes the audio output by setting up a stream that continuously reads audio samples from the provided receiver.
    ///
    /// # Example
    /// ```rust
    /// use std::sync::mpsc::sync_channel;
    /// use audio_engine::output::AudioOutput;
    /// 
    /// // Create a channel for audio samples
    /// let (tx, rx) = sync_channel::<f32>(1024);
    /// 
    /// // Initialize the audio output with the receiver
    /// let output = AudioOutput::new(rx);
    /// ```
    pub fn new(sample_rx: Receiver<f32>) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let config = device.default_output_config().expect("Failed to get default output config");

        let stream = device.build_output_stream (
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = sample_rx.try_recv().unwrap_or(0.0);
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        ).expect("Failed to build output stream");

        stream.play().expect("Failed to play stream");

        AudioOutput {
            _stream: stream,
        }
    }
}