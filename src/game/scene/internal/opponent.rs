use bevy::prelude::*;

use crate::game::{
    data::data::Data,
    physics::physics_object::PhysicsComponent,
    scene::{battle::spawn_opponent, internal::bullet_board::BulletBoard},
    state::state::AppState,
};

pub struct OpponentPlugin;
impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Level), spawn_opponent)
            .add_systems(
                Update,
                update_opponent_position.run_if(in_state(AppState::Level)),
            );
    }
}

#[derive(Component)]
pub struct Opponent {}

fn update_opponent_position(
    bullet_board: Res<BulletBoard>,
    mut opponent_query: Query<(&mut Opponent, &mut PhysicsComponent)>,
    data: Res<Data>,
) {
    for (mut opponent, mut physics) in opponent_query.iter_mut() {
        physics.position.x = 0.;
        physics.position.y = bullet_board.position.y
            + bullet_board.height / 2.0
            + bullet_board.border
            + 10.0
            + data.game.opponent_data.height / 2.0;
    }
}
