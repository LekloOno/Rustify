use audio_engine::{AudioCommand, AudioEngine};

slint::include_modules!();

fn main() {
    let engine = AudioEngine::new();
    let ui = AppWindow::new().unwrap();

    let engin_clone_play = engine.clone();
    ui.on_play_clicked(move || {
        println!("Play button clicked");
    });

    let engine_clone_pause = engine.clone();
    ui.on_pause_clicked(move || {
        println!("Pause button clicked");
    });

    let engine_clone_resume = engine.clone();
    ui.on_resume_clicked(move || {
        println!("Resume button clicked");
    });

    let engine_clone_stop = engine.clone();
    ui.on_stop_clicked(move || {
        println!("Stop button clicked");
    });

    let engine_clone_volume = engine.clone();
    ui.on_volume_changed(move |volume| {
        println!("Volume changed: {}", volume);
    });

    ui.run().unwrap();
}