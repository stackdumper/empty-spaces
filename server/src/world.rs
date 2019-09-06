use super::engine::{components, resources, systems, types};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

pub fn create_world() -> (World, Dispatcher<'static, 'static>) {
    // create world
    let mut world = World::new();

    // register components
    world.register::<components::Position>();
    world.register::<components::Force>();
    world.register::<components::Mass>();
    world.register::<components::Velocity>();

    // add resources
    world.insert(resources::Clock::new(1.0));

    // create dispatcher with systems
    let dispatcher = DispatcherBuilder::new()
        .with(systems::Gravity, "gravity", &[])
        .with(systems::Force, "force", &["gravity"])
        .with(systems::Velocity, "velocity", &["force"])
        .with(systems::Collision, "collision", &["velocity"])
        .with(systems::Sync::new("127.0.0.1:8000"), "sync", &["collision"])
        .build();

    for x in 0..3 {
        for y in 0..3 {
            world
                .create_entity()
                .with(components::Force {
                    data: types::Vector::new(0.0, 0.0),
                })
                .with(components::Velocity {
                    data: types::Vector::new(0.0, 0.0),
                })
                .with(components::Position {
                    data: types::Vector::new(1.0 + (x * 5) as f64, 1.0 + (y * 5) as f64),
                })
                .with(components::Mass { data: 1.0 })
                .build();
        }
    }

    return (world, dispatcher);
}
