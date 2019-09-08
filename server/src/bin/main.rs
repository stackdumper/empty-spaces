use empty_spaces::engine::{components, resources, types};
use empty_spaces::world;
use fps_clock::FpsClock;
use rand::{thread_rng, Rng};
use specs::{Builder, WorldExt};

fn main() {
    let (mut world, mut dispatcher) = world::create_world();

    let mut rng = thread_rng();
    for _ in 0..100 {
        for _ in 0..10 {
            let x = rng.gen_range(-1000.0, 1000.0);
            let y = rng.gen_range(-1000.0, 1000.0);
            let vx = rng.gen_range(-1.0, 1.0);
            let vy = rng.gen_range(-1.0, 1.0);

            world
                .create_entity()
                .with(components::Force::new(types::Vector::new(0.0, 0.0)))
                .with(components::Position::new(types::Vector::new(x, y)))
                .with(components::Velocity::new(types::Vector::new(vx, vy)))
                .with(components::Structure::new())
                .build();
        }
    }

    // world
    //     .create_entity()
    //     .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Position::new(types::Vector::new(50.0, 50.0)))
    //     .with(components::Velocity::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Structure::new())
    //     .build();

    // world
    //     .create_entity()
    //     .with(components::Force::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Position::new(types::Vector::new(50.0, 55.0)))
    //     .with(components::Velocity::new(types::Vector::new(0.0, 0.0)))
    //     .with(components::Structure::new())
    //     .build();

    // let mut fps = FpsClock::new(60);
    // loop {
    //     world.write_resource::<resources::Clock>().dt = (fps.tick() / 1e+9) as f64 * 20.0;

    //     dispatcher.dispatch_par(&mut world);
    //     world.maintain();
    // }

    loop {
        dispatcher.dispatch_par(&mut world);
        world.maintain();

        println!("tick");
    }
}
