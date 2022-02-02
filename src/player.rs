use super::components::Player;
use bevy::prelude::*;

use super::AppState;
use super::camera;

pub struct PlayerPlugin;


struct PlayerData {
    player_entity: Entity,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        //app.add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
        // .add_system(player_jumps.system())
        // .add_system(player_movement.system())
        // .add_system(jump_reset.system())

        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(spawn_player.system()),
        )
        //.add_system_set(
        //    SystemSet::on_update(AppState::InGame)
                // .with_system(player_jumps.system())
                // .with_system(player_movement.system())
                // .with_system(jump_reset.system()),
        //)
        //.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(cleanup.system()))
        //.add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup.system()))
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_player.system()));
    }
}


fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let player_entity = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(Player)
        .with_children(|parent| { 
            parent.spawn_bundle(camera::new_camera_2d());
        })
        .id();


    commands.insert_resource(PlayerData { player_entity });
}


fn cleanup_player(mut commands: Commands, player_data: Res<PlayerData>) {
    println!("{:?}", player_data.player_entity);
    commands
        .entity(player_data.player_entity)
        .despawn_recursive();
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        //println!("{:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}
