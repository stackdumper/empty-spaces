use super::super::{components, math::collision::AABB};
use crossbeam::sync::ShardedLock;
use rayon::iter::ParallelIterator;
use specs::{BitSet, Entities, Entity, ParJoin, ReadStorage, System, WriteStorage};
use std::sync::{Arc, RwLock};

pub struct Collision {
    deleted: Arc<ShardedLock<BitSet>>,
    collisions: Arc<RwLock<Vec<(Entity, Entity)>>>,
}

impl Collision {
    pub fn new() -> Self {
        Self {
            deleted: Arc::new(ShardedLock::new(BitSet::new())),
            collisions: Arc::new(RwLock::new(Vec::with_capacity(1024))),
        }
    }
}

impl<'a> System<'a> for Collision {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, components::Position>,
        WriteStorage<'a, components::Structure>,
        WriteStorage<'a, components::Force>,
        WriteStorage<'a, components::Velocity>,
    );

    fn run(
        &mut self,
        (entity, position, mut structure, mut force, mut velocity): Self::SystemData,
    ) {
        // TODO: store pairs of entity.id() and skip if one already exists
        // find collisions
        (&entity, &position, &structure).par_join().for_each(
            |(source_entity, source_position, source_structure)| {
                (&entity, &position, &structure).par_join().for_each(
                    |(target_entity, target_position, target_structure)| {
                        // 1. skip self
                        let source_id = source_entity.id();
                        let target_id = target_entity.id();

                        if source_id == target_id {
                            return;
                        }

                        // 2. skip deleted entities
                        {
                            let deleted = self.deleted.read().unwrap();

                            if deleted.contains(source_id) || deleted.contains(target_id) {
                                return;
                            }
                        }

                        // 3. get sections
                        let source_sections = source_structure.get_sections(&source_position.data);
                        let target_sections = target_structure.get_sections(&target_position.data);

                        // // 4. calculate aabbs
                        let source_aabb = AABB::new_complex(&source_sections);
                        let target_aabb = AABB::new_complex(&target_sections);

                        // 5. check if collision
                        if AABB::collides(&source_aabb, &target_aabb)
                            && AABB::collides_complex(&source_sections, &target_sections)
                        {
                            // 5.5 check if deleted again
                            {
                                let deleted = self.deleted.read().unwrap();

                                if deleted.contains(source_id) || deleted.contains(target_id) {
                                    return;
                                }
                            }

                            // 6. save collision
                            self.collisions
                                .write()
                                .unwrap()
                                .push((source_entity, target_entity));

                            self.deleted.write().unwrap().add(target_id);
                        }
                    },
                )
            },
        );

        // resolve collisions
        for (source_entity, target_entity) in self.collisions.write().unwrap().drain(0..) {
            // 1. merge structures
            let target_position = position.get(target_entity).unwrap();
            let target_structure = structure.get(target_entity).unwrap();
            let target_mass = target_structure.get_mass();
            let target_sections = target_structure.get_sections(&target_position.data);

            let source_position = position.get(source_entity).unwrap();
            let source_structure = structure.get_mut(source_entity).unwrap();
            let source_mass = source_structure.get_mass();
            source_structure.merge_structure(&source_position.data, target_sections);

            // 2. transfer forces
            let source_force = force.get_mut(source_entity).unwrap();
            let target_velocity_data = velocity.get_mut(target_entity).unwrap().data;
            let source_velocity = velocity.get_mut(source_entity).unwrap();
            source_force.data =
                source_velocity.data * source_mass + target_velocity_data * target_mass;
            source_velocity.data *= 0.0;

            // 3. delete target entity
            entity.delete(target_entity).unwrap();
        }
    }
}
