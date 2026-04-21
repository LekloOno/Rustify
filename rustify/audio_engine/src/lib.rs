/// Main library file for the audio engine module.
/// It defines the AudioEngine struct and its associated methods, as well as the AudioCommand enum for controlling playback.

pub mod decoder;
pub mod output;

use std::sync::mpsc::{self, Receiver, Sender, sync_channel};
use std::thread;

use decoder::AudioDecoder;
use output::AudioOutput;

/// Enum representing commands for controlling audio playback.
pub enum AudioCommand {
    /// Play a new audio file specified by the path.
    Play(String),
    /// Pause the current audio playback.
    Pause,
    /// Resume the paused audio playback.
    Resume,
    /// Stop the current audio playback.
    Stop,
    /// Set the playback volume (0.0 to 1.0).
    SetVolume(f32),
}

/// Represents the audio engine that manages audio playback, decoding, and output.
/// It runs an audio thread that listens for commands and handles audio processing.
#[derive(Clone)]
pub struct AudioEngine {
    command_tx: Sender<AudioCommand>,
}

impl AudioEngine {

    /// Initializes the audio engine by starting the audio thread and setting up communication channels.
    ///
    /// # Example
    /// ```rust, no_run
    /// use audio_engine::{AudioEngine, AudioCommand};
    /// 
    /// // Initialize the audio engine
    /// let engine = AudioEngine::new();
    /// 
    /// // Send a command to play an audio file
    /// engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    /// ```
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        thread::spawn(move || {
            Self::audio_thread_loop(command_rx);
        });

        AudioEngine { command_tx }
    }

    /// Send a command to the audio engine to control playback.
    pub fn send_command(&self, cmd: AudioCommand) {
        if let Err(e) = self.command_tx.send(cmd) {
            eprintln!("Failed to send command: {}", e);
        }
    }

    /// Main loop for the audio thread that processes commands and handles audio decoding and output.
    fn audio_thread_loop(rx: Receiver<AudioCommand>) {
        let (sample_tx, sample_rx) = sync_channel::<f32>(44100*2);
        let _audio_output = AudioOutput::new(sample_rx);
        let mut decoder: Option<AudioDecoder> = None;
        let mut is_paused = false;
        let mut volume = 1.0;

        loop {
            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    AudioCommand::Play(path) => {
                        decoder = Some(AudioDecoder::new(&path));
                        is_paused = false;
                    }
                    AudioCommand::Pause => {
                        is_paused = true;
                        println!("Audio paused");
                    }
                    AudioCommand::Resume => {
                        is_paused = false;
                        println!("Audio resumed");
                    }
                    AudioCommand::Stop => {
                        decoder = None;
                        println!("Audio stopped");
                    }
                    AudioCommand::SetVolume(v) => {
                        volume = v;
                        println!("Volume set to {}", (v*100.0) as i32);
                    }
                }
            }

            if !is_paused {
                if let Some(ref mut dec) = decoder {
                    match dec.decode_next(&sample_tx, volume) {
                        Ok(true) => {}
                        Ok(false) => {
                            println!("Playback finished");
                            decoder = None;
                        }
                        Err(e) => {
                            eprintln!("Error during decoding: {}", e);
                            decoder = None;
                        }
                    }
                }
            }

            if is_paused || decoder.is_none() {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let _engine = AudioEngine::new();
    }

    #[test]
    fn test_send_play_command() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    }

    #[test]
    fn test_send_pause_command() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::Pause);
    }

    #[test]
    fn test_send_resume_command() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::Resume);
    }

    #[test]
    fn test_send_stop_command() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::Stop);
    }

    #[test]
    fn test_send_set_volume_command() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::SetVolume(0.5));
    }

    #[test]
    fn test_send_play_when_music_is_playing() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::Play("test.mp3".to_string()));
        engine.send_command(AudioCommand::Play("test2.mp3".to_string()));
    }

    #[test]
    fn test_send_multiple_commands() {
        let engine = AudioEngine::new();
        engine.send_command(AudioCommand::Play("test.mp3".to_string()));
        engine.send_command(AudioCommand::SetVolume(0.5));
        engine.send_command(AudioCommand::Pause);
        engine.send_command(AudioCommand::Resume);
        engine.send_command(AudioCommand::Stop);
    }

}