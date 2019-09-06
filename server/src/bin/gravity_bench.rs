#![feature(test)]
extern crate test;

use empty_spaces::engine::{components, resources, systems, types};
use empty_spaces::world;
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};

pub fn create_world() -> (World, Dispatcher<'static, 'static>) {
    // create world
    let mut world = World::new();

    // register components
    world.register::<components::Position>();
    world.register::<components::Force>();
    world.register::<components::Mass>();

    // add resources
    world.insert(resources::Clock::new(1.0));

    // create dispatcher with systems
    let dispatcher = DispatcherBuilder::new()
        .with(systems::Gravity, "gravity", &[])
        .build();

    return (world, dispatcher);
}

#[bench]
fn bench_10_entities(b: &mut test::Bencher) {
    let (mut world, mut dispatcher) = create_world();

    for x in 0..10 {
        for y in 0..1 {
            world
                .create_entity()
                .with(components::Force {
                    data: types::Vector::new(0.0, 0.0),
                })
                .with(components::Position {
                    data: types::Vector::new((x as f64) * 10.0, (y as f64) * 10.0),
                })
                .with(components::Mass { data: 1.0 })
                .build();
        }
    }

    b.iter(|| {
        dispatcher.dispatch(&mut world);
    });
}

#[bench]
fn bench_100_entities(b: &mut test::Bencher) {
    let (mut world, mut dispatcher) = create_world();

    for x in 0..10 {
        for y in 0..10 {
            world
                .create_entity()
                .with(components::Force {
                    data: types::Vector::new(0.0, 0.0),
                })
                .with(components::Position {
                    data: types::Vector::new((x as f64) * 10.0, (y as f64) * 10.0),
                })
                .with(components::Mass { data: 1.0 })
                .build();
        }
    }

    b.iter(|| {
        dispatcher.dispatch(&mut world);
    });
}

#[bench]
fn bench_1000_entities(b: &mut test::Bencher) {
    let (mut world, mut dispatcher) = create_world();

    for x in 0..10 {
        for y in 0..100 {
            world
                .create_entity()
                .with(components::Force {
                    data: types::Vector::new(0.0, 0.0),
                })
                .with(components::Position {
                    data: types::Vector::new((x as f64) * 10.0, (y as f64) * 10.0),
                })
                .with(components::Mass { data: 1.0 })
                .build();
        }
    }

    b.iter(|| {
        dispatcher.dispatch(&mut world);
    });
}
