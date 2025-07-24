use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

pub struct AtlasAnimationPlugin;
impl Plugin for AtlasAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, animate_sprite);
    }
}
#[derive(Deserialize, Clone, Default)]
pub struct Animation {
    pub name: String,
    pub cooldown: f32,
    pub start: i32,
    pub end: i32,
    pub looping: bool,
}
#[derive(Clone, Component)]
pub struct Animator {
    pub animation_bank: HashMap<String, Animation>,
    pub current_animation: String,
    pub last_animation: String,
    pub timer: f32,
    pub cooldown: f32,
    pub finished: bool,
    pub index: usize,
}
impl Default for Animator {
    fn default() -> Self {
        Animator {
            index: 0,
            animation_bank: create_anim_hashmap(),
            timer: 0.,
            cooldown: 0.1,
            last_animation: " ".to_string(),
            current_animation: "Idle".to_string(),
            finished: false,
        }
    }
}
pub fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Animator, &mut Sprite)>) {
    for (mut animator, mut sprite_component) in query.iter_mut() {
        let anim = animator.animation_bank[animator.current_animation.as_str()].clone();
        let sprite = sprite_component.texture_atlas.as_mut().unwrap();
        if animator.last_animation != animator.current_animation {
            sprite.index = anim.start as usize - 1;
            animator.finished = false;
        }
        animator.timer -= time.delta().as_secs_f32();
        if animator.timer <= 0. {
            animator.timer = anim.cooldown;
            if anim.looping {
                if sprite.index < anim.start as usize - 1 {
                    sprite.index = anim.start as usize - 1;
                }
                sprite.index = ((sprite.index + 1 - (anim.start as usize - 1))
                    % (anim.end as usize - anim.start as usize + 1))
                    + anim.start as usize
                    - 1;
            } else if !anim.looping {
                sprite.index += 1;
                if sprite.index > anim.end as usize - 1 {
                    sprite.index = anim.end as usize - 1;
                    animator.finished = true;
                }
            }
        }
        animator.index = sprite.index;
        animator.last_animation = animator.current_animation.clone();
    }
}
pub fn create_anim_hashmap() -> HashMap<String, Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "".to_string(),
        Animation {
            name: "".to_string(),
            start: 1,
            end: 1,
            looping: true,
            cooldown: 0.1,
        },
    );

    hash_map
}
