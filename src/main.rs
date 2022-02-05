use bevy::prelude::*;
mod player;
mod components;
mod camera;
mod sh_menu;
use sh_menu::MainMenuPlugin;


// setup system
fn setup(mut commands: Commands) {
    commands.spawn_bundle(camera::new_camera_2d());
}



// system
fn main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    
    if *app_state.current() == AppState::MainMenu {
        if keys.just_pressed(KeyCode::Return) {
            app_state.set(AppState::InGame).unwrap();
            keys.reset(KeyCode::Return);  // workaround for https://github.com/bevyengine/bevy/issues/1700
        }
    } else {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);  // workaround for https://github.com/bevyengine/bevy/issues/1700
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}



fn main() {
    App::build()
        
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(MainMenuPlugin)
        .add_system(main_menu_controls.system())
        .run();
}

