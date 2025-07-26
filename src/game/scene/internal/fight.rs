use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::game::{
    animation::animation::Animator, data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, player::player::Player, scene::{
        battle::BattleEvents,
        internal::{
            bullet_board::{self, BulletBoard},
            decisions::Decisions,
            helpers::menu_item::MenuItem,
            menu::MenuState,
            menu_transition::MenuTransition,
            opponent::{Opponent, OpponentHealthBarManager},
            progress::Progress,
        },
    }, sound::sound::SoundPlayer, state::state::AppState
};

pub struct FightPlugin;
impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FightManager>()
            .add_systems(OnEnter(AppState::Level), (spawn_fight_bar, spawn_slash))
            .add_systems(OnEnter(MenuState::Fight), init_fight)
            .add_systems(
                FixedUpdate,
                (update_fight_logic, update_slash_position).run_if(in_state(AppState::Level)),
            )
            .add_systems(
                Update,
                update_fight_controls.run_if(in_state(MenuState::Fight)),
            )
            .add_systems(
                FixedUpdate,
                update_fight_bar.run_if(in_state(MenuState::Fight)),
            )
            .add_systems(
                Update,
                update_player_visibility.run_if(in_state(MenuState::Fight)),
            );
    }
}

#[derive(Resource, Default)]
pub struct FightManager {
    pub attack_animation: f32,
    pub fade_timer: f32,
    pub strike: bool,
    pub position: f32,
    pub exit_fight_menu: bool,
    pub trigger_damage: bool,
    pub miss: bool,

    pub death_animation: f32,
}
impl FightManager {
    pub fn trigger_damage(&mut self) {
        if !self.strike {
            self.trigger_damage = true;
        }
        self.strike = true;
    }
    pub fn calculate_damage(&mut self, mut atk: f32, def: f32) -> i32 {
        atk = atk + 10.;
        let distance_from_center = self.position.abs();
        let target_width = 565.0 / 2.0;
        let mut rand = thread_rng();
        if distance_from_center <= 12. {
            return ((atk - def + rand.gen_range((1.)..(2.))) * 2.2).round() as i32;
        } else {
            return ((atk - def + rand.gen_range((1.)..(2.)))
                * (1. - distance_from_center / target_width)
                * 2.)
                .round() as i32;
        }
    }
}

#[derive(Component)]
pub struct FightBar;

#[derive(Component)]
pub struct TimingBar;

fn update_player_visibility(mut player_query: Query<(&mut Visibility), With<Player>>) {
    if let Ok(mut v) = player_query.single_mut() {
        *v = Visibility::Hidden;
    }
}
fn update_fight_logic(
    mut commands: Commands,
    menu_state: Res<State<MenuState>>,
    mut fight: ResMut<FightManager>,
    mut fightbar_query: Query<(&mut Sprite, &mut Transform), With<FightBar>>,
    mut timing_query: Query<(&mut Visibility), With<TimingBar>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut battle: ResMut<BattleEvents>,
    mut progress: ResMut<Progress>,
    mut opponent_bar_manager: ResMut<OpponentHealthBarManager>,
    data: Res<Data>,
    time: Res<Time<Fixed>>,
) {
    if fight.strike {
        if fight.trigger_damage {
            if !fight.miss {
                let damage = fight.calculate_damage(
                    data.game.player.at as f32,
                    data.game.opponent_data.df as f32,
                );
                opponent_bar_manager.damage_display = damage;
                opponent_bar_manager.old_health = progress.health;
                progress.health -= damage;
                if progress.health < 0 {
                    progress.health = 0;
                }
                opponent_bar_manager.new_health = progress.health;
            }

            fight.trigger_damage = false;
        }
        fight.attack_animation -= time.delta_secs();
        if fight.attack_animation <= 0. {
            fight.attack_animation = 0.;
            fight.fade_timer -= time.delta_secs();
            if fight.fade_timer <= 0. {
                fight.strike = false;
            }
            if progress.health <= 0 {
                menu_transition.new_state(MenuState::EnemyDeath);
            }
            else {
                if !fight.exit_fight_menu {
                    commands.run_system(battle.advance_attacks);
                }

                fight.exit_fight_menu = true;
            }
        }
    }
    if let Ok((mut s, mut t)) = fightbar_query.single_mut() {
        let ratio = fight.fade_timer / data.game.fight_bar.fade_time;
        s.color.set_alpha(ratio);
        t.scale.x = ratio;
    }
    if let Ok((mut v)) = timing_query.single_mut() {
        if fight.attack_animation > 0. {
            *v = Visibility::Visible;
        } else {
            *v = Visibility::Hidden;
        }
    }
}
fn spawn_fight_bar(
    mut commands: Commands,
    asset_manager: Res<AssetManager>,
    bullet_board: Res<BulletBoard>,
) {
    commands.spawn((
        Sprite {
            image: asset_manager.images["sprites/fightbar.png"].clone(),
            ..Default::default()
        },
        Transform::from_translation((bullet_board.position.round()).extend(1.0)),
        FightBar,
        Name::new("FightBar"),
        MenuItem,
    ));

    commands.spawn((
        Sprite {
            image: asset_manager.images["sprites/timing.png"].clone(),
            texture_atlas: Some(TextureAtlas {
                layout: asset_manager.atlases["timing"].clone(),
                index: 0,
                ..default()
            }),
            ..Default::default()
        },
        Animator {
            current_animation: "idle".to_string(),
            animation_bank: asset_manager.animations["timing"].clone(),
            ..Default::default()
        },
        Transform::from_translation(bullet_board.position.extend(1.0)),
        TimingBar,
        MenuItem,
        Visibility::Hidden,
    ));
}
#[derive(Component)]
pub struct Slash;
fn spawn_slash(mut commands: Commands, asset_manager: Res<AssetManager>) {
    commands.spawn((
        Sprite {
            image: asset_manager.images["sprites/slash.png"].clone(),
            texture_atlas: Some(TextureAtlas {
                layout: asset_manager.atlases["slash"].clone(),
                index: 0,
                ..default()
            }),
            ..Default::default()
        },
        Transform::from_translation(Vec2::ZERO.extend(5.0)),
        Animator {
            current_animation: "idle".to_string(),
            animation_bank: asset_manager.animations["slash"].clone(),
            ..Default::default()
        },
        Slash {},
        MenuItem,
    ));
}

fn update_slash_position(
    mut fight: ResMut<FightManager>,
    mut slash_query: Query<(&mut Transform, &mut Slash, &mut Animator)>,
    mut opponent_query: Query<(&mut PhysicsComponent, &mut Opponent)>,
    data: Res<Data>,
) {
    if let Ok((mut transform, mut slash, mut animator)) = slash_query.single_mut() {
        if let Ok((mut physics, mut opponent)) = opponent_query.single_mut() {
            transform.translation.x = (physics.position.x).round();
            transform.translation.y =
                (physics.position.y - data.game.opponent_data.height * 2.0 / 2.0 + 94.0 / 2.0).round();
        }
        animator.current_animation = "idle".to_string();
        if fight.strike && !fight.miss {
            if fight.attack_animation >= 1.0 {
                animator.current_animation = "slash".to_string();
            }
        }
    }
}
fn init_fight(
    mut commands: Commands,
    mut fight: ResMut<FightManager>,
    mut timing_query: Query<(&mut TimingBar, &mut Transform)>,
    mut decisions: ResMut<Decisions>,
    bullet_board: Res<BulletBoard>,
    data: Res<Data>,
) {
    if let Ok((mut bar, mut t)) = timing_query.single_mut() {
        fight.position = -bullet_board.border - bullet_board.width / 2.0;
        fight.fade_timer = data.game.fight_bar.fade_time;
        fight.attack_animation = data.game.fight_bar.attack_animation;
        fight.exit_fight_menu = false;
        fight.strike = false;
        commands.run_system(decisions.remove_decisions.unwrap());
    }
}
fn update_fight_bar(
    mut timing_query: Query<(&mut TimingBar, &mut Animator, &mut Transform)>,
    mut fight: ResMut<FightManager>,
    bullet_board: Res<BulletBoard>,
    data: Res<Data>,
) {
    if let Ok((mut bar, mut a, mut t)) = timing_query.single_mut() {
        if !fight.strike {
            fight.position += data.game.player.attack_speed;
            if fight.position >= bullet_board.width / 2.0 + bullet_board.border {
                fight.trigger_damage();
                //fight.attack_animation = 0.;
                fight.miss = true;
                log::info!("miss");
            }
            a.current_animation = "idle".to_string();
        } else {
            a.current_animation = "flash".to_string();
        }
        t.translation.x = fight.position.floor();
    
    }
}

fn update_fight_controls(asset_manager: Res<AssetManager>,keys: Res<ButtonInput<KeyCode>>, mut fight: ResMut<FightManager>,mut sounds : ResMut<SoundPlayer>,) {
    if keys.just_pressed(KeyCode::KeyZ) {
        fight.trigger_damage();
        sounds.play_sound_once_local(asset_manager.sounds["attack"].clone());
        fight.miss = false;
    }
}
