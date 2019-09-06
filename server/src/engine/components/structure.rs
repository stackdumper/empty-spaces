use super::super::types::Vector;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    position: Vector,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
    pub sections: Vec<Section>,
}

impl Component for Structure {
    type Storage = VecStorage<Self>;
}

impl Structure {
    pub fn intersects(&self, other: &Structure) -> bool {
        false
    }

    pub fn merge(&mut self, other: &Structure) {}
}
