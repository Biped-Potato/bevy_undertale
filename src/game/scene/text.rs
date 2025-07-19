use bevy::{prelude::*, text::TextBounds};

use crate::game::{camera::render_layers::RenderLayerStorage, loading::loading::AssetManager, scene::bullet_board::{spawn_bullet_board, BulletBoard, BulletBoardFill}, state::state::AppState};


pub struct TextBoxPlugin;
impl Plugin for TextBoxPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<TextMode>()
        .add_systems(OnEnter(AppState::Level),spawn_text.after(spawn_bullet_board));
    }
}

#[derive(Resource,Default)]
pub enum TextMode{
    #[default]
    Static,
    Decisions,
    None
}

#[derive(Component)]
pub struct TextBoxText;
fn spawn_text(
    mut commands : Commands,
    mut bullet_board : ResMut<BulletBoard>,
    asset_manager : Res<AssetManager>,
    render_layers : Res<RenderLayerStorage>,
) {
    // let mut scale = Vec2::new(bullet_board.width,bullet_board.height);
    // let mut position = bullet_board.position;
    let text_font = TextFont {
        font: asset_manager.fonts["fonts/DTM-Mono.ttf"].clone(),
        font_size: 26.0,
        font_smoothing : bevy::text::FontSmoothing::None,
        ..Default::default()
    };
    let mut pos = Vec2::new(14.1, -16.)+bullet_board.position;
    commands.spawn((
        Text2d::new("* You encountered the Dummy."),
        TextBounds::from(Vec2::new(bullet_board.width,bullet_board.height)),
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        Name::new("text"),
        text_font,
        Transform::from_translation((pos).extend(1.0)),
        TextBoxText,
    )).id();
}