use super::super::types::Vector;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Velocity {
    pub data: Vector,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
