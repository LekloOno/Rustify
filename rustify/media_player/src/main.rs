use audio_engine::{AudioCommand, AudioEngine};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Test Starting....");
    let engine = AudioEngine::new();
    engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    thread::sleep(Duration::from_secs(2));
    engine.send_command(AudioCommand::SetVolume(0.5));
    engine.send_command(AudioCommand::Stop);
    thread::sleep(Duration::from_millis(100));
}