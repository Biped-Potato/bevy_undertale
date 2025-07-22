use bevy::prelude::*;

use crate::game::{
    data::data::Data,
    loading::loading::AssetManager,
    player::player::Player,
    scene::{
        battle::BattleEvents, bullet_board::{self, BulletBoard}, decisions::Decisions, menu::MenuState, menu_transition::MenuTransition, progress::Progress
    },
    state::state::AppState,
};

pub struct FightPlugin;
impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FightManager>()
            .add_systems(OnEnter(AppState::Level), spawn_fight_bar)
            .add_systems(OnEnter(MenuState::Fight), init_fight)
            .add_systems(
                FixedUpdate,
                update_visibility.run_if(in_state(AppState::Level)),
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
fn update_visibility(
    mut commands : Commands,
    menu_state: Res<State<MenuState>>,
    mut fight: ResMut<FightManager>,
    mut fightbar_query: Query<(&mut Sprite, &mut Transform), With<FightBar>>,
    mut timing_query: Query<(&mut Visibility), With<TimingBar>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut battle : ResMut<BattleEvents>,
    mut progress : ResMut<Progress>,
    data: Res<Data>,
    time: Res<Time<Fixed>>,
) {
    if fight.strike {
        fight.attack_animation -= time.delta_secs();
        if fight.attack_animation <= 0. {
            fight.attack_animation = 0.;
            fight.fade_timer -= time.delta_secs();
            if fight.fade_timer <= 0. {
                fight.strike = false;
            }
            if !fight.exit_fight_menu {
                commands.run_system(battle.advance_attacks);
                //menu_transition.new_state(MenuState::Selection);
            }

            fight.exit_fight_menu = true;
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
    ));

    commands.spawn((
        Sprite {
            image: asset_manager.images["sprites/timing.png"].clone(),
            ..Default::default()
        },
        Transform::from_translation(bullet_board.position.extend(1.0)),
        TimingBar,
        Visibility::Hidden,
    ));
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
    mut timing_query: Query<(&mut TimingBar, &mut Transform)>,
    mut fight: ResMut<FightManager>,
    bullet_board: Res<BulletBoard>,
    data: Res<Data>,
) {
    if let Ok((mut bar, mut t)) = timing_query.single_mut() {
        if !fight.strike {
            fight.position += data.game.player.attack_speed;
        }
        t.translation.x = fight.position.floor();

        if fight.position >= bullet_board.width / 2.0 + bullet_board.border {
            fight.strike = true;
        }
    }
}

fn update_fight_controls(keys: Res<ButtonInput<KeyCode>>, mut fight: ResMut<FightManager>) {
    if keys.just_pressed(KeyCode::KeyZ) {
        fight.strike = true;
    }
}
