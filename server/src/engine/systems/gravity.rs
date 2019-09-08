use super::super::{components, resources};
use cgmath::{InnerSpace, MetricSpace};
use specs::{Entities, Read, ReadStorage, System, WriteStorage};

const G: f64 = 6.67e-11f64;

pub struct Gravity;

impl<'a> System<'a> for Gravity {
    type SystemData = (
        Read<'a, resources::Clock>,
        Entities<'a>,
        ReadStorage<'a, components::Structure>,
        ReadStorage<'a, components::Position>,
        WriteStorage<'a, components::Force>,
    );

    fn run(&mut self, (clock, entity, structure, position, mut force): Self::SystemData) {
        use rayon::iter::ParallelIterator;
        use specs::{Join, ParJoin};

        (&entity, &structure, &position, &mut force)
            .par_join()
            .for_each(
                |(source_entity, source_structure, source_position, source_force)| {
                    (&entity, &structure, &position).join().for_each(
                        |(target_entity, target_structure, target_position)| {
                            if source_entity.id() == target_entity.id() {
                                return;
                            }

                            let total_mass =
                                source_structure.get_mass() + target_structure.get_mass();
                            let total_distance =
                                source_position.data.distance(target_position.data);
                            let direction =
                                (source_position.data - target_position.data).normalize();
                            let amount = (G * total_mass) / total_distance.powf(2.0);
                            let grav_force = direction * amount;

                            source_force.data -= grav_force * clock.dt * (10.0 as f64).powf(10.0);
                        },
                    )
                },
            )
    }
}
