use bevy::prelude::*;

use crate::game::{data::data::Data, loading::loading::AssetManager, state::state::AppState};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Level), spawn_player);

    }
}

fn spawn_player(mut commands: Commands, asset_manager: Res<AssetManager>,data : Res<Data>) {
    commands.spawn((
        Sprite {
            image: asset_manager.images[&data.player.sprite.clone()].clone(),
            ..Default::default()
        },

    ));
}
