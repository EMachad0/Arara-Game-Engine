use arara::prelude::*;

fn main() {
    App::new()
        .add_plugin(GreetPeoplePlugin)
        .add_startup_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello World")
}

pub struct GreetPeoplePlugin;

impl Plugin for GreetPeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people).add_system(greet_people);
    }
}

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name {
    name: String,
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name {
        name: "Elaina Proctor".to_string(),
    });
    commands.spawn().insert(Person).insert(Name {
        name: "Renzo Hume".to_string(),
    });
    commands.spawn().insert(Person).insert(Name {
        name: "Zayna Nieves".to_string(),
    });
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.name);
    }
}
