use fps_clock::FpsClock;
use specs::WorldExt;
use starpixel::{engine::resources, world};

fn main() {
    let (mut world, mut dispatcher) = world::create_world();

    let mut fps = FpsClock::new(30);
    loop {
        world.write_resource::<resources::Clock>().dt = (fps.tick() / 1e+9) as f64 * 500.0;

        dispatcher.dispatch_par(&mut world);
        world.maintain();
    }
}
