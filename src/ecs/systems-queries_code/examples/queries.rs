use bevy::prelude::*;
use std::fmt;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_startup_system(spawn_readers.system())
        // This system shows the reader's initial education
        .add_startup_system(report_education.system())
        .add_system(educate.system())
        // This system shows it after they've been taught!
        .add_system(report_education.system())
        .run()
}

const N_READERS: usize = 10;
struct Name {
    name: String,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Educated {
    educated: bool,
}

impl fmt::Display for Educated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = if self.educated{"educated"} else {"uneducated"};
        write!(f, "{}", out)
    }
}

fn spawn_readers(commands: &mut Commands) {
    for i in 1..N_READERS {
        let i_string = i.to_string();
        let name = ["Anonymous Reader", &*i_string].join(" ");
        // Because our variable name is the same as our field name,
        // we can elide `Name{name: name}`
        // into `Name{name}`
        commands.spawn((Name { name }, Educated { educated: false }));
    }
}

// We need to access everything mutably 
// so we can change it
fn educate(mut query: Query<&mut Educated>) {
    // Queries are typically iterated over
    // We're changing the values here, so need `iter_mut`
    // and `mut educated`
    for mut educated in query.iter_mut() {
        // In practice, you'll want to speed this up by impl Deref and DerefMut
        // for your structs with one field
        educated.educated = true;
    }
}

// Entity can be included in your queries, because it has the WorldQuery trait
fn report_education(query: Query<(Entity, &Name, &Educated)>) {
    // Use tuple unpacking to access the components in your queries
    for (entity, name, educated) in query.iter() {
        println!(
            "{}, with an entity identifier of {:?}, is {}.",
            name, entity, educated
        );
    }
}
