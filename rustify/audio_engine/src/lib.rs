use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

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
            eprintln!("erreur {}", e);
        }
    }

    fn audio_thread_loop(rx: Receiver<AudioCommand>) {

        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        println!("Using output device: {}", device.name().unwrap_or("Unknown".to_string()));

        let config = device.default_output_config().expect("Failed to get default output config");
        println!("Default output config: {:?}", config);

        for cmd in rx {
            match cmd {
                AudioCommand::Play(file) => {
                    println!("Playing audio file: {}", file);
                }
                AudioCommand::Pause => {
                    println!("Pausing audio");
                }
                AudioCommand::Resume => {
                    println!("Resuming audio");
                }
                AudioCommand::Stop => {
                    println!("Stopping audio");
                } 
                AudioCommand::SetVolume(vol) => {
                    println!("Setting volume to: {}", (vol * 100.0) as i32);
                }
            }
        }
    }
}