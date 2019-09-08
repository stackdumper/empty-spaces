use empty_spaces::engine::{components, resources, types};
use empty_spaces::world;
use fps_clock::FpsClock;
use rand::{thread_rng, Rng};
use specs::{Builder, WorldExt};

fn main() {
    let (mut world, mut dispatcher) = world::create_world();

    let mut rng = thread_rng();

    for _ in 0..30 {
        for _ in 0..30 {
            let x = rng.gen_range(-100.0, 100.0);
            let y = rng.gen_range(-100.0, 100.0);
            let vx = rng.gen_range(-3.0, 3.0);
            let vy = rng.gen_range(-3.0, 3.0);

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

    let mut fps = FpsClock::new(30);
    loop {
        world.write_resource::<resources::Clock>().dt = (fps.tick() / 1e+9) as f64 * 10.0;

        dispatcher.dispatch_par(&mut world);
        world.maintain();
    }
}
