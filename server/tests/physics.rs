use specs::{Builder, WorldExt};
use starpixel::engine::{components, types::Vector};
use starpixel::world;

#[test]
fn physics() {
    let (mut world, mut dispatcher) = world::create_world();

    let entity1 = world
        .create_entity()
        .with(components::Force {
            data: Vector::new(0.0, 0.0),
        })
        .with(components::Velocity {
            data: Vector::new(0.0, 0.0),
        })
        .with(components::Position {
            data: Vector::new(0.0, 0.0),
        })
        .with(components::Mass { data: 1.0 })
        .build();

    let entity2 = world
        .create_entity()
        .with(components::Force {
            data: Vector::new(0.0, 0.0),
        })
        .with(components::Velocity {
            data: Vector::new(0.0, 0.0),
        })
        .with(components::Position {
            data: Vector::new(1.0, 1.0),
        })
        .with(components::Mass { data: 10000.0 })
        .build();

    for _ in 0..1000 {
        dispatcher.dispatch(&mut world);
    }

    let position = world.read_storage::<components::Position>();

    println!("{:?}", position.get(entity1).unwrap());
    println!("{:?}", position.get(entity2).unwrap());
}
