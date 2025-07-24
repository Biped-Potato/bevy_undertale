use bevy::{ecs::system::SystemId, prelude::*};

use crate::game::{
    data::data::Data,
    loading::loading::AssetManager,
    physics::physics_object::PhysicsComponent,
    player::player::{Player, player_movement},
    scene::{
        battle::BattleEvents,
        internal::{bullet_board::BulletBoard, menu::MenuState, menu_transition::MenuTransition},
    },
};

pub struct DodgingPlugin;
impl Plugin for DodgingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DodgingPhaseManager>()
            .add_systems(
                OnEnter(MenuState::Dodging),
                init_attack.before(update_dodging_phase),
            )
            .add_systems(
                FixedUpdate,
                constrain_player
                    .after(player_movement)
                    .run_if(in_state(MenuState::Dodging)),
            )
            .add_systems(
                FixedUpdate,
                update_dodging_phase.run_if(in_state(MenuState::Dodging)),
            );
    }
}
#[derive(Resource, Default)]
pub struct DodgingPhaseManager {
    pub time: f32,
    pub attack: Option<SystemId>,
    pub init_attack: Option<SystemId>,
}
impl DodgingPhaseManager {
    pub fn queue_attack() {}
}

fn init_attack(mut commands: Commands, mut dodging_manager: ResMut<DodgingPhaseManager>) {
    if dodging_manager.init_attack.is_some() {
        let init = dodging_manager.init_attack.unwrap();
        commands.run_system(init);
    }
}
fn update_dodging_phase(
    mut commands: Commands,
    mut dodging_manager: ResMut<DodgingPhaseManager>,
    mut time: Res<Time<Fixed>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
    mut battle_events: ResMut<BattleEvents>,
    asset_manager: Res<AssetManager>,
) {
    dodging_manager.time -= time.delta_secs();
    if dodging_manager.time <= 0. {
        menu_transition.new_state(MenuState::Selection);
        bullet_board.transition_board(asset_manager.board_layouts["selection"].clone());
        commands.run_system(battle_events.despawn_projectiles);
    } else {
        if dodging_manager.attack.is_some() {
            commands.run_system(dodging_manager.attack.unwrap());
        }
    }
}

fn constrain_player(
    mut bullet_board: ResMut<BulletBoard>,
    mut player_query: Query<(&mut Player, &mut PhysicsComponent)>,
    data: Res<Data>,
) {
    for (mut player, mut physics) in player_query.iter_mut() {
        if physics.position.x + data.game.player.sprite_size_x / 2.0
            > bullet_board.position.x + bullet_board.width / 2.0
        {
            physics.position.x = bullet_board.position.x + bullet_board.width / 2.0
                - data.game.player.sprite_size_x / 2.0;
        }
        if physics.position.x - data.game.player.sprite_size_x / 2.0
            < bullet_board.position.x - bullet_board.width / 2.0
        {
            physics.position.x = bullet_board.position.x - bullet_board.width / 2.0
                + data.game.player.sprite_size_x / 2.0;
        }
        if physics.position.y - data.game.player.sprite_size_y / 2.0
            < bullet_board.position.y - bullet_board.height / 2.0
        {
            physics.position.y = bullet_board.position.y - bullet_board.height / 2.0
                + data.game.player.sprite_size_y / 2.0;
        }
        if physics.position.y + data.game.player.sprite_size_y / 2.0
            > bullet_board.position.y + bullet_board.height / 2.0
        {
            physics.position.y = bullet_board.position.y + bullet_board.height / 2.0
                - data.game.player.sprite_size_y / 2.0;
        }
    }
}
