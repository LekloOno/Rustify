use audio_engine::{AudioCommand, AudioEngine};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Test Starting....");
    let engine = AudioEngine::new();
    
    engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    
    thread::sleep(Duration::from_secs(3));
    
    engine.send_command(AudioCommand::SetVolume(0.2));
    thread::sleep(Duration::from_secs(3));

    engine.send_command(AudioCommand::Pause);
    thread::sleep(Duration::from_secs(3));

    engine.send_command(AudioCommand::SetVolume(1.0));
    engine.send_command(AudioCommand::Resume);
    thread::sleep(Duration::from_secs(3));

    engine.send_command(AudioCommand::Stop);
    thread::sleep(Duration::from_secs(1));
}