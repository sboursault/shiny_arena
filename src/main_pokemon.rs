

use bevy::prelude::*;

mod core;


fn main() {

    // let pokemons = core::repository::get_all();

    // println!("got {} pokemons", pokemons.len());

    // for i in 20..25 {
    //     println!("{} - {}", pokemons[i].id, pokemons[i].name.french);
    // }


    App::build()
    .insert_resource(WindowDescriptor {
        title: "Platformer!".to_string(),
        width: 640.0,
        height: 400.0,
        vsync: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .run();



}




