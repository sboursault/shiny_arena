// Components
struct Person;

struct Name(String);

// Systems
fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

// main
fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .run();
}

// prints: 
// hello Elaina Proctor!
// hello Renzo Hume!
// hello Zayna Nieves!
