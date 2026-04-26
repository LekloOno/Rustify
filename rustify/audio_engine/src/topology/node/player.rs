use std::{sync::mpsc::{self, Receiver, Sender, SyncSender}, thread};

use crate::{process::{command::Command, node::{PullProcessNode, impls::decoder::DecoderNode}}, topology::node::{TopologyNode, player::{command::AudioCommand, decoder::AudioDecoder}}};

pub mod command;
pub mod decoder;


pub struct TopologyPlayer {
    pub node: TopologyNode,
    pub command_tx: Sender<AudioCommand>
}

impl TopologyPlayer {
    pub fn new(tx: Sender<Command>) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        let (sample_tx, sample_rx) = mpsc::sync_channel(44100 * 2);
        
        thread::spawn(move || {
            Self::audio_thread_loop(command_rx, sample_tx);
        });

        let decoder: Box<dyn PullProcessNode> = Box::new(DecoderNode::new(sample_rx));
        let node = TopologyNode::new(decoder, tx);

        Self {node, command_tx }
    }

/// Send a command to the audio engine to control playback.
    pub fn send_command(&self, cmd: AudioCommand) {
        if let Err(e) = self.command_tx.send(cmd) {
            eprintln!("Failed to send command: {}", e);
        }
    }

    /// Main loop for the audio thread that processes commands and handles audio decoding and output.
    fn audio_thread_loop(command_rx: Receiver<AudioCommand>, mut sample_tx: SyncSender<f32>) {
        let mut decoder: Option<AudioDecoder> = None;
        let mut is_paused = false;
        let mut volume = 1.0;

        loop {
            while let Ok(cmd) = command_rx.try_recv() {
                match cmd {
                    AudioCommand::Play(path) => {
                        decoder = AudioDecoder::new(&path).ok();
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
