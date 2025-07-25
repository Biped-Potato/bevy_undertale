use bevy::{prelude::*, text::TextLayoutInfo};

use crate::game::{
    data::data::Data,
    loading::loading::AssetManager,
    scene::internal::{
        bullet_board::BulletBoard,
        decisions::Decisions,
        helpers::menu_item::MenuItem,
        progress::Progress,
        stats::{HealthBar, HealthBarType},
    },
    state::state::AppState,
};

pub struct EnemyHealthPlugin;
impl Plugin for EnemyHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPreUpdate, make_enemy_bar_invis)
            .add_systems(OnEnter(AppState::Level), spawn_bar);
    }
}

#[derive(Component)]
pub struct EnemyHealthBar {
    pub text_entity: Option<Entity>,
}

pub fn manage_enemy_healthbar(
    mut decisions: ResMut<Decisions>,
    mut enemy_healthbar: Query<(
        &mut HealthBar,
        &mut HealthBarType,
        &mut EnemyHealthBar,
        &mut Visibility,
    )>,
    mut text_query: Query<(&mut TextLayoutInfo)>,
    data: Res<Data>,
    b_board: Res<BulletBoard>,
    progress: Res<Progress>,
) {
    for (mut h, mut h_t, mut e, mut v) in enemy_healthbar.iter_mut() {
        if let Ok(mut t) = text_query.get_mut(decisions.menu_entities.left_column[0]) {
            h.health = progress.health;
            h.max_health = data.game.opponent_data.health;

            let pos = Vec2::new(
                b_board.position.x - b_board.width / 2.0 + 65.0 + t.size.x + 62.0,
                -data.game.player.sprite_size_y / 2.0 + b_board.position.y + b_board.height / 2.0
                    - 23.0,
            );
            h.position = pos;
            *v = Visibility::Visible;
        }
    }
}

pub fn make_enemy_bar_invis(
    mut enemy_healthbar: Query<(&mut HealthBar, &mut EnemyHealthBar, &mut Visibility)>,
) {
    for (mut h, mut e, mut v) in enemy_healthbar.iter_mut() {
        *v = Visibility::Hidden;
    }
}

fn spawn_bar(
    mut commands: Commands,
    asset_manager: Res<AssetManager>,
    data: Res<Data>,
    mut b_board: Res<BulletBoard>,
) {
    let mut healthbar_width = 101.0;
    let pos = Vec2::new(
        data.game.player.sprite_size_x / 2.0 + b_board.position.x - b_board.width / 2.0 + 27.0,
        -data.game.player.sprite_size_y / 2.0 + b_board.position.y + b_board.height / 2.0 - 23.0,
    );
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(pos.extend(0.0))
            .with_scale(Vec2::new(healthbar_width, 21.0).extend(1.0)),
        HealthBarType::Red,
        HealthBar {
            custom_size: None,
            enemy_bar: false,
            position: pos,
            health: 0,
            max_health: 0,
            center: false,
        },
        EnemyHealthBar { text_entity: None },
        MenuItem,
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(pos.extend(1.0))
            .with_scale(Vec2::new(healthbar_width, 21.0).extend(1.0)),
        HealthBarType::Green,
        HealthBar {
            custom_size: None,
            enemy_bar: false,
            position: pos,
            health: 0,
            max_health: 0,
            center: false,
        },
        EnemyHealthBar { text_entity: None },
        MenuItem,
    ));
}
