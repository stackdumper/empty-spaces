use empty_spaces::{
    engine::{components, resources, types},
    world,
};
use fps_clock::FpsClock;
use rand::{thread_rng, Rng};
use specs::{Builder, WorldExt};

fn main() {
    let (mut world, mut dispatcher) = world::create_world();

    // world
    //     .create_entity()
    //     .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Position::new(types::Vector::new(100.0, 250.0)))
    //     .with(components::Velocity::new(types::Vector::new(0.0, -2.0)))
    //     .with(components::Structure::new())
    //     .with(components::Mass::new(1.0))
    //     .build();

    // world
    //     .create_entity()
    //     .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Position::new(types::Vector::new(100.0, 10.0)))
    //     .with(components::Velocity::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Structure::new())
    //     .with(components::Mass::new(20.0))
    //     .build();

    // world
    //     .create_entity()
    //     .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Position::new(types::Vector::new(25.0, 10.0)))
    //     .with(components::Velocity::new(types::Vector::new(-2.0, 0.0)))
    //     .with(components::Structure::new())
    //     .with(components::Mass::new(1.0))
    //     .build();

    // for x in 0..3 {
    //     for y in 0..3 {
    //         world
    //             .create_entity()
    //             .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //             .with(components::Position::new(types::Vector::new(
    //                 (x as f64) * 2.0,
    //                 (y as f64) * 2.0,
    //             )))
    //             .with(components::Velocity::new(types::Vector::new(0.0, 0.0)))
    //             .with(components::Structure::new())
    //             .with(components::Mass::new(1.0))
    //             .build();
    //     }
    // }

    let mut rng = thread_rng();

    // for _ in 0..30 {
    //     for _ in 0..30 {
    //         let x = rng.gen_range(-100.0, 100.0);
    //         let y = rng.gen_range(-100.0, 100.0);
    //         // let vx = rng.gen_range(-0.1, 0.1);
    //         // let vy = rng.gen_range(-0.1, 0.1);

    //         world
    //             .create_entity()
    //             .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //             .with(components::Position::new(types::Vector::new(x, y)))
    //             .with(components::Velocity::new(types::Vector::new(0.0, 0.0)))
    //             .with(components::Structure::new())
    //             .with(components::Mass::new(2.0))
    //             .build();
    //     }
    // }

    // for _ in 0..20 {
    //     for _ in 0..20 {
    //         let x = rng.gen_range(-50.0, 50.0);
    //         let y = rng.gen_range(-50.0, 50.0);
    //         let vx = rng.gen_range(-1.0, 1.0);
    //         let vy = rng.gen_range(-1.0, 1.0);

    //         world
    //             .create_entity()
    //             .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //             .with(components::Position::new(types::Vector::new(x, y)))
    //             .with(components::Velocity::new(types::Vector::new(vx, vy)))
    //             .with(components::Structure::new())
    //             .with(components::Mass::new(2.0))
    //             .build();
    //     }
    // }

    // for _ in 0..10 {
    //     for _ in 0..10 {
    //         let x = rng.gen_range(-200.0, 200.0);
    //         let y = rng.gen_range(-200.0, 200.0);
    //         let vx = rng.gen_range(-5.0, 5.0);
    //         let vy = rng.gen_range(-5.0, 5.0);

    //         world
    //             .create_entity()
    //             .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //             .with(components::Position::new(types::Vector::new(x, y)))
    //             .with(components::Velocity::new(types::Vector::new(vx, vy)))
    //             .with(components::Structure::new())
    //             .with(components::Mass::new(1.0))
    //             .build();
    //     }
    // }

    for _ in 0..20 {
        for _ in 0..20 {
            let x = rng.gen_range(-200.0, 200.0);
            let y = rng.gen_range(-200.0, 200.0);
            let vx = rng.gen_range(-6.0, 6.0);
            let vy = rng.gen_range(-6.0, 6.0);

            world
                .create_entity()
                .with(components::Force::new(types::Vector::new(0.0, 0.0)))
                .with(components::Position::new(types::Vector::new(x, y)))
                .with(components::Velocity::new(types::Vector::new(vx, vy)))
                .with(components::Structure::new())
                .with(components::Mass::new(1.0))
                .build();
        }
    }

    // let mut fps = FpsClock::new(30);
    // loop {
    //     world.write_resource::<resources::Clock>().dt = (fps.tick() / 1e+9) as f64 * 10.0;

    //     dispatcher.dispatch_par(&mut world);
    //     world.maintain();
    // }

    for _ in 0..50 {
        println!("tick");
        dispatcher.dispatch_par(&mut world);
        world.maintain();
    }
}
