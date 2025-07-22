use bevy::{prelude::*, text::TextBounds};

use crate::game::{
    data::data::Data, loading::loading::AssetManager, player::player::PlayerStats,
    scene::internal::bullet_board::BulletBoard, state::state::AppState,
};

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStatsBox>()
            .add_systems(Update, update_health_bar.run_if(in_state(AppState::Level)))
            .add_systems(
                FixedUpdate,
                (update_health_bar, update_hp_text, update_name).run_if(in_state(AppState::Level)),
            )
            .add_systems(OnEnter(AppState::Level), spawn_stats);
    }
}
#[derive(Component, Default)]
pub enum HealthBar {
    #[default]
    Green,
    Red,
}

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct PlayerStatsText;

#[derive(Resource, Default)]
pub struct PlayerStatsBox {
    pub box_size: Vec2,
    pub box_position: Vec2,
}
pub fn spawn_stats(
    mut commands: Commands,
    mut player_stats_box: ResMut<PlayerStatsBox>,
    asset_manager: Res<AssetManager>,
    bullet_box: Res<BulletBoard>,
    player_stats: Res<PlayerStats>,
) {
    let lvl_font = TextFont {
        font: asset_manager.fonts["fonts/Mars_Needs_Cunnilingus.ttf"].clone(),
        font_size: 24.0,
        ..Default::default()
    };

    let box_size = Vec2::new(570.0 + bullet_box.border * 2.0, 42.0);
    let box_position = Vec2::new(0.0, -145.0 - box_size.y / 2.0 - bullet_box.border);

    player_stats_box.box_size = box_size;
    player_stats_box.box_position = box_position;

    let hp_font = TextFont {
        font: asset_manager.fonts["fonts/8-BIT WONDER.ttf"].clone(),
        font_size: 12.0,
        ..Default::default()
    };

    let healthbar_width = 1.0 + player_stats.max_health as f32 * 1.2;

    commands
        .spawn((
            Sprite::from_color(Color::srgba(0.1, 0.1, 0.1, 0.0), box_size),
            Transform::from_translation(box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("BP   LV 1"),
                lvl_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextBounds::from(box_size),
                Transform::from_translation(Vec2::new(0., -box_size.y / 2.0 + 13.0).extend(0.0)),
                Name::new("NAME"),
                PlayerStatsText,
            ));

            builder.spawn((
                Text2d::new("HP"),
                hp_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextBounds::from(box_size),
                Transform::from_translation(Vec2::new(214., -box_size.y / 2.0 + 9.0).extend(0.0)),
                Name::new("HP"),
            ));

            builder.spawn((
                Text2d::new("20 / 20"),
                lvl_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextBounds::from(box_size),
                Transform::from_translation(
                    Vec2::new(245. + healthbar_width + 14., -box_size.y / 2.0 + 13.0).extend(0.0),
                ),
                Name::new("HPNUM"),
                HealthText,
            ));
        });

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(
            Vec2::new(
                245. + healthbar_width / 2.0 - box_size.x / 2.0,
                box_position.y,
            )
            .extend(0.0),
        )
        .with_scale(Vec2::new(healthbar_width, 21.0).extend(1.0)),
        HealthBar::Red,
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 1.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(
            Vec2::new(
                245. + healthbar_width / 2.0 - box_size.x / 2.0,
                box_position.y,
            )
            .extend(1.0),
        )
        .with_scale(Vec2::new(healthbar_width, 21.0).extend(1.0)),
        HealthBar::Green,
    ));
}

fn update_health_bar(
    mut health_bar_query: Query<(&mut HealthBar, &mut Transform)>,
    player_stats_box: Res<PlayerStatsBox>,
    player_stats: Res<PlayerStats>,
) {
    let box_size = player_stats_box.box_size;
    for (mut h, mut t) in health_bar_query.iter_mut() {
        match *h {
            HealthBar::Green => {
                let healthbar_width = 1.0 + 1.2 * player_stats.health as f32;
                t.translation.x = 245. + healthbar_width / 2.0 - box_size.x / 2.0;
            }
            HealthBar::Red => {
                let healthbar_width = 1.0 + 1.2 * player_stats.max_health as f32;
                t.translation.x = 245. + healthbar_width / 2.0 - box_size.x / 2.0;
            }
        }
    }
}

fn update_name(
    mut writer: Text2dWriter,
    mut name_query: Query<(Entity), With<PlayerStatsText>>,
    data: Res<Data>,
) {
    if let Ok(e) = name_query.single() {
        *writer.text(e, 0) = data.game.player.name.clone() + "   " + "LV 1";
    }
}

fn update_hp_text(
    mut writer: Text2dWriter,
    mut hp_query: Query<(Entity), With<HealthText>>,
    player_stats: Res<PlayerStats>,
) {
    if let Ok(e) = hp_query.single() {
        *writer.text(e, 0) =
            player_stats.health.to_string() + " / " + player_stats.max_health.to_string().as_str();
    }
}
