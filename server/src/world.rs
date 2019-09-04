use super::engine::{components, resources, systems, types};
use rand::{thread_rng, Rng};
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
        .with(systems::Force, "force", &[])
        .with(systems::Velocity, "velocity", &[])
        .with(systems::Sync::new("127.0.0.1:8000"), "sync", &[])
        .build();

    let mut rng = thread_rng();

    // add test entities
    for _ in 0..1000 {
        let x = rng.gen_range(-0.5, 0.5);
        let y = rng.gen_range(-0.5, 0.5);
        let mass = rng.gen_range(5.0, 15.0);

        world
            .create_entity()
            .with(components::Force {
                data: types::Vector::new(0.0, 0.0),
            })
            .with(components::Velocity {
                data: types::Vector::new(0.0, 0.0025),
            })
            .with(components::Position {
                data: types::Vector::new(4.0 + x, 1.3 + y),
            })
            .with(components::Mass { data: mass })
            .build();
    }

    world
        .create_entity()
        .with(components::Force {
            data: types::Vector::new(0.0, 0.0),
        })
        .with(components::Velocity {
            data: types::Vector::new(0.0, 0.0),
        })
        .with(components::Position {
            data: types::Vector::new(3.0, 1.5),
        })
        .with(components::Mass { data: 1000000.0 })
        .build();

    // world
    //     .create_entity()
    //     .with(components::Force {
    //         data: types::Vector::new(0.0, 0.0),
    //     })
    //     .with(components::Velocity {
    //         data: types::Vector::new(0.0, 0.0),
    //     })
    //     .with(components::Position {
    //         data: types::Vector::new(1.2, 2.0),
    //     })
    //     .with(components::Mass { data: 200000.0 })
    //     .build();

    // for _ in 0..50 {
    //     let x = rng.gen_range(0.00, 0.1);
    //     let y = rng.gen_range(0.00, 0.2);

    //     world
    //         .create_entity()
    //         .with(components::Force {
    //             data: types::Vector::new(0.0, 0.0),
    //         })
    //         .with(components::Velocity {
    //             data: types::Vector::new(-0.002, 0.0),
    //         })
    //         .with(components::Position {
    //             data: types::Vector::new(1.0, 2.2 + y),
    //         })
    //         .with(components::Mass { data: 10.0 })
    //         .build();
    // }

    return (world, dispatcher);
}
