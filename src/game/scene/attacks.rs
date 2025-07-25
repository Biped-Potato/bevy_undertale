use bevy::prelude::*;

use crate::game::{
    loading::loading::AssetManager,
    physics::physics_object::PhysicsComponent,
    scene::internal::{
        bullet_board::BulletBoard,
        dodging::DodgingPhaseManager,
        health::Damage,
        helpers::{despawn::DespawnInMenu, menu_item::MenuItem},
        menu_transition::MenuTransition,
    },
};

pub struct AttacksPlugin;
impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShovelAttack>()
            .init_resource::<Attack1>();
    }
}

#[derive(Resource, Default)]
pub struct Attack1 {
    pub attack_timer: f32,
    pub attack_time: f32,
    pub attack_dir: i32,
}

#[derive(Resource, Default)]
pub struct ShovelAttack {
    pub timer: f32,
}
pub fn enter_attack_1(
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
    mut dodge_manager: ResMut<DodgingPhaseManager>,
    asset_manager: Res<AssetManager>,
    mut attack: ResMut<Attack1>,
) {
    bullet_board.transition_board(asset_manager.board_layouts["battle_1"].clone());
    dodge_manager.time = 10.0;
    attack.attack_time = 1.0;
    attack.attack_timer = 1.0;
    attack.attack_dir = 0;
}

pub fn attack_1(
    mut commands: Commands,
    mut time: Res<Time<Fixed>>,
    mut attack: ResMut<Attack1>,
    bullet_board: Res<BulletBoard>,
    asset_manager: Res<AssetManager>,
) {
    attack.attack_timer -= time.delta_secs();
    if attack.attack_timer <= 0. {
        attack.attack_timer = attack.attack_time;
        let mut dir = Vec2::ZERO;
        let mut spawn_dir = Vec2::ZERO;
        let mut distance = bullet_board.width;
        let mut line_up_distance = bullet_board.height;
        match attack.attack_dir {
            //left
            0 => {
                spawn_dir = Vec2::new(-1., 0.);
            }
            //top
            1 => {
                spawn_dir = Vec2::new(0., 1.);
                distance = bullet_board.height;
                line_up_distance = bullet_board.width;
            }
            //right
            2 => {
                spawn_dir = Vec2::new(1., 0.);
            }
            //bottom
            3 => {
                spawn_dir = Vec2::new(0., -1.);
                distance = bullet_board.height;
                line_up_distance = bullet_board.width;
            }
            _ => {}
        }

        dir = -spawn_dir;
        let spacing = 16.0;
        let bullet_count = line_up_distance as i32 / spacing as i32 - 1;

        let offset_dir = Vec2::new(spawn_dir.y, spawn_dir.x);
        let mut start = spawn_dir * distance / 2.0;
        let mut half_size = Vec2::splat(3.0);
        let mut physics_half_size = Vec2::splat(3.0);
        start += offset_dir * (line_up_distance / 2.0 - spacing);
        let mut speed = 3.0;

        for i in 0..bullet_count {
            if i != 1 {
                let pos = start - offset_dir * spacing * i as f32;
                commands.spawn((
                    Sprite {
                        image: asset_manager.images["sprites/potato.png"].clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec2::ZERO.extend(-1.0)),
                    PhysicsComponent::new_full(
                        bullet_board.position + pos,
                        dir * speed,
                        half_size,
                        physics_half_size,
                    ),
                    DespawnInMenu,
                    Damage { damage: 5 },
                ));
            }
        }

        attack.attack_dir = (attack.attack_dir + 1) % 4;
    }
}

#[derive(Component)]
pub struct Shovel {
    pub initial_pos: f32,
    pub offset: Vec2,
    pub time: f32,
    pub timer: f32,
}
pub fn enter_shovel_attack(
    mut commands: Commands,
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
    mut dodge_manager: ResMut<DodgingPhaseManager>,
    asset_manager: Res<AssetManager>,
    mut shovel_atk: ResMut<ShovelAttack>,
) {
    bullet_board.transition_board(asset_manager.board_layouts["shovel_tunnel"].clone());
    dodge_manager.time = 10.0;
    shovel_atk.timer = 0.;
}

pub fn spawn_shovels(
    mut commands: Commands,
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
    mut dodge_manager: ResMut<DodgingPhaseManager>,
    asset_manager: Res<AssetManager>,
    mut shovel_atk: ResMut<ShovelAttack>,
) {
    let shovel_count = (bullet_board.target_width / 10 as f32) as i32;
    let spacing = 10.0;
    let mut half_size = Vec2::new(5.0, 17.0);
    let mut physics_half_size = Vec2::new(5.0, 20.0);
    let start = -bullet_board.target_width / 2.0 + half_size.x;
    let half_gap = 24.0;
    let top = bullet_board.target_position.y + physics_half_size.y + half_gap;
    let bottom = bullet_board.target_position.y - physics_half_size.y - half_gap;

    for i in 0..shovel_count {
        let pos = start + spacing * i as f32;

        commands.spawn((
            Sprite {
                image: asset_manager.images["sprites/shovel.png"].clone(),
                flip_y: true,
                ..Default::default()
            },
            Transform::from_translation(Vec2::ZERO.extend(-1.0)),
            PhysicsComponent::new_full(
                bullet_board.target_position + Vec2::new(pos, physics_half_size.y),
                Vec2::ZERO,
                half_size,
                physics_half_size,
            ),
            DespawnInMenu,
            Damage { damage: 5 },
            Shovel {
                initial_pos: top,
                offset: Vec2::new(0., physics_half_size.y * 2.0),
                time: 2.0,
                timer: 2.0,
            },
        ));

        commands.spawn((
            Sprite {
                image: asset_manager.images["sprites/shovel.png"].clone(),
                flip_y: false,
                ..Default::default()
            },
            Transform::from_translation(Vec2::ZERO.extend(-1.0)),
            PhysicsComponent::new_full(
                bullet_board.target_position + Vec2::new(pos, -physics_half_size.y),
                Vec2::ZERO,
                half_size,
                physics_half_size,
            ),
            DespawnInMenu,
            Damage { damage: 5 },
            Shovel {
                initial_pos: bottom,
                offset: Vec2::new(0., -physics_half_size.y * 2.0),
                time: 2.0,
                timer: 2.0,
            },
        ));
    }
}
pub fn shovel_attack(
    mut commands: Commands,
    mut time: Res<Time<Fixed>>,
    mut attack: ResMut<ShovelAttack>,
    mut shovel_query: Query<(&mut PhysicsComponent, &mut Shovel)>,
    bullet_board: Res<BulletBoard>,
    asset_manager: Res<AssetManager>,
) {
    attack.timer += time.delta_secs();
    for (mut p, mut s) in shovel_query.iter_mut() {
        s.timer -= time.delta_secs();
        if s.timer <= 0. {
            s.timer = 0.;
        }
        let t = p.position.x / bullet_board.width;
        p.position.y = s.initial_pos
            + f32::sin(t * 2. * 3.14159 + attack.timer * 5.0) * 24.0
            + s.offset.y * (s.timer / s.time);
    }
}
