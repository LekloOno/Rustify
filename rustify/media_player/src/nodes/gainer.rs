use audio_engine::topology::node::gain::TopologyGain;
use slint::{Model, ModelRc};

use crate::App;

impl App {
    pub fn setup_gainer(self) -> Self {
        self.ui.set_gainers(ModelRc::from(self.gainers_model.clone()));

        self.ui.on_add_gainer({
            let process_tx = self.process_tx.clone();
            let model = self.gainers_model.clone();
            let handles = self.gainer_handles.clone();

            move || {
                let pan = TopologyGain::new(process_tx.clone(), 1.0);
                let id = pan.node.id() as i32;
                handles.borrow_mut().insert(id, pan);

                model.push(crate::UiGainNode {gain: 1.0, id, source_id: -1});
            }
        });

        
        self.ui.on_remove_gainer({
            let model = self.gainers_model.clone();
            let handles = self.gainer_handles.clone();

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

        self.ui.on_set_gain({
            let handles = self.gainer_handles.clone();

            move |id, gain| {

                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.try_send(gain);
            }
        });

        self
    }
}