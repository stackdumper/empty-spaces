use super::super::types::Vector;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub data: Vector,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
