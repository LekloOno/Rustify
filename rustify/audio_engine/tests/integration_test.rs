use audio_engine::{AudioEngine, AudioCommand};
use std::thread;
use std::time::Duration;

#[test]
fn test_public_api_flow() {
    let engine = AudioEngine::new();

    // Test play command
    engine.send_command(AudioCommand::Play("test.mp3".to_string()));
    thread::sleep(Duration::from_secs(1)); // Wait for the audio to start

    // Test pause command
    engine.send_command(AudioCommand::Pause);
    thread::sleep(Duration::from_secs(1)); // Wait for the audio to pause

    // Test resume command
    engine.send_command(AudioCommand::Resume);
    thread::sleep(Duration::from_secs(1)); // Wait for the audio to resume

    // Test set volume command
    engine.send_command(AudioCommand::SetVolume(0.5));
    thread::sleep(Duration::from_secs(1)); // Wait for the volume change to take effect

    // Test stop command
    engine.send_command(AudioCommand::Stop);
}