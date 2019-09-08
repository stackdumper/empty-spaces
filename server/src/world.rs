use super::engine::{components, resources, systems};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

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
        .with(systems::Force, "force", &["gravity"])
        .with(systems::Velocity, "velocity", &["force"])
        .with(systems::Collision::new(), "collision", &["velocity"])
        .with(systems::Sync::new("127.0.0.1:8000"), "sync", &["collision"])
        .build();

    return (world, dispatcher);
}
