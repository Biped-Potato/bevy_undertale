use bevy::prelude::*;

use crate::game::{
    data::data::Data,
    physics::physics_object::PhysicsComponent,
    player::player::{Player, player_movement},
    scene::{bullet_board::BulletBoard, menu::MenuState},
};

pub struct DodgingPlugin;
impl Plugin for DodgingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            constrain_player
                .after(player_movement)
                .run_if(in_state(MenuState::Dodging)),
        );
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
