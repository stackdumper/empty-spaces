use super::super::components;
use shred::{ResourceId, World};
use specs::{Join, ReadStorage, System, SystemData, WriteStorage};

pub struct Force;

#[derive(SystemData)]
pub struct ForceData<'a> {
    structure: ReadStorage<'a, components::Structure>,
    velocity: WriteStorage<'a, components::Velocity>,
    force: WriteStorage<'a, components::Force>,
}

impl<'a> System<'a> for Force {
    type SystemData = ForceData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (structure, force, velocity) in
            (&data.structure, &mut data.force, &mut data.velocity).join()
        {
            velocity.data += force.data / structure.get_mass();

            // reset force
            force.data *= 0.0;
        }
    }
}
