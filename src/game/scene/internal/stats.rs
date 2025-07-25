use bevy::{prelude::*, text::TextBounds};

use crate::game::{
    data::data::Data,
    loading::loading::AssetManager,
    player::player::PlayerStats,
    scene::internal::{bullet_board::BulletBoard, helpers::menu_item::MenuItem},
    state::state::AppState,
};

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStatsBox>()
            .add_systems(Update, update_health_bar.run_if(in_state(AppState::Level)))
            .add_systems(
                FixedUpdate,
                (
                    update_player_health_bar,
                    update_health_bar,
                    update_hp_text,
                    update_name,
                )
                    .run_if(in_state(AppState::Level)),
            )
            .add_systems(OnEnter(AppState::Level), spawn_stats);
    }
}
#[derive(Component, Default, PartialEq)]
pub enum HealthBarType {
    #[default]
    Green,
    Red,
}
#[derive(Component)]
pub struct HealthBar {
    pub enemy_bar: bool,
    pub health: i32,
    pub max_health: i32,
    pub position: Vec2,
    pub custom_size: Option<IVec2>,
    pub center: bool,
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

#[derive(Component)]
pub struct PlayerHealthBar {}
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
            MenuItem,
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

    let healthbar_position = Vec2::new(245. - box_size.x / 2.0, box_position.y);

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(healthbar_position.extend(0.0))
            .with_scale(Vec2::new(healthbar_width, 21.0).extend(1.0)),
        HealthBarType::Red,
        HealthBar {
            custom_size: None,
            enemy_bar: false,
            position: healthbar_position,
            health: 0,
            max_health: 0,
            center: false,
        },
        PlayerHealthBar {},
        MenuItem,
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 1.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(healthbar_position.extend(1.0))
            .with_scale(Vec2::new(healthbar_width, 21.0).extend(1.0)),
        HealthBarType::Green,
        HealthBar {
            custom_size: None,
            enemy_bar: false,
            position: healthbar_position,
            health: 0,
            max_health: 0,
            center: false,
        },
        PlayerHealthBar {},
        MenuItem
    ));
}

impl HealthBar {
    pub fn get_size_x(&mut self, amount: i32) -> f32 {
        let mut healthbar_width = 1.0 + 1.2 * amount as f32;
        if self.enemy_bar {
            healthbar_width = 1.0 + 100.0 * amount as f32 / self.max_health as f32;
        }
        if self.custom_size.is_some() {
            healthbar_width =
                self.custom_size.unwrap().x as f32 * amount as f32 / self.max_health as f32;
        }
        return healthbar_width;
    }
}
fn update_health_bar(
    mut health_bar_query: Query<(&mut HealthBarType, &mut HealthBar, &mut Transform)>,
    player_stats_box: Res<PlayerStatsBox>,
    player_stats: Res<PlayerStats>,
) {
    let box_size = player_stats_box.box_size;
    for (mut h_t, mut h, mut t) in health_bar_query.iter_mut() {
        let mut amount = 0;

        match *h_t {
            HealthBarType::Green => {
                amount = h.health;
            }
            HealthBarType::Red => {
                amount = h.max_health;
            }
        }

        let mut healthbar_width = h.get_size_x(amount);

        t.translation.x = h.position.x + healthbar_width / 2.0;

        if h.center {
            let max_h = h.max_health;
            let mut max_size = h.get_size_x(max_h);
            t.translation.x -= max_size / 2.0;
        }
        t.translation.y = h.position.y;
        t.scale.x = healthbar_width;
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

fn update_player_health_bar(
    mut health_bar_query: Query<(&mut HealthBar, &mut Transform, &mut PlayerHealthBar)>,
    player_stats: Res<PlayerStats>,
) {
    for (mut h_t, mut t, mut p) in health_bar_query.iter_mut() {
        h_t.health = player_stats.health;
        h_t.max_health = player_stats.max_health;
    }
}
