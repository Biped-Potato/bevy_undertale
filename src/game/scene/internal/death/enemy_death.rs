use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::{physics::physics_object::PhysicsComponent, scene::internal::{menu::MenuState, opponent::Opponent}};

pub struct EnemyDeathPlugin;
impl Plugin for EnemyDeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::EnemyDeath), kill_enemy_visual);
    }
}

fn kill_enemy_visual(
    mut commands : Commands,
    mut opponent_query : Query<(&mut Opponent,&mut Sprite,&mut Transform,&mut Visibility)>,
    mut images : ResMut<Assets<Image>>,
) {
    if let Ok((mut o, mut s,mut t,mut v)) = opponent_query.single_mut() {
        *v = Visibility::Hidden;
        if let Some(mut image) = images.get_mut(&s.image) {
            let width = image.width();
            let height = image.height();

            let mut rand = thread_rng();
            
            for x in 0..width {
                for y in 0..height {
                    let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, height - y - 1, 0)).unwrap();
                    if pixel_bytes[3] > 0 {
                        let v_x = rand.gen_range(-1.0..1.0);
                        let v_y = rand.gen_range(0.0..1.0);

                        let speed = 3.0;
                        let velocity = Vec2::new(v_x,v_y).normalize_or(Vec2::new(1.0,0.0));
                        let pos = Vec2::new(t.translation.x,t.translation.y) - Vec2::new(width as f32 / 2.0 , height as f32 / 2.0) + Vec2::splat(0.5) + Vec2::new(x as f32, y as f32);
                        println!("{}", velocity);
                        commands.spawn((
                            Sprite::from_color(Color::WHITE, Vec2::ONE),
                            Transform::from_translation(pos.extend(2.0)),
                            PhysicsComponent {
                                position : pos,
                                velocity : velocity * 2.0,
                                ..Default::default()
                            },
                        ));
                    }
                }
            }
        }
    }
}

