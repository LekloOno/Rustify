pub mod decoder;
pub mod output;

use std::sync::mpsc::{self, Receiver, Sender, sync_channel};
use std::thread;

use decoder::AudioDecoder;
use output::AudioOutput;

pub enum AudioCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub struct AudioEngine {
    command_tx: Sender<AudioCommand>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        thread::spawn(move || {
            Self::audio_thread_loop(command_rx);
        });

        AudioEngine { command_tx }
    }

    pub fn send_command(&self, cmd: AudioCommand) {
        if let Err(e) = self.command_tx.send(cmd) {
            eprintln!("Failed to send command: {}", e);
        }
    }

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