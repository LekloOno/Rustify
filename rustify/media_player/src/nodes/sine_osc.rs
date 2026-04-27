use audio_engine::topology::node::sine_osc::TopologySineOsc;
use slint::{Model, ModelRc, SharedString};

use crate::App;

impl App {
    pub fn setup_sine_osc(self) -> Self {
        self.ui.set_sine_oscs(ModelRc::from(self.sine_oscs_model.clone()));

        self.ui.on_add_sine_osc({
            let process_tx = self.process_tx.clone();
            let model = self.sine_oscs_model.clone();
            let handles = self.sine_osc_handles.clone();

            move || {
                let pan = TopologySineOsc::new(process_tx.clone(), 200.0, 480000.0);
                let id = pan.node.id() as i32;
                handles.borrow_mut().insert(id, pan);

                model.push(crate::UiSineOscNode { freq: SharedString::from("200.0"), id });
            }
        });

        
        self.ui.on_remove_sine_osc({
            let model = self.sine_oscs_model.clone();
            let handles = self.sine_osc_handles.clone();

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

        self.ui.on_set_sine_osc_freq({
            let handles = self.sine_osc_handles.clone();

            move |id, freq| {
                let Ok(to) = freq.parse::<f32>() else {
                    return;
                };

                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.try_send(to);
            }
        });

        self
    }
}