use audio_engine::{process::node::impls::distorsion::{cubic_distortion, hard_clip, overdrive, soft_clip, stoopid}, topology::node::distorsion::TopologyDistorsion};
use slint::{Model, ModelRc, SharedString};

use crate::App;

pub enum DistorsionMode {
    None,
    SoftClip,
    HardClip,
    Cubic,
    Overdrive,
    Stoopid,
}

impl Into<fn(f32) -> f32> for DistorsionMode {
    fn into(self) -> fn(f32) -> f32 {
        match self {
            DistorsionMode::None => |x| x,
            DistorsionMode::SoftClip => soft_clip,
            DistorsionMode::HardClip => hard_clip,
            DistorsionMode::Cubic => cubic_distortion,
            DistorsionMode::Overdrive => overdrive,
            DistorsionMode::Stoopid => stoopid,
        }
    }
}

impl From<SharedString> for DistorsionMode {
    fn from(value: SharedString) -> Self {
        let s = value.to_lowercase().replace('-', "_");

        match s.as_str() {
            "none" => DistorsionMode::None,
            "soft_clip" => DistorsionMode::SoftClip,
            "hard_clip" => DistorsionMode::HardClip,
            "cubic" => DistorsionMode::Cubic,
            "overdrive" => DistorsionMode::Overdrive,
            "stoopid" => DistorsionMode::Stoopid,
            _ => DistorsionMode::None,
        }
    }
}

impl App {
    pub fn setup_distorsion(self) -> Self {
        self.ui.set_distorsions(ModelRc::from(self.distorsion_model.clone()));

        self.ui.on_add_distorsion({
            let process_tx = self.process_tx.clone();
            let model = self.distorsion_model.clone();
            let handles = self.distorsion_handles.clone();

            move || {
                let disto = TopologyDistorsion::new(process_tx.clone(), soft_clip);
                let id = disto.node.id() as i32;
                handles.borrow_mut().insert(id, disto);

                model.push(crate::UiDistorsionNode {id, source_id: -1, r#type: SharedString::from("soft_clip")});
            }
        });

        
        self.ui.on_remove_distorsion({
            let model = self.distorsion_model.clone();
            let handles = self.distorsion_handles.clone();

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

        self.ui.on_set_distorsion_mode({
            let handles = self.distorsion_handles.clone();

            move |id, mode| {

                let mut map = handles.borrow_mut();

                let Some(node) = map.get_mut(&id) else {
                    return;
                };
                
                let _ = node.command_tx.send(DistorsionMode::from(mode).into());
            }
        });

        self
    }
}