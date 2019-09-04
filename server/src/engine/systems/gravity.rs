use super::super::{components, resources};
use cgmath::{InnerSpace, MetricSpace};
use shred::{ResourceId, World};
use specs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

const G: f64 = 6.67e-11f64;

pub struct Gravity;

#[derive(SystemData)]
pub struct GravityData<'a> {
    clock: Read<'a, resources::Clock>,
    position: ReadStorage<'a, components::Position>,
    mass: ReadStorage<'a, components::Mass>,
    force: WriteStorage<'a, components::Force>,
}

impl<'a> System<'a> for Gravity {
    type SystemData = GravityData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (source_mass, source_position) in (&data.mass, &data.position).join() {
            for (target_mass, target_position, target_force) in
                (&data.mass, &data.position, &mut data.force).join()
            {
                // grav_force = (G * total_mass) / total_distance ^ 2
                // https://www.wikihow.com/Calculate-Force-of-Gravity

                if source_mass.data == target_mass.data {
                    continue;
                }

                let total_mass = source_mass.data + target_mass.data;
                let total_distance = source_position.data.distance(target_position.data);

                if total_distance <= 0.2 {
                    continue;
                }

                let grav_amount = (G * total_mass) / total_distance.powf(2.0);
                let grav_force =
                    (source_position.data - target_position.data).normalize() * grav_amount;

                target_force.data += grav_force * data.clock.dt;
            }
        }
    }
}
