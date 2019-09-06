use super::super::{components, math::collision, types::Vector};
use cgmath::InnerSpace;
use shred::{ResourceId, World};
use specs::{BitSet, Entities, Join, ReadStorage, System, SystemData, WriteStorage};

pub struct Collision {
    killed: BitSet,
}

impl Collision {
    pub fn new() -> Self {
        Self {
            killed: BitSet::new(),
        }
    }
}

#[derive(SystemData)]
pub struct CollisionData<'a> {
    entities: Entities<'a>,
    position: ReadStorage<'a, components::Position>,
    structure: WriteStorage<'a, components::Structure>,
    force: WriteStorage<'a, components::Force>,
    mass: WriteStorage<'a, components::Mass>,
}

impl<'a> System<'a> for Collision {
    type SystemData = CollisionData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (source_entity) in (&data.entities).join() {
            for (target_entity) in (&data.entities).join() {
                // skip self collision
                // skip distances bigger than needed for collision
                if source_entity.id() == target_entity.id()
                // || (tp.data - sp.data).magnitude() > 1.42
                {
                    continue;
                }

                if let (Some(source_position), Some(target_position)) = (
                    data.position.get(source_entity),
                    data.position.get(target_entity),
                ) {
                    let a = collision::get_aabb(
                        &source_position.data,
                        &data
                            .structure
                            .get(source_entity)
                            .unwrap()
                            .get_sections(&source_position.data),
                    );
                    let b = collision::get_aabb(
                        &target_position.data,
                        &data
                            .structure
                            .get(target_entity)
                            .unwrap()
                            .get_sections(&target_position.data),
                    );

                    // check if collision is present
                    if collision::has_collision(&a, &b) {
                        // merge structures

                        let target_structure = data.structure.get(target_entity).unwrap();
                        let target_sections = target_structure.get_sections(&target_position.data);
                        let source_structure = data.structure.get_mut(source_entity).unwrap();

                        if !(self.killed.contains(source_entity.id())
                            || self.killed.contains(target_entity.id()))
                        {
                            source_structure
                                .merge_structure(&source_position.data, target_sections);

                            // source force += target velocity * target mass
                            let target_velocity = data.force.get(target_entity).unwrap().data;
                            let target_mass = data.mass.get(target_entity).unwrap().data;
                            let source_force = data.force.get_mut(source_entity).unwrap();
                            source_force.data += target_velocity * target_mass;

                            let source_mass = data.mass.get_mut(source_entity).unwrap();
                            source_mass.data += target_mass;

                            data.entities.delete(target_entity).unwrap();
                            self.killed.add(target_entity.id());
                        }

                        // resolve collision
                        // let normal = collision::get_collision_normal(&a, &b);
                        // let relative_velocity = target_velocity.data - source_velocity.data;
                        // let normal_vector = relative_velocity.dot(normal);

                        // if normal_vector <= 0.0 {
                        //     let restitution = 0.8;

                        //     let impulse_scalar = (-(1.0 + restitution) * normal_vector) / 2.0;
                        //     let impulse = normal * impulse_scalar;

                        //     // apply forces to resolve collision
                        //     if let Some(source_velocity_mut) = data.velocity.get_mut(source_entity)
                        //     {
                        //         source_velocity_mut.data -= impulse;
                        //     }

                        //     if let Some(target_velocity_mut) = data.velocity.get_mut(target_entity)
                        //     {
                        //         target_velocity_mut.data += impulse;
                        //     }
                        // }
                    }
                }
            }
        }
    }
}
