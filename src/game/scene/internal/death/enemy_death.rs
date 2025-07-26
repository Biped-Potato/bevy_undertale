use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::{data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, player::player::Player, scene::internal::{helpers::despawn::{DespawnInTime, OpacityFromTimer}, menu::MenuState, menu_transition::MenuTransition, opponent::Opponent}, sound::sound::SoundPlayer};

pub struct EnemyDeathPlugin;
impl Plugin for EnemyDeathPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DeathActivator>()
            .add_systems(OnEnter(MenuState::EnemyDeath), (kill_enemy_visual,hide_player))
            .add_systems(OnExit(MenuState::EnemyDeath),show_player)
            .add_systems(FixedUpdate,(activate_dust,update_death_timer).run_if(in_state(MenuState::EnemyDeath)));
    }
}

#[derive(Resource,Default)]
pub struct DeathActivator {
    pub rows : Vec<Vec<Entity>>,
    pub timer : f32,
    pub i : i32,
    pub death_time : f32,
}

fn hide_player(mut player_query: Query<(&mut Visibility), With<Player>>) {
    if let Ok(mut v) = player_query.single_mut() {
        *v = Visibility::Hidden;
    }
}

fn show_player(mut player_query: Query<(&mut Visibility), With<Player>>) {
    if let Ok(mut v) = player_query.single_mut() {
        *v = Visibility::Visible;
    }
}
fn update_death_timer(
    mut d_a : ResMut<DeathActivator>,
    mut menu_transition : ResMut<MenuTransition>,
    mut time : Res<Time<Fixed>>,
) {
    d_a.death_time -= time.delta_secs();
    if d_a.death_time <= 0. {
        menu_transition.new_state(MenuState::Restart);
    }
}
fn activate_dust(
    mut commands : Commands,
    mut d_a : ResMut<DeathActivator>,
    mut query : Query<(&mut PhysicsComponent)>,
    mut time : Res<Time<Fixed>>,
    data : Res<Data>,
) {
    d_a.timer -= time.delta_secs();
    if d_a.timer <= 0. && d_a.i < d_a.rows.len() as i32 {
        let mut rand = thread_rng();
        d_a.timer = data.game.opponent_data.dust_time - d_a.timer.abs();
        let l = d_a.rows[d_a.i as usize].len();
        let row= &d_a.rows[d_a.i as usize];
        for i in 0..l {
            if let Ok(mut p) = query.get_mut(row[i]) {
                let v_x = rand.gen_range(-1.0..1.0);
                let v_y = rand.gen_range(0.0..1.0);

                let speed = 1.0;
                let velocity = Vec2::new(v_x,v_y).normalize_or(Vec2::new(1.0,0.0));
                p.velocity = velocity * speed;
                commands.entity(row[i]).insert(DespawnInTime::new(data.game.opponent_data.dust_life,None));
            }
        }
        d_a.i+=1;
    }
}
fn kill_enemy_visual(
    mut d_a : ResMut<DeathActivator>,
    mut commands : Commands,
    mut opponent_query : Query<(&mut Opponent,&mut Sprite,&mut Transform,&mut Visibility)>,
    mut images : ResMut<Assets<Image>>,
    mut sounds : ResMut<SoundPlayer>,
    asset_manager : Res<AssetManager>,
    data : Res<Data>,
) {
    if let Ok((mut o, mut s,mut t,mut v)) = opponent_query.single_mut() {
        sounds.play_sound_once_local(asset_manager.sounds["dust"].clone());
        *v = Visibility::Hidden;
        d_a.rows.clear();
        d_a.death_time = data.game.opponent_data.death_time;
        d_a.i = 0;
        if let Some(mut image) = images.get_mut(&s.image) {
            let width = image.width();
            let height = image.height();

            for y in (0..height).rev() {
                let mut row = Vec::new();
                for x in 0..width {
                    let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, height - y - 1, 0)).unwrap();
                    if pixel_bytes[3] > 0 {
                        let pos = Vec2::new(t.translation.x,t.translation.y) - Vec2::new(width as f32 * 2.0  / 2.0 , height as f32 * 2.0 / 2.0) + Vec2::splat(1.0) + Vec2::new(x as f32, y as f32) * 2.0; 
                        
                        let e = commands.spawn((
                            Sprite::from_color(Color::WHITE, Vec2::ONE * 2.0),
                            Transform::from_translation(pos.extend(2.0)),
                            PhysicsComponent {
                                position : pos,
                                ..Default::default()
                            },
                            OpacityFromTimer,
                        )).id();
                        row.push(e);
                    }
                }
                d_a.rows.push(row);
            }
        }
    }
}

