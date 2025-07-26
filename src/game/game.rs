use crate::game::{
    animation::animation::AtlasAnimationPlugin, camera::render_layers::RenderLayersPlugin,
    data::data::DataPlugin, loading::loading::AssetManagerPlugin, physics::physics::PhysicsPlugin,
    scene::internal::scene::ScenePlugin, sound::sound::SoundPlugin,
};
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use super::player::player::PlayerPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_plugins((DataPlugin, AssetManagerPlugin))
            .add_plugins(PlayerPlugin)
            .add_plugins(RenderLayersPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(AtlasAnimationPlugin)
            .add_plugins(PhysicsPlugin)
            .add_plugins(SoundPlugin)
            .add_plugins((
                EguiPlugin {
                    enable_multipass_for_primary_context: true,
                },
                //WorldInspectorPlugin::default(),
            ));
    }
}
