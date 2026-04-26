use audio_engine::process::command::Command;
use slint::Model;

use crate::{App};

pub mod pan;

impl App {
    pub fn setup_nodes(self) -> Self {
        self.ui.on_connect({
            let process_tx = self.process_tx.clone();

            move |id, port, source_txt| {
                let to: i32 = source_txt.parse::<i32>().unwrap_or(-1);

                if to >= 0 {
                    let _ = process_tx.send(Command::ConnectNodes {
                        id: id as usize,
                        port: port as usize,
                        input_id: to as usize
                    });
                } else {
                    let _ = process_tx.send(Command::DisconnectNode {
                        id: id as usize,
                        port: port as usize
                    });
                }
            }
        });

        self
    }
}