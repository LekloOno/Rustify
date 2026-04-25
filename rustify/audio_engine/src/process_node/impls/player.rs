use std::sync::mpsc::{self, Receiver, Sender, sync_channel};
use std::thread;
use ringbuf::{HeapCons, HeapProd, HeapRb};

use crate::decoder::AudioDecoder;
use crate::process_node::topology::base::ProcessNode;
use crate::process_node::topology::graph::ProcessGraph;
use crate::process_node::topology::graph::cache::{OutputContext, ProcessGraphCache};
use ringbuf::traits::{Consumer, Split};

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
pub struct Player {
    id: usize,
    command_tx: Sender<AudioCommand>,
    sample_rx: HeapCons<f32>,
}

impl Player {

    /// Initializes the audio engine by starting the audio thread and setting up communication channels.
    ///
    /// # Example
    /// ```rust, no_run
    /// use audio_engine::{Player, AudioCommand};
    /// 
    /// // Initialize the audio engine
    /// let engine = Player::new();
    /// 
    /// // Send a command to play an audio file
    /// engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    /// ```
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        let ring_buffer = HeapRb::<f32>::new(44100 * 2);
        let (sample_tx, sample_rx) = ring_buffer.split();

        thread::spawn(move || {
            Self::audio_thread_loop(command_rx, sample_tx);
        });

        Player { id: Self::next_id(), command_tx, sample_rx }
    }

    /// Send a command to the audio engine to control playback.
    pub fn send_command(&self, cmd: AudioCommand) {
        if let Err(e) = self.command_tx.send(cmd) {
            eprintln!("Failed to send command: {}", e);
        }
    }

    /// Main loop for the audio thread that processes commands and handles audio decoding and output.
    fn audio_thread_loop(command_rx: Receiver<AudioCommand>, mut sample_tx: HeapProd<f32>) {
        let mut decoder: Option<AudioDecoder> = None;
        let mut is_paused = false;
        let mut volume = 1.0;

        loop {
            while let Ok(cmd) = command_rx.try_recv() {
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
                    match dec.decode_next(&mut sample_tx, volume) {
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

impl ProcessNode for Player {
    fn id(&self) -> usize {
        self.id
    }
    
    fn process_new<'a>(
        &mut self,
        output_ctx: &mut OutputContext<'a>,
        _graph_cache: &mut dyn ProcessGraphCache,
        _graph: &mut ProcessGraph
    ) {
        output_ctx.output.fill_with(||
            self.sample_rx
                .try_pop()
                .unwrap_or(0.0)
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let _player = Player::new();
    }

    #[test]
    fn test_send_play_command() {
        let player = Player::new();
        player.send_command(AudioCommand::Play("test.mp3".to_string()));
    }

    #[test]
    fn test_send_pause_command() {
        let player = Player::new();
        player.send_command(AudioCommand::Pause);
    }

    #[test]
    fn test_send_resume_command() {
        let player = Player::new();
        player.send_command(AudioCommand::Resume);
    }

    #[test]
    fn test_send_stop_command() {
        let player = Player::new();
        player.send_command(AudioCommand::Stop);
    }

    #[test]
    fn test_send_set_volume_command() {
        let player = Player::new();
        player.send_command(AudioCommand::SetVolume(0.5));
    }

    #[test]
    fn test_send_play_when_music_is_playing() {
        let player = Player::new();
        player.send_command(AudioCommand::Play("test.mp3".to_string()));
        player.send_command(AudioCommand::Play("test2.mp3".to_string()));
    }

    #[test]
    fn test_send_multiple_commands() {
        let player = Player::new();
        player.send_command(AudioCommand::Play("test.mp3".to_string()));
        player.send_command(AudioCommand::SetVolume(0.5));
        player.send_command(AudioCommand::Pause);
        player.send_command(AudioCommand::Resume);
        player.send_command(AudioCommand::Stop);
    }

}