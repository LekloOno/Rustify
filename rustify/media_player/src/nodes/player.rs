use audio_engine::topology::node::player::{TopologyPlayer, command::AudioCommand};
use slint::{ModelRc, SharedString};

use crate::App;

impl App {
    pub fn setup_player(self) -> Self {
        self.ui.set_players(ModelRc::from(self.players_model.clone()));

        self.ui.on_add_player({
            let process_tx = self.process_tx.clone();
            let model: std::rc::Rc<slint::VecModel<crate::UiPlayerNode>> = self.players_model.clone();
            let handles = self.player_handles.clone();

            move || {
                let player = TopologyPlayer::new(process_tx.clone());
                let id = player.node.id() as i32;
                handles.borrow_mut().insert(id, player);

                model.push(crate::UiPlayerNode {
                    id,
                    source_path: SharedString::from("./music\\uhoh2.mp3")
                });
            }
        });

        
        self.ui.on_remove_player({
            let model = self.players_model.clone();
            let handles = self.player_handles.clone();

            move |id| {
                handles.borrow_mut().remove(&id);
                model.remove(id as usize);
            }
        });

        self.ui.on_play_player({
            let handles = self.player_handles.clone();

            move |id, path| {
                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.send(AudioCommand::Play(path.to_string()));
            }
        });

        self.ui.on_pause_player({
            let handles = self.player_handles.clone();

            move |id| {
                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.send(AudioCommand::Pause);
            }
        });

        self.ui.on_stop_player({
            let handles = self.player_handles.clone();

            move |id| {
                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.send(AudioCommand::Stop);
            }
        });

        self
    }
}