use criterion::Criterion;
use empty_spaces::engine::{components, types};
use empty_spaces::world;
use rand::{thread_rng, Rng};
use specs::{Builder, Dispatcher, World, WorldExt};

fn setup_world(num_entities: isize) -> (World, Dispatcher<'static, 'static>) {
    let (mut world, dispatcher) = world::create_world();

    let mut rng = thread_rng();
    for _ in 0..num_entities {
        let x = rng.gen_range(-100.0, 100.0);
        let y = rng.gen_range(-100.0, 100.0);
        let vx = rng.gen_range(-1.0, 1.0);
        let vy = rng.gen_range(-1.0, 1.0);
        world
            .create_entity()
            .with(components::Force::new(types::Vector::new(0.0, 0.0)))
            .with(components::Position::new(types::Vector::new(x, y)))
            .with(components::Velocity::new(types::Vector::new(vx, vy)))
            .with(components::Structure::new())
            .with(components::Mass::new(1.0))
            .build();
    }

    return (world, dispatcher);
}

pub fn general(c: &mut Criterion) {
    c.bench_function("general - 100 entities", |b| {
        let (mut world, mut dispatcher) = setup_world(100);
        b.iter(|| {
            dispatcher.dispatch_par(&mut world);
            world.maintain();
        })
    });

    c.bench_function("general - 1000 entities", |b| {
        let (mut world, mut dispatcher) = setup_world(1000);
        b.iter(|| {
            dispatcher.dispatch_par(&mut world);
            world.maintain();
        })
    });
}
