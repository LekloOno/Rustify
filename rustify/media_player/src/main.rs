use audio_engine::{process::{ProcessThread, command::Command}, topology::node::{TopologyNode, distorsion::TopologyDistorsion, gain::TopologyGain, panning::TopologyPanning, pitch_shifter::TopologyPitch, player::TopologyPlayer, sine_osc::TopologySineOsc}};
mod scanner;

use slint::VecModel;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::mpsc::Sender};

slint::include_modules!();

mod nodes;

pub struct App {
    ui: AppWindow,
    
    process_tx: Sender<Command>,

    pan_handles: Rc<RefCell<HashMap<i32, TopologyPanning>>>,
    pans_model: Rc<VecModel<UiPanningNode>>,

    player_handles: Rc<RefCell<HashMap<i32, TopologyPlayer>>>,
    players_model: Rc<VecModel<UiPlayerNode>>,

    mixer_handles: Rc<RefCell<HashMap<i32, TopologyNode>>>,
    mixers_model: Rc<VecModel<UiMixerNode>>,

    distorsion_handles: Rc<RefCell<HashMap<i32, TopologyDistorsion>>>,
    distorsion_model: Rc<VecModel<UiDistorsionNode>>,

    sine_osc_handles: Rc<RefCell<HashMap<i32, TopologySineOsc>>>,
    sine_oscs_model: Rc<VecModel<UiSineOscNode>>,

    gainer_handles: Rc<RefCell<HashMap<i32, TopologyGain>>>,
    gainers_model: Rc<VecModel<UiGainNode>>,

    pitcher_handles: Rc<RefCell<HashMap<i32, TopologyPitch>>>,
    pitchers_model: Rc<VecModel<UiPitchNode>>,
}

impl App {
    pub fn new(process_tx: Sender<Command>) -> Self {
        let ui = AppWindow::new().unwrap();

        let pan_handles: Rc<RefCell<HashMap<i32, TopologyPanning>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let pans_model =
            Rc::new(VecModel::<UiPanningNode>::default());

        let player_handles: Rc<RefCell<HashMap<i32, TopologyPlayer>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let players_model =
            Rc::new(VecModel::<UiPlayerNode>::default());

        let mixer_handles: Rc<RefCell<HashMap<i32, TopologyNode>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let mixers_model =
            Rc::new(VecModel::<UiMixerNode>::default());

        let distorsion_handles: Rc<RefCell<HashMap<i32, TopologyDistorsion>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let distorsion_model =
            Rc::new(VecModel::<UiDistorsionNode>::default());

        let sine_osc_handles: Rc<RefCell<HashMap<i32, TopologySineOsc>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let sine_oscs_model =
            Rc::new(VecModel::<UiSineOscNode>::default());

        let gainer_handles: Rc<RefCell<HashMap<i32, TopologyGain>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let gainers_model =
            Rc::new(VecModel::<UiGainNode>::default());

        let pitcher_handles: Rc<RefCell<HashMap<i32, TopologyPitch>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let pitchers_model =
            Rc::new(VecModel::<UiPitchNode>::default());

        Self {
            ui,
            process_tx,
            pan_handles,
            pans_model,
            player_handles,
            players_model,
            mixer_handles,
            mixers_model,
            distorsion_handles,
            distorsion_model,
            sine_osc_handles,
            sine_oscs_model,
            gainer_handles,
            gainers_model,
            pitcher_handles,
            pitchers_model,
        }
    }

    pub fn run(self) -> Self {
        self.ui.run().unwrap();
        self
    }
}

fn main() {
    let (_process_thread, process_tx) = ProcessThread::new();

    let _app = App::new(process_tx)
        .setup_nodes()
        .setup_pan()
        .setup_player()
        .setup_mixers()
        .setup_sine_osc()
        .setup_gainer()
        .setup_distorsion()
        .setup_pitcher()
        .run();
    
    //ui.set_pans(model.clone().into());
/* 
    let tracks = scanner::scan_directory("./music");

    let (process_thread, process_tx) = ProcessThread::new();

    let decoder = TopologyPlayer::new(process_tx.clone());
    let decoder2 = TopologyPlayer::new(process_tx.clone());

    let mixer = TopologyNode::new(Box::new(MixerNode::new()), process_tx.clone());

    decoder2.command_tx.send(AudioCommand::Play("./music\\uhoh2.mp3".to_string()));

    let sine_osc = TopologySineOsc::new(process_tx.clone(), 100.0, 48000.0);
    let gain = TopologyGain::new(process_tx.clone(), 1.4);
    let distorsion = TopologyDistorsion::new(process_tx.clone(), cubic_distortion);
    let panning = TopologyPanning::new(process_tx.clone(), 0.0);

    panning.node.connect(0, &distorsion.node);


    gain.node.connect(0, &decoder2.node);
    distorsion.node.connect(0, &mixer);

    panning.node.set_root();

    mixer.connect(0, &decoder.node);
    mixer.connect(1, &gain.node);
    //mixer.connect(2, &sine_osc.node);
    
    let ui = AppWindow::new().unwrap();

    ui.on_add_panning_node({
        let model = model.clone();
        let nodes = nodes.clone(); // Arc<Mutex<AppState>> typically

        move || {
            let id = state.next_id;
            state.next_id += 1;

            let panning = TopologyPanning::new(process_tx.clone(), 0.0);

            let tx = panning.command_tx.clone();

            nodes.insert(id, PanningHandle {
                node: panning,
                tx,
            });

            model.push(PanningItem {
                id,
                pan: 0.0,
            });
        }
    });

    ui.on_pan_changed({
        let nodes = nodes.clone();

        move |id, value| {
            if let Some(handle) = nodes.lock().unwrap().get(&id) {
                let _ = handle.tx.send(value);
            }
        }
    });

    ui.on_remove_node({
        let nodes = nodes.clone();
        let model = model.clone();

        move |id| {
            nodes.lock().unwrap().remove(&id);

            let index = model.iter().position(|x| x.id == id);
            if let Some(i) = index {
                model.remove(i);
            }
        }
    });

    let panning_changed = panning.command_tx.clone();
    ui.on_pan_changed(move |pan| {
        panning_changed.send(pan);
    });

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
    });*/
}