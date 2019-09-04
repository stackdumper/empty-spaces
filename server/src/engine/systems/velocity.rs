use super::super::{components, resources};
use shred::{ResourceId, World};
use specs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

pub struct Velocity;

#[derive(SystemData)]
pub struct VelocityData<'a> {
    clock: Read<'a, resources::Clock>,
    velocity: ReadStorage<'a, components::Velocity>,
    position: WriteStorage<'a, components::Position>,
}

impl<'a> System<'a> for Velocity {
    type SystemData = VelocityData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (position, velocity) in (&mut data.position, &data.velocity).join() {
            position.data += velocity.data * data.clock.dt;
        }
    }
}
