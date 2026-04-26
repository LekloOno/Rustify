use audio_engine::{process::node::{PullProcessNode, impls::mixer::MixerNode}, topology::node::{TopologyNode}};
use slint::{Model, ModelRc};

use crate::App;

impl App {
    pub fn setup_mixers(self) -> Self {
        self.ui.set_mixers(ModelRc::from(self.mixers_model.clone()));

        self.ui.on_add_mixer({
            let process_tx = self.process_tx.clone();
            let model: std::rc::Rc<slint::VecModel<crate::UiMixerNode>> = self.mixers_model.clone();
            let handles = self.mixer_handles.clone();

            move || {
                let node : Box<dyn PullProcessNode> = Box::new(MixerNode::new());
                let mixer = TopologyNode::new(node, process_tx.clone());
                let id = mixer.id() as i32;
                handles.borrow_mut().insert(id, mixer);

                model.push(crate::UiMixerNode {
                    id,
                });
            }
        });

        
        self.ui.on_remove_mixer({
            let model = self.mixers_model.clone();
            let handles = self.mixer_handles.clone();

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

        self
    }
}