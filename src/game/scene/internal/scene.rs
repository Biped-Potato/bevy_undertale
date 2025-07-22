use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::{
        bloom::Bloom,
        tonemapping::{DebandDither, Tonemapping},
    },
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::{
        camera::ScalingMode,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    window::{Monitor, PrimaryMonitor, PrimaryWindow, WindowMode, WindowResolution},
};

use crate::game::{
    camera::{
        render_layers::RenderLayerStorage,
        target::{create_final_camera, create_image, render_image},
    },
    scene::{battle::BattlePlugin, internal::menu::MenuPlugin},
    state::state::AppState,
};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BattlePlugin).add_systems(Startup, setup);
    }
}
#[derive(Resource)]
pub struct Platform {
    pub web: bool,
}

#[derive(Resource)]
pub struct Resolution {
    pub game_res: Vec2,
}
const WINDOW_SIZE: Vec2 = Vec2::new(640., 480.);

fn setup(
    mut state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    render_layers: Res<RenderLayerStorage>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    monitor_query: Query<(&Monitor), With<PrimaryMonitor>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut platform: ResMut<Platform>,
) {
    let mut window = window_query.single_mut().unwrap();
    let mut window_size = Vec2::new(window.width(), window.height());
    if !platform.web {
        log::info!("web detected");
        let monitor = monitor_query.single().unwrap();

        window.resolution = WindowResolution::new(
            monitor.physical_width as f32,
            monitor.physical_height as f32,
        );
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
        log::info!(
            "monitor dimensions {} {}",
            monitor.physical_width,
            monitor.physical_height
        );

        window_size = Vec2::new(
            monitor.physical_width as f32,
            monitor.physical_height as f32,
        );
    } else {
        window.resolution = WindowResolution::new(640 as f32, 480 as f32);
        window.mode = WindowMode::Windowed;
        window_size = Vec2::new(640.0, 480.0);
    }

    let dimensions = Vec2::new(WINDOW_SIZE.x, WINDOW_SIZE.y);
    commands.insert_resource(Resolution {
        game_res: dimensions,
    });

    let mut screen_ratio = window_size.y / dimensions.y;

    let mut image = create_image(&mut images, dimensions);
    //original camera
    let main_camera = commands
        .spawn((
            Camera2d,
            Transform::IDENTITY,
            Camera {
                hdr: true,
                // render before the "main pass" camera
                order: 0,
                target: image.clone().into(),
                ..default()
            },
            Projection::from(OrthographicProjection {
                scale: 1.0,
                far: -1000.,
                near: 1000.,
                ..OrthographicProjection::default_2d()
            }),
            Tonemapping::None,
            Bloom {
                intensity: 0.0,
                ..Default::default()
            },
            Msaa::Off,
            render_layers.pre.clone(),
        ))
        .id();
    render_image(&mut commands, &image, render_layers.downscaled.clone(), 1.0);
    let final_camera = create_final_camera(
        &mut commands,
        render_layers.downscaled.clone(),
        1.0 / screen_ratio,
    );
}
