use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

pub fn create_image(images: &mut ResMut<Assets<Image>>, dimensions: Vec2) -> Handle<Image> {
    let size = Extent3d {
        width: (dimensions.x) as u32,
        height: (dimensions.y) as u32,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    return images.add(image);
}
pub fn create_render_camera(commands: &mut Commands, image: &Handle<Image>, layer: RenderLayers) {
    commands.spawn((
        Transform::IDENTITY,
        Camera3d::default(),
        Camera {
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
        layer,
    ));
}
pub fn render_image(
    commands: &mut Commands,
    image: &Handle<Image>,
    layer: RenderLayers,
    scale: f32,
) {
    commands.spawn((
        Sprite {
            image: image.clone(),
            ..Default::default()
        },
        layer,
        Transform::IDENTITY.with_scale(Vec3::splat(scale)),
        Name::new("RenderTarget"),
    ));
}
pub fn create_final_camera(commands: &mut Commands, layer: RenderLayers, scale: f32) -> Entity {
    return commands
        .spawn((
            Msaa::Off,
            UiAntiAlias::Off,
            Transform::IDENTITY,
            Camera2d,
            Camera {
                // render before the "main pass" camera
                order: 0,
                ..default()
            },
            Projection::from(OrthographicProjection {
                scale: scale,
                far: -1000.,
                near: 1000.,
                ..OrthographicProjection::default_2d()
            }),
            layer,
        ))
        .id();
}
// pub fn create_effect<T: Asset + Material>(
//     commands: &mut Commands,
//     images: &mut ResMut<Assets<Image>>,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     dimensions: Vec2,
//     ratio: f32,
//     layer: usize,
//     material: Handle<T>,
// ) -> Handle<Image> {
//     let image = create_image(images, dimensions, ratio);
//     commands.spawn((
//         MaterialMeshBundle {
//             mesh: meshes.add(Mesh::from(Rectangle::new(
//                 dimensions.x / ratio,
//                 dimensions.y / ratio,
//             ))),
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             material: material,
//             ..default()
//         },
//         RenderLayers::layer(layer),
//     ));
//     create_render_camera(commands, &image, layer);
//     return image;
// }
pub fn layer_effects() {}
