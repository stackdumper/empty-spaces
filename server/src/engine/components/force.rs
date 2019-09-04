use super::super::types::Vector;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Force {
    pub data: Vector,
}

impl Component for Force {
    type Storage = VecStorage<Self>;
}
