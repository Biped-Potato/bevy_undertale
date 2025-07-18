use bevy::{prelude::*, render::view::RenderLayers};
#[derive(Resource)]
pub struct RenderLayerStorage {
    pub pre: RenderLayers,
    pub post: RenderLayers,
    pub downscaled: RenderLayers,
}

pub struct RenderLayersPlugin;
impl Plugin for RenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, initialize_render_layers);
    }
}
fn initialize_render_layers(mut commands: Commands) {
    let pre = RenderLayers::layer(0);
    let post = RenderLayers::layer(1);
    let downscaled = RenderLayers::layer(2);
    commands.insert_resource(RenderLayerStorage {
        pre: pre,
        post: post,
        downscaled: downscaled,
    });
}
