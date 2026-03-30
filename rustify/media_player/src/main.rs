use audio_engine::{AudioCommand, AudioEngine};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Test Starting....");
    let engine = AudioEngine::new();
    engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    thread::sleep(Duration::from_millis(100000000));
}