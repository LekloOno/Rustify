use audio_engine::topology::node::{gain::TopologyGain, pitch_shifter::TopologyPitch};
use slint::{Model, ModelRc};

use crate::App;

impl App {
    pub fn setup_pitcher(self) -> Self {
        self.ui.set_pitchers(ModelRc::from(self.pitchers_model.clone()));

        self.ui.on_add_pitcher({
            let process_tx = self.process_tx.clone();
            let model = self.pitchers_model.clone();
            let handles = self.pitcher_handles.clone();

            move || {
                let pitch = TopologyPitch::new(process_tx.clone(), 1.0);
                let id = pitch.node.id() as i32;
                handles.borrow_mut().insert(id, pitch);

                model.push(crate::UiPitchNode { id, pitch: 1.0, source_id: -1 });
            }
        });

        
        self.ui.on_remove_pitcher({
            let model = self.pitchers_model.clone();
            let handles = self.pitcher_handles.clone();

            move |id| {
                handles.borrow_mut().remove(&id);
                
                if let Some(index) = (0..model.row_count())
                    .find(|&i| {
                        model
                            .row_data(i)
                            .map(|m| m.id == id)
                            .unwrap_or(false)
                    })
                {
                    model.remove(index);
                }
            }
        });

        self.ui.on_set_pitch({
            let handles = self.pitcher_handles.clone();

            move |id, pitch| {

                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.send(pitch);
            }
        });

        self
    }
}