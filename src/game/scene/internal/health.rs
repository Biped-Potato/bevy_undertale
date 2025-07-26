use bevy::prelude::*;

use crate::game::{
    data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, player::player::{Player, PlayerStats}, scene::internal::{menu::MenuState, menu_transition::MenuTransition}, sound::sound::SoundPlayer
};

use crate::game::physics::rectangle::Rectangle;

pub struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_damage)
            .add_systems(FixedUpdate, (update_flash_animation));
    }
}

#[derive(Component)]
pub struct Damage {
    pub damage: i32,
}

fn update_damage(
    mut player_query: Query<(&mut PhysicsComponent, &mut Player)>,
    mut damage_query: Query<(&mut PhysicsComponent, &mut Damage), Without<Player>>,
    mut player_stats: ResMut<PlayerStats>,
    mut menu_transition: ResMut<MenuTransition>,
    mut sounds : ResMut<SoundPlayer>,
    asset_manager : Res<AssetManager>,
    data: Res<Data>,
) {
    if let Ok((mut physics, mut player)) = player_query.single_mut() {
        if player_stats.invincibility <= 0. {
            let rect_1 = Rectangle::new_v(physics.position, physics.half_hitbox);
            for (mut physics_2, mut damage) in damage_query.iter_mut() {
                let rect_2 = Rectangle::new_v(physics_2.position, physics_2.half_hitbox);
                if rect_1.intersects(rect_2) {
                    player_stats.invincibility = data.game.player.iframes;
                    player_stats.health -= damage.damage;

                    sounds.play_sound_once_local(asset_manager.sounds["hurt"].clone());

                    if player_stats.health <= 0 {
                        player_stats.health = 0;
                        menu_transition.new_state(MenuState::Restart);
                    }
                    break;
                }
            }
        }
    }
}

fn update_flash_animation(
    time: Res<Time<Fixed>>,
    mut health_query: Query<(&mut Player, &mut Sprite)>,
    mut player_stats: ResMut<PlayerStats>,
) {
    for (mut p, mut sprite) in health_query.iter_mut() {
        player_stats.invincibility -= time.delta_secs();

        if player_stats.invincibility >= 0. {
            player_stats.flash_animation -= time.delta_secs();
            if player_stats.flash_animation <= 0. {
                player_stats.flash_animation = player_stats.interval;
                if sprite.color.to_srgba().alpha == 1.0 {
                    sprite.color.set_alpha(0.1);
                } else {
                    sprite.color.set_alpha(1.0);
                }
            }
        } else {
            sprite.color.set_alpha(1.0);
        }
    }
}
