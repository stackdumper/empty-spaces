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
    world.register::<components::Structure>();

    // add resources
    world.insert(resources::Clock::new(1.0));

    // create dispatcher with systems
    let dispatcher = DispatcherBuilder::new()
        .with(systems::Gravity, "gravity", &[])
        .with(systems::Force, "force", &[])
        .with(systems::Velocity, "velocity", &[])
        .with(systems::Collision::new(), "collision", &[])
        .with(systems::Sync::new("127.0.0.1:8000"), "sync", &[])
        .build();

    return (world, dispatcher);
}
