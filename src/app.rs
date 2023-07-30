use bevy::prelude::*;

pub mod entity;
use self::entity::player::PlayerPlugin;
use entity::camera::*;
use entity::game_entity::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_startup_system(spawn_game_entities);
    }
}

fn spawn_game_entities(mut command: Commands) {
    command.spawn(GameEntity::default());
}
