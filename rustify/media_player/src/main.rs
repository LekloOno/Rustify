use audio_engine::{AudioCommand, AudioEngine};

slint::include_modules!();

fn main() {
    let engine = AudioEngine::new();
    let ui = AppWindow::new().unwrap();

    let engine_clone_play = engine.clone();
    ui.on_play_clicked(move || {
        println!("Play button clicked");
        engine_clone_play.send_command(AudioCommand::Play("test.mp3".to_string()));
    });

    let engine_clone_pause = engine.clone();
    ui.on_pause_clicked(move || {
        println!("Pause button clicked");
        engine_clone_pause.send_command(AudioCommand::Pause);
    });

    let engine_clone_resume = engine.clone();
    ui.on_resume_clicked(move || {
        println!("Resume button clicked");
        engine_clone_resume.send_command(AudioCommand::Resume);
    });

    let engine_clone_stop = engine.clone();
    ui.on_stop_clicked(move || {
        println!("Stop button clicked");
        engine_clone_stop.send_command(AudioCommand::Stop);
    });

    let engine_clone_volume = engine.clone();
    ui.on_volume_changed(move |volume| {
        println!("Volume changed: {}", volume);
        engine_clone_volume.send_command(AudioCommand::SetVolume(volume));
    });

    ui.run().unwrap();
}