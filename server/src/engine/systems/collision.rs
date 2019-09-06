use super::super::{components, types::Vector};
use cgmath::InnerSpace;
use shred::{ResourceId, World};
use specs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage};

pub struct Collision;

const SIZE: f64 = 1.0;

#[derive(Debug)]
struct AABB {
    position: Vector,
    min: Vector,
    max: Vector,
}

#[derive(Debug)]
struct Manifold {
    normal: Vector,
    penetration: f64,
}

struct Resolver {}

impl Resolver {
    fn get_aabb(p: &Vector) -> AABB {
        AABB {
            position: p.clone(),
            min: Vector::new(p.x - SIZE / 2.0, p.y - SIZE / 2.0),
            max: Vector::new(p.x + SIZE / 2.0, p.y + SIZE / 2.0),
        }
    }

    fn has_collision(a: &AABB, b: &AABB) -> bool {
        if a.max.x < b.min.x || a.min.x > b.max.x {
            return false;
        }
        if a.max.y < b.min.y || a.min.y > b.max.y {
            return false;
        }

        return true;
    }

    fn get_manifold(a: &AABB, b: &AABB) -> Option<Manifold> {
        let normal = b.position - a.position;

        // Calculate half extents along x axis for each object
        let a_extent = (a.max.x - a.min.x) / 2.0;
        let b_extent = (b.max.x - b.min.x) / 2.0;

        let x_overlap = a_extent + b_extent - normal.x.abs();

        if x_overlap > 0.0 {
            let a_extent = (a.max.y - a.min.y) / 2.0;
            let b_extent = (b.max.y - b.min.y) / 2.0;

            let y_overlap = a_extent + b_extent - normal.y.abs();

            if y_overlap > 0.0 {
                if x_overlap < y_overlap {
                    return Some(Manifold {
                        penetration: x_overlap,
                        normal: if normal.x < 0.0 {
                            Vector::new(-1.0, 0.0)
                        } else {
                            Vector::new(0.0, 0.0)
                        },
                    });
                } else {
                    return Some(Manifold {
                        penetration: y_overlap,
                        normal: if normal.y < 0.0 {
                            Vector::new(0.0, -1.0)
                        } else {
                            Vector::new(0.0, 1.0)
                        },
                    });
                }
            }
        }

        None
    }
}

#[derive(SystemData)]
pub struct CollisionData<'a> {
    entities: Entities<'a>,
    position: ReadStorage<'a, components::Position>,
    velocity: WriteStorage<'a, components::Velocity>,
}

impl<'a> System<'a> for Collision {
    type SystemData = CollisionData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (source_entity, sp) in (&data.entities, &data.position).join() {
            for (target_entity, tp) in (&data.entities, &data.position).join() {
                // skip self collision
                // skip distances bigger than needed for collision
                if source_entity.id() == target_entity.id()
                    || (tp.data - sp.data).magnitude() > 1.42
                {
                    continue;
                }

                if let (
                    Some(source_position),
                    Some(target_position),
                    Some(source_velocity),
                    Some(target_velocity),
                ) = (
                    data.position.get(source_entity),
                    data.position.get(target_entity),
                    data.velocity.get(source_entity),
                    data.velocity.get(target_entity),
                ) {
                    let a = Resolver::get_aabb(&source_position.data);
                    let b = Resolver::get_aabb(&target_position.data);

                    if Resolver::has_collision(&a, &b) {
                        if let Some(manifold) = Resolver::get_manifold(&a, &b) {
                            // resolve collision

                            let relative_velocity = target_velocity.data - source_velocity.data;
                            let normal_vector = relative_velocity.dot(manifold.normal);

                            if normal_vector <= 0.0 {
                                let restitution = 0.8;

                                let impulse_scalar = (-(1.0 + restitution) * normal_vector) / 2.0;
                                let impulse = manifold.normal * impulse_scalar;

                                if let Some(source_velocity_mut) =
                                    data.velocity.get_mut(source_entity)
                                {
                                    source_velocity_mut.data -= impulse;
                                }

                                if let Some(target_velocity_mut) =
                                    data.velocity.get_mut(target_entity)
                                {
                                    target_velocity_mut.data += impulse;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
