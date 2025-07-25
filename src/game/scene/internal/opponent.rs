use bevy::{prelude::*, text::TextLayoutInfo};

use crate::game::{
    data::data::Data,
    loading::loading::AssetManager,
    physics::physics_object::PhysicsComponent,
    scene::{
        battle::spawn_opponent,
        internal::{
            bullet_board::BulletBoard,
            enemy_health::EnemyHealthBar,
            fight::FightManager,
            helpers::menu_item::MenuItem,
            stats::{HealthBar, HealthBarType},
        },
    },
    state::state::AppState,
};

pub struct OpponentPlugin;
impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OpponentHealthBarManager>()
            .add_systems(
                OnEnter(AppState::Level),
                (spawn_opponent, spawn_opponent_health_ui),
            )
            .add_systems(
                FixedUpdate,
                update_opponent_position
                    .before(update_enemy_healthbar)
                    .run_if(in_state(AppState::Level)),
            )
            .add_systems(
                FixedUpdate,
                (
                    update_enemy_healthbar,
                    update_text_color.before(update_enemy_healthbar),
                )
                    .run_if(in_state(AppState::Level)),
            );
    }
}

#[derive(Resource, Default)]
pub struct OpponentHealthBarManager {
    pub old_health: i32,
    pub new_health: i32,
    pub damage_display: i32,
    pub damage_text: Option<Entity>,
}
#[derive(Component)]
pub struct Opponent {
    pub offset: Vec2,
}

fn update_opponent_position(
    bullet_board: Res<BulletBoard>,
    mut opponent_query: Query<(&mut Opponent, &mut PhysicsComponent)>,
    mut fight_manager: ResMut<FightManager>,
    data: Res<Data>,
) {
    for (mut opponent, mut physics) in opponent_query.iter_mut() {
        physics.position.x = 0.;
        physics.position.y = bullet_board.position.y
            + bullet_board.height / 2.0
            + bullet_board.border
            + 10.0
            + data.game.opponent_data.height * 2.0 / 2.0;
        if fight_manager.strike && !fight_manager.miss {
            if fight_manager.attack_animation <= 1.0 {
                let time = 1.0 - fight_manager.attack_animation;
                let shake_speed = 10.0;
                opponent.offset = Vec2::X
                    * f32::sin(time * 2.0 * 3.14159 * shake_speed)
                    * 10.0
                    * fight_manager.attack_animation;
                physics.position += opponent.offset;
            }
        }
    }
}
fn update_text_color(
    mut fight_manager: ResMut<FightManager>,
    mut text_query: Query<(&mut DamageText, &mut TextColor)>,
) {
    for (mut d, mut t) in text_query.iter_mut() {
        if fight_manager.miss {
            *t = TextColor(Color::srgb(0.75, 0.75, 0.75));
        } else {
            *t = TextColor(Color::srgb(1.0, 0., 0.));
        }
    }
}
fn update_enemy_healthbar(
    mut fight_manager: ResMut<FightManager>,
    mut bar_manager: ResMut<OpponentHealthBarManager>,
    mut opponent_query: Query<(&mut Opponent, &mut PhysicsComponent)>,
    mut bar_query: Query<
        (
            &mut HealthBarType,
            &mut HealthBar,
            &mut OpponentHealthBar,
            &mut Visibility,
            &mut Transform,
        ),
        With<OpponentHealthBar>,
    >,
    mut text_query: Query<
        (
            &mut DamageText,
            &mut Visibility,
            &mut Transform,
            &mut TextLayoutInfo,
            Entity,
        ),
        Without<OpponentHealthBar>,
    >,
    mut writer: Text2dWriter,
    data: Res<Data>,
) {
    if let Ok((mut o, mut physics)) = opponent_query.single_mut() {
        let mut healthbar_height = 0.0;
        let mut healthbar_pos = Vec2::ZERO;
        for (mut h_t, mut h, mut b, mut v, mut t) in bar_query.iter_mut() {
            if fight_manager.strike {
                if fight_manager.attack_animation <= 1.0 || fight_manager.miss {
                    h.max_health = data.game.opponent_data.health;
                    h.position = physics.position - o.offset
                        + Vec2::new(0., data.game.opponent_data.height * 2.0 / 2.0)
                        + Vec2::new(0., h.custom_size.unwrap().y as f32 / 2.0);
                    let diff = bar_manager.old_health - bar_manager.new_health;
                    h.health = (bar_manager.new_health as f32
                        + diff as f32 * (fight_manager.attack_animation))
                        .round() as i32;
                    if !fight_manager.miss {
                        *v = Visibility::Visible;
                    }

                    healthbar_pos = h.position;
                    healthbar_height = t.scale.y;
                }
            } else {
                *v = Visibility::Hidden;
            }
        }
        if let Ok((mut d, mut v, mut t, mut t_i, e)) = text_query.single_mut() {
            if fight_manager.strike {
                if fight_manager.attack_animation <= 1.0 || fight_manager.miss {
                    let mut position = healthbar_pos;

                    if !fight_manager.miss {
                        position += Vec2::new(0., healthbar_height);
                        *writer.text(e, 0) = bar_manager.damage_display.to_string();
                        let time =
                            f32::clamp((1.0 - fight_manager.attack_animation) * 2.0, 0.0, 1.0);
                        position.y -= ((f32::cos(time * 2.0 * 3.14159) - 1.0) / 2.0) * 12.0;
                    } else {
                        *writer.text(e, 0) = "MISS".to_string();
                    }
                    t.translation = position.round().extend(5.0) + Vec3::Y * 0.3;
                    *v = Visibility::Visible;
                }
            } else {
                *v = Visibility::Hidden;
            }
        }
    }
}
#[derive(Component)]
pub struct OpponentHealthBar;

#[derive(Component)]
pub struct DamageText;
fn spawn_opponent_health_ui(
    mut commands: Commands,
    asset_manager: Res<AssetManager>,
    data: Res<Data>,
    mut opponent_health_bar_manager: ResMut<OpponentHealthBarManager>,
    mut b_board: Res<BulletBoard>,
) {
    let mut healthbar_width = 125;
    let pos = Vec2::ZERO;
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(pos.extend(0.0))
            .with_scale(Vec2::new(healthbar_width as f32, 21.0).extend(1.0)),
        HealthBarType::Red,
        HealthBar {
            enemy_bar: false,
            position: pos,
            health: 0,
            max_health: 0,
            custom_size: Some(IVec2::new(healthbar_width, 21)),
            center: true,
        },
        OpponentHealthBar {},
        Visibility::Hidden,
        MenuItem,
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(pos.extend(1.0))
            .with_scale(Vec2::new(healthbar_width as f32, 21.0).extend(1.0)),
        HealthBarType::Green,
        HealthBar {
            enemy_bar: false,
            position: pos,
            health: 0,
            max_health: 0,
            custom_size: Some(IVec2::new(healthbar_width, 21)),
            center: true,
        },
        OpponentHealthBar {},
        Visibility::Hidden,
        MenuItem,
    ));

    let text_font = TextFont {
        font: asset_manager.fonts["fonts/Hachiro.ttf"].clone(),
        font_size: 32.0,
        font_smoothing: bevy::text::FontSmoothing::None,
        ..Default::default()
    };

    let e = commands
        .spawn((
            Text2d::new("35"),
            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
            text_font,
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
            Transform::from_translation((Vec2::ZERO).extend(3.0)),
            DamageText,
            Name::new("DamageText"),
        ))
        .id();

    opponent_health_bar_manager.damage_text = Some(e);
}
