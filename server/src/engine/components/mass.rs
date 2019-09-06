use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mass {
    pub data: f64,
}

impl Component for Mass {
    type Storage = VecStorage<Self>;
}

impl Mass {
    pub fn new(data: f64) -> Self {
        Self { data }
    }
}
