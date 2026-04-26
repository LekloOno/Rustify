use audio_engine::topology::node::panning::TopologyPanning;
use slint::{Model, ModelRc};

use crate::{App, UiPanningNode};

impl App {
    pub fn setup_pan(self) -> Self {
        self.ui.set_pans(ModelRc::from(self.pans_model.clone()));

        self.ui.on_add_panning({
            let process_tx = self.process_tx.clone();
            let pans_model = self.pans_model.clone();
            let pan_handles = self.pan_handles.clone();

            move || {
                let pan = TopologyPanning::new(process_tx.clone(), 0.5);
                let id = pan.node.id() as i32;
                pan_handles.borrow_mut().insert(id, pan);

                pans_model.push(UiPanningNode {
                    id: id,
                    pan: 0.5,
                    source_id: -1,
                });
            }
        });

        
        self.ui.on_remove_pan_node({
            let model = self.pans_model.clone();
            let pan_handles = self.pan_handles.clone();

            move |id| {
                pan_handles.borrow_mut().remove(&id);
                
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

        self.ui.on_set_pan({
            let handles = self.pan_handles.clone();

            move |id, pan| {
                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.send(pan);
            }
        });

        self
    }
}