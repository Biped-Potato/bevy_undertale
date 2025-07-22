use bevy::prelude::*;

use crate::game::{loading::loading::AssetManager, physics::physics_object::PhysicsComponent, scene::{bullet_board::BulletBoard, dodging::DodgingPhaseManager, menu_transition::MenuTransition}};

pub struct AttacksPlugin;
impl Plugin for AttacksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Attack1>();  
    }
}


#[derive(Resource,Default)]
pub struct Attack1 {
    pub attack_timer : f32,
    pub attack_time : f32,
    pub attack_dir : i32,
}

pub fn enter_attack_1(
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
    mut dodge_manager : ResMut<DodgingPhaseManager>,
    asset_manager: Res<AssetManager>,
    mut attack : ResMut<Attack1>,
) {
    bullet_board.transition_board(asset_manager.board_layouts["battle_1"].clone());
    dodge_manager.time = 10.0;
    attack.attack_time = 2.0;
    attack.attack_timer = 2.0;
    attack.attack_dir = 0;
}

pub fn attack_1(
    mut commands : Commands,
    mut time : Res<Time<Fixed>>,
    mut attack : ResMut<Attack1>,
    bullet_board : Res<BulletBoard>,
    asset_manager: Res<AssetManager>
) {
    attack.attack_timer -= time.delta_secs();
    if attack.attack_timer <= 0. {
        attack.attack_timer = attack.attack_time;
        let mut dir = Vec2::ZERO;
        let mut spawn_dir = Vec2::ZERO;

        match attack.attack_dir {
            //left
            0 => {
                spawn_dir = Vec2::new(-1.,0.);
            }
            //top
            1 => {
                spawn_dir = Vec2::new(0.,1.);
            }
            //right
            2 => {
                spawn_dir = Vec2::new(1.,0.);
            }
            //bottom
            3 => {
                spawn_dir = Vec2::new(0.,-1.);
            }
            _ => {

            }
        }

        dir = -spawn_dir;
        let spacing = 20.0;
        let bullet_count = (bullet_board.width/spacing).ceil() as i32;
        let offset_dir = Vec2::new(spawn_dir.y,spawn_dir.x);
        let mut start = spawn_dir * bullet_board.width + (offset_dir * spacing * bullet_count as f32 / 2.0);
        let mut speed = 5.0;
        
        for i in 0.. bullet_count {
            if i != 4 {
                let pos = start -offset_dir * spacing * i as f32; 
                commands.spawn((
                    Sprite {
                        image : asset_manager.images["sprites/potato.png"].clone(),
                        ..Default::default()
                    },
                    PhysicsComponent::new_full(bullet_board.position + pos,dir * speed,Vec2::splat(6.0),Vec2::splat(8.0)),
                ));
                
            }
        }

        attack.attack_dir = (attack.attack_dir + 1) % 4;
    }
}