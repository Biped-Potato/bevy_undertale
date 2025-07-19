use bevy::prelude::*;

use crate::game::{
    camera::render_layers::RenderLayerStorage,
    data::data::Data,
    loading::loading::AssetManager,
    physics::physics_object::PhysicsComponent,
    scene::{bullet_board::BulletBoard, menu::MenuState, stats::spawn_stats},
    state::state::AppState,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerStats {
            health: 0,
            max_health: 0,
        })
        .add_systems(OnEnter(MenuState::Dodging), move_soul)
        .add_systems(OnEnter(AppState::Level), spawn_player.before(spawn_stats))
        .add_systems(
            FixedUpdate,
            player_movement.run_if(in_state(MenuState::Dodging)),
        );
    }
}

#[derive(Resource)]
pub struct PlayerStats {
    pub health: i32,
    pub max_health: i32,
}

#[derive(Component)]
pub struct Player {}

fn move_soul(
    mut bullet_board: Res<BulletBoard>,
    mut player_query: Query<(&mut Player, &mut PhysicsComponent)>,
) {
    if let Ok((mut p, mut physics)) = player_query.single_mut() {
        physics.position = bullet_board.position;
    }
}
fn spawn_player(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    asset_manager: Res<AssetManager>,
    data: Res<Data>,
    render_layers: Res<RenderLayerStorage>,
    bullet_board: Res<BulletBoard>,
) {
    player_stats.health = data.player.health;
    player_stats.max_health = data.player.health;
    commands.spawn((
        Sprite {
            image: asset_manager.images[&data.player.sprite.clone()].clone(),
            ..Default::default()
        },
        PhysicsComponent::new(bullet_board.position),
        render_layers.pre.clone(),
        Player {},
    ));
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut PhysicsComponent)>,
    data: Res<Data>,
) {
    for (mut player, mut physics) in player_query.iter_mut() {
        let mut horizontal = 0.;
        let mut vertical = 0.;
        if input.pressed(KeyCode::ArrowUp) {
            vertical += 1.;
        }
        if input.pressed(KeyCode::ArrowDown) {
            vertical -= 1.;
        }
        if input.pressed(KeyCode::ArrowRight) {
            horizontal += 1.;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            horizontal -= 1.;
        }
        physics.position.x += horizontal * data.player.speed;
        physics.position.y += vertical * data.player.speed;
    }
}
