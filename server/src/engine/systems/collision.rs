use super::super::{components, math::collision};
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
    velocity: WriteStorage<'a, components::Velocity>,
    force: WriteStorage<'a, components::Force>,
    mass: WriteStorage<'a, components::Mass>,
}

impl<'a> System<'a> for Collision {
    type SystemData = CollisionData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        self.killed.clear();
        for (source_entity, source_position) in (&data.entities, &data.position).join() {
            for (target_entity, target_position) in (&data.entities, &data.position).join() {
                // skip self collision
                if source_entity.id() == target_entity.id() {
                    continue;
                }

                // skip if distance is bigger than possible collision distance
                let distance = (target_position.data - source_position.data).magnitude();
                if let (Some(source_structure), Some(target_structure)) = (
                    data.structure.get(source_entity),
                    data.structure.get(target_entity),
                ) {
                    let total_sections =
                        source_structure.sections.len() + target_structure.sections.len();
                    if distance > total_sections as f64 {
                        continue;
                    }
                }

                // calculate aabb-s
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

                // skip if collision is impossible
                if !collision::might_collide(&a, &b) {
                    continue;
                }

                // skip if collision doesn't happen
                if !collision::has_collision(
                    &data
                        .structure
                        .get(source_entity)
                        .unwrap()
                        .get_sections(&source_position.data),
                    &data
                        .structure
                        .get(target_entity)
                        .unwrap()
                        .get_sections(&target_position.data),
                ) {
                    continue;
                }

                // skip if already killed
                if self.killed.contains(source_entity.id())
                    || self.killed.contains(target_entity.id())
                {
                    continue;
                }
                // resolve collision

                // merge structures
                let target_structure = data.structure.get(target_entity).unwrap();
                let target_sections = target_structure.get_sections(&target_position.data);
                let source_structure = data.structure.get_mut(source_entity).unwrap();
                source_structure.merge_structure(&source_position.data, target_sections);

                // get velocity and mass
                let target_velocity = data.velocity.get(target_entity).unwrap().data;
                let target_mass = data.mass.get(target_entity).unwrap().data;

                // transfer forces
                let source_force = data.force.get_mut(source_entity).unwrap();
                let source_velocity = data.velocity.get_mut(source_entity).unwrap();
                let source_mass = data.mass.get_mut(source_entity).unwrap();
                source_force.data =
                    source_velocity.data * source_mass.data + target_velocity * target_mass;
                source_velocity.data *= 0.0;
                source_mass.data += target_mass;

                // delete second entity
                data.entities.delete(target_entity).unwrap();
                self.killed.add(target_entity.id());
            }
        }
    }
}
