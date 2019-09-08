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
                    (&entity, &position, &structure).join().for_each(
                        |(target_entity, target_position, target_structure)| {
                            if source_entity.id() == target_entity.id() {
                                return;
                            }

                            let source_center_of_mass =
                                source_structure.get_center_of_mass(&source_position.data);

                            let target_center_of_mass =
                                target_structure.get_center_of_mass(&target_position.data);

                            let total_mass =
                                source_structure.get_mass() + target_structure.get_mass();
                            let total_distance =
                                source_center_of_mass.distance(target_center_of_mass);
                            let direction =
                                (source_center_of_mass - target_center_of_mass).normalize();
                            let amount = (G * total_mass) / total_distance.powf(2.0);
                            let grav_force = direction * amount;

                            source_force.data -= grav_force * clock.dt * (10.0 as f64).powf(10.0);
                        },
                    )
                },
            )
    }
}
