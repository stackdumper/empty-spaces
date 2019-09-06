use super::super::components;
use rayon::iter::ParallelIterator;
use shred::{ResourceId, World};
use specs::{Join, ParJoin, ReadStorage, System, SystemData, WriteStorage};

pub struct Force;

#[derive(SystemData)]
pub struct ForceData<'a> {
    velocity: WriteStorage<'a, components::Velocity>,
    force: WriteStorage<'a, components::Force>,
    mass: ReadStorage<'a, components::Mass>,
}

impl<'a> System<'a> for Force {
    type SystemData = ForceData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // (&data.mass, &mut data.force, &mut data.velocity)
        //     .par_join()
        //     .for_each(|(mass, force, velocity)| {
        //         // F = m * a, a = F / m
        //         velocity.data += force.data / mass.data;

        //         // reset force
        //         force.data *= 0.0;
        //     })

        for (mass, force, velocity) in (&data.mass, &mut data.force, &mut data.velocity).join() {
            velocity.data += force.data / mass.data;

            // reset force
            force.data *= 0.0;
        }
    }
}
