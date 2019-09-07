use super::super::components;
use serde_json::to_string;
use shred::{ResourceId, World};
use specs::{prelude::SystemData, Join, ReadStorage, System};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};
use ws;

pub struct Sync {
    out: Arc<Mutex<Option<ws::Sender>>>,
}

impl Sync {
    pub fn new(address: &'static str) -> Self {
        let sf = Self {
            out: Arc::new(Mutex::new(None)),
        };

        // listen in a separate thread
        let out = Arc::clone(&sf.out);
        thread::spawn(move || {
            ws::listen(address, |connection| {
                // keep single connections, anyway we use .broadcast
                *out.lock().unwrap() = Some(connection);

                |_| Ok(())
            })
        });

        sf
    }
}

#[derive(SystemData)]
pub struct SyncData<'a> {
    position: ReadStorage<'a, components::Position>,
    structure: ReadStorage<'a, components::Structure>,
}

impl<'a> System<'a> for Sync {
    type SystemData = SyncData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if let Some(connection) = self.out.lock().unwrap().deref() {
            let ents: Vec<(&components::Position, &components::Structure)> =
                (&data.position, &data.structure).join().collect();

            // send entities to client
            let message = ws::Message::Text(to_string(&ents).unwrap());

            let _ = connection.broadcast(message);
        }
    }
}
