use audio_engine::{process::{ProcessThread, node::impls::mixer::MixerNode}, topology::node::{TopologyNode, player::{TopologyPlayer, command::AudioCommand}, sine_osc::TopologySineOsc}};
use slint::SharedString;
mod scanner;

slint::include_modules!();

fn main() {
    let tracks = scanner::scan_directory("./music");

    let (process_thread, process_tx) = ProcessThread::new();

    let decoder = TopologyPlayer::new(process_tx.clone());
    let decoder2 = TopologyPlayer::new(process_tx.clone());

    let mixer = TopologyNode::new(Box::new(MixerNode::new()), process_tx.clone());

    decoder2.command_tx.send(AudioCommand::Play("./music\\uhoh2.mp3".to_string()));

    let sine_osc = TopologySineOsc::new(process_tx.clone(), 500.0, 48000.0);

    mixer.set_root();

    //mixer.connect(0, &decoder.node);
    //mixer.connect(1, &decoder2.node);
    mixer.connect(2, &sine_osc.node);
    
    let ui = AppWindow::new().unwrap();

    if let Some(first_track) = tracks.first() {
        let display_text = format!("{} - {}",
            first_track.artist.as_deref().unwrap_or("Unknown Artist"),
            first_track.title.as_deref().unwrap_or("Unknown Title")
        );
        ui.set_current_track(SharedString::from(display_text));
    }
    else {
        ui.set_current_track(SharedString::from("No tracks found"));
    }

    let engine_clone_play = decoder.command_tx.clone();
    ui.on_play_clicked(move || {
        if let Some(first_track) = tracks.first() {
            println!("Play button clicked: {}", first_track.path);
            engine_clone_play.send(AudioCommand::Play(first_track.path.clone()));
        }
        else {
            println!("Play button clicked but no tracks found");
        }
    });

    let engine_clone_pause = decoder.command_tx.clone();
    ui.on_pause_clicked(move || {
        println!("Pause button clicked");
        engine_clone_pause.send(AudioCommand::Pause);
    });

    let engine_clone_resume = decoder.command_tx.clone();
    ui.on_resume_clicked(move || {
        println!("Resume button clicked");
        engine_clone_resume.send(AudioCommand::Resume);
    });

    let engine_clone_stop = decoder.command_tx.clone();
    ui.on_stop_clicked(move || {
        println!("Stop button clicked");
        engine_clone_stop.send(AudioCommand::Stop);
    });

    let engine_clone_volume = decoder.command_tx.clone();
    ui.on_volume_changed(move |volume| {
        println!("Volume changed: {}", volume);
        engine_clone_volume.send(AudioCommand::SetVolume(volume));
    });

    ui.run().unwrap();
}