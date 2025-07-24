use bevy::prelude::*;

use crate::game::{
    data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, scene::{battle::spawn_opponent, internal::{bullet_board::BulletBoard, enemy_health::EnemyHealthBar, fight::FightManager, stats::{HealthBar, HealthBarType}}}, state::state::AppState
};

pub struct OpponentPlugin;
impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OpponentHealthBarManager>()
            .add_systems(OnEnter(AppState::Level), (spawn_opponent,spawn_bar))
            .add_systems(
                Update,
                update_opponent_position.run_if(in_state(AppState::Level)),
            )
            .add_systems(FixedUpdate, update_enemy_healthbar.run_if(in_state(AppState::Level)));
    }
}

#[derive(Resource,Default)]
pub struct OpponentHealthBarManager {
    pub old_health : i32,
    pub new_health : i32,
    pub damage_display : i32,
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

fn update_enemy_healthbar(
    mut fight_manager : ResMut<FightManager>,
    mut bar_manager : ResMut<OpponentHealthBarManager>,
    mut opponent_query: Query<(&mut Opponent, &mut PhysicsComponent)>,
    mut bar_query : Query<(&mut HealthBarType,&mut HealthBar,&mut OpponentHealthBar,&mut Visibility,&mut Transform)>,
    data : Res<Data>,
) {
    if let Ok((mut o,mut physics)) = opponent_query.single_mut() {
        for(mut h_t,mut h,mut b,mut v,mut t) in bar_query.iter_mut() {
            if fight_manager.strike {
                if fight_manager.attack_animation <= 1.0 {
                    h.max_health = data.game.opponent_data.health;
                    h.position = physics.position + Vec2::new(0.,data.game.opponent_data.height / 2.0) + Vec2::new(0.,h.custom_size.unwrap().y as f32 / 2.0);
                    let diff = bar_manager.old_health - bar_manager.new_health;
                    h.health = (bar_manager.new_health as f32 + diff as f32 * (fight_manager.attack_animation)).round() as i32;
                    *v = Visibility::Visible;
                }
            }
            else {
                 *v = Visibility::Hidden;
            }
        }
    }
}
#[derive(Component)]
pub struct OpponentHealthBar;

fn spawn_bar(
    mut commands: Commands,
    asset_manager: Res<AssetManager>,
    data : Res<Data>,
    mut b_board: Res<BulletBoard>,
){
    
    let mut healthbar_width = 101;
    let pos = Vec2::ZERO;
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(
            pos
            .extend(0.0),
        )
        .with_scale(Vec2::new(healthbar_width as f32, 21.0).extend(1.0)),
        HealthBarType::Red,
        HealthBar {
            enemy_bar : false,
            position : pos,
            health : 0,
            max_health : 0,
            custom_size : Some(IVec2::new(healthbar_width,21)),
            center : true,
        },
        OpponentHealthBar {

        },
        Visibility::Hidden,
    ));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::splat(1.0)),
        Transform::from_translation(
            pos
            .extend(1.0),
        )
        .with_scale(Vec2::new(healthbar_width as f32, 21.0).extend(1.0)),
        HealthBarType::Green,
        HealthBar {
            enemy_bar : false,
            position : pos,
            health : 0,
            max_health : 0,
            custom_size : Some(IVec2::new(healthbar_width,21)),
            center : true,
        },
        OpponentHealthBar {

        },
        Visibility::Hidden,
    ));
}