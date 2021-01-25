use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt;

// Denizen is a "marker component" here, to allow us to filter for denizens in our queries
// In general, marker components do not store data, and are just used for queries
// Marker components are talked about in more detail in section 2.2: Components
#[derive(Debug)]
struct Denizen;

// We use the owned form String, rather than &str in this struct
// Because resources and components must be thread-safe with a 'static lifetime
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Name(String);

// A custom impl of Display to ensure we can print these names nicely
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
enum World {
    Venus,
    Earth,
    Mars,
}

// Creating a unique timer type for each system that needs one allows us to be sure we're grabbing the right one
struct HelloTimer(Timer);

fn main() {
    App::build()
        // We add the default plugins so our game will loop
        // This also adds the Time resource that we use in `say_hello`
        .add_plugins(DefaultPlugins)
        // We're creating a HashMap to record which planet entities a name are on
        .add_resource(HashMap::<Name, World>::new())
        // Startup systems only run once, before normal systems take place
        .add_startup_system(place_denizens.system())
        // When systems cannot be run in parallel, priority is based on insertion order
        .add_startup_system(spawn_denizens.system())
        // The `say_hello` system will be run every frame, but only prints when the timer is complete
        // (See `say_hello` logic)
        .add_resource(HelloTimer(Timer::from_seconds(2.0, true)))
        .add_system(say_hello.system())
        .run();
}

// Because we're modifying the `directory` argument, we need to get the mutable version of it with `ResMut`
// Bevy's ECS finds a Resource with the matching type; we want to be sure we have exactly one resource of each type that we need
// Storing this list in a HashMap resource allows us to quickly look up the relationships,
// rather than having to search through each query every time
fn place_denizens(mut directory: ResMut<HashMap<Name, World>>) {
    // .into() converts our string literal from &str to the required String
    directory.insert(Name("Alice".into()), World::Venus);
    directory.insert(Name("Bevy".into()), World::Earth);
    directory.insert(Name("Cart".into()), World::Mars);
}

// The special `Commands` resource queues up actions that should be performed to modify the World
// We only need to read from the directory resource, so we can call it with `Res` instead of `ResMut`
fn spawn_denizens(commands: &mut Commands, directory: Res<HashMap<Name, World>>) {
    // We need to use .clone and .into_iter rather than .iter here
    // to satisfy the lifetime requirements of .spawn()
    for (name, world) in directory.clone().into_iter() {
        if name == Name("Bevy".into()) {
            // .spawn creates new entities with the specified components
            // each tuple entry is a different component
            commands.spawn((name, world));
        } else {
            // Only Alice and Cart are Denizens, so Bevy will be excluded from our query filter in `say_hello`
            commands.spawn((name, world, Denizen));
        }
    }
}

// Queries extract each entity that have all of the components specified in their first type argument
// They only return the components specified in the query, not any other components that may be associated with the entities
// The second type argument is a query filter, which restricts which entities are actually provided
fn say_hello(
    query: Query<(&Name, &World), With<Denizen>>,
    mut timer: ResMut<HelloTimer>,
    time: Res<Time>,
) {
    // Only run this system when the timer has elapsed
    if timer.0.tick(time.delta_seconds()).just_finished() {
        // Iterating over and then unpacking the query gives us access to the components for each of its entities
        for (name, world) in query.iter() {
            // Because we're querying for &Query and &World, we need to dereference them before we work with them
            println!("Hello {:?}, my name is {}!", *world, *name);
        }
    }
}
