use bevy::{math::VectorSpace, prelude::*};

use crate::game::{
    data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, player::player::{Player, PlayerStats}, scene::internal::{
        bullet_board::{move_towards_vec, BulletBoard}, dodging::DodgingPhaseManager, helpers::{despawn::DespawnInMenu, menu_item::MenuItem}, menu::MenuState, menu_transition::MenuTransition, progress::Progress
    }
};

pub struct RestartPlugin;
impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MenuState::Restart),
            (init_restart_screen, hide_menu),
        )
        .add_systems(
            OnExit(MenuState::Restart),
            (despawn_restart_screen, show_menu),
        )
        .add_systems(
            Update,
            (update_restart).run_if(in_state(MenuState::Restart)),
        )
        .add_systems(
            FixedUpdate,
            (hover_soul).run_if(in_state(MenuState::Restart)),
        );
    }
}

fn update_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut progress: ResMut<Progress>,
    mut player_stats: ResMut<PlayerStats>,
    mut bullet_board : ResMut<BulletBoard>,
    asset_manager : Res<AssetManager>,
    mut dodging_manager: ResMut<DodgingPhaseManager>,
    mut player_query : Query<(&mut PhysicsComponent, &mut Player)>,
    data: Res<Data>,
) {
    if let Ok((mut physics,mut p)) = player_query.single_mut() {
        if Vec2::length(physics.position) <= 2.0 && keys.just_pressed(KeyCode::KeyZ) {
            bullet_board.absolute_board(asset_manager.board_layouts["selection"].clone());
            menu_transition.new_state(MenuState::Selection);
            progress.turns = 0;
            progress.health = data.game.opponent_data.health;
            player_stats.health = player_stats.max_health;
            dodging_manager.time = 0.;
        }
    }
}

#[derive(Component)]
pub struct RestartText;
fn init_restart_screen(mut commands: Commands, asset_manager: Res<AssetManager>) {
    let text_font = TextFont {
        font: asset_manager.fonts["fonts/DTM-Mono.ttf"].clone(),
        font_size: 26.0,
        font_smoothing: bevy::text::FontSmoothing::None,
        ..Default::default()
    };

    let e = commands
        .spawn((
            Text2d::new("Press Z to Restart"),
            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
            text_font,
            Transform::from_translation((Vec2::ZERO).extend(3.0) + Vec3::new(0.5,26.0,0.0)),
            Name::new("RestartText"),
            RestartText,
        ))
        .id();
}

fn hover_soul(
    mut player_query : Query<(&mut PhysicsComponent, &mut Player)>,
    time : Res<Time>,
) {
    for(mut physics, mut player) in player_query.iter_mut() {
        let initial = physics.position;
        physics.position += move_towards_vec(initial,Vec2::ZERO,5.0);
    }
}
fn hide_menu(
    mut commands: Commands,
    mut despawn_query: Query<(Entity), With<DespawnInMenu>>,
    mut menu_query: Query<(&mut Visibility), With<MenuItem>>,
) {
    for (mut v) in menu_query.iter_mut() {
        *v = Visibility::Hidden;
    }
    for (e) in despawn_query.iter_mut() {
        commands.entity(e).despawn();
    }
}

fn despawn_restart_screen(
    mut commands: Commands,
    despawn_query: Query<(Entity), With<RestartText>>,
) {
    for (e) in despawn_query.iter() {
        commands.entity(e).despawn();
    }
}

fn show_menu(mut menu_query: Query<(&mut Visibility), With<MenuItem>>) {
    for (mut v) in menu_query.iter_mut() {
        *v = Visibility::Visible;
    }
}
