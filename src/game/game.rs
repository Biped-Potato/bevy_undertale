use crate::game::{
    animation::animation::AtlasAnimationPlugin, camera::render_layers::RenderLayersPlugin, data::data::DataPlugin, loading::loading::AssetManagerPlugin, scene::scene::ScenePlugin
};
use bevy::prelude::*;

use super::player::player::PlayerPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DataPlugin, AssetManagerPlugin))
            .add_plugins(PlayerPlugin)
            .add_plugins(RenderLayersPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(AtlasAnimationPlugin);
    }
}
