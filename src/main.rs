#![allow(unexpected_cfgs)]
#![allow(unused)]
use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::game::{game::GamePlugin, scene::internal::scene::Platform};

pub mod game;
const AUDIO_SCALE: f32 = 1. / 100.0;
fn main() {
    App::new()
        .insert_resource(Platform { web: false })
        .add_plugins(
            DefaultPlugins
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bevy Undertale"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        present_mode: PresentMode::AutoNoVsync,
                        mode: WindowMode::Windowed,
                        //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(GamePlugin)
        .run();
}
