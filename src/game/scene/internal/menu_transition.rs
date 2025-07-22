use bevy::prelude::*;

use crate::game::{
    scene::internal::{bullet_board::BulletBoard, menu::MenuState},
    state::state::AppState,
};

pub struct MenuTransitionPlugin;
impl Plugin for MenuTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuTransition>().add_systems(
            PostUpdate,
            update_menu_state.run_if(in_state(AppState::Level)),
        );
    }
}

#[derive(Resource, Default)]
pub struct MenuTransition {
    pub new_menu: Option<MenuState>,
}

impl MenuTransition {
    pub fn new_state(&mut self, state: MenuState) {
        self.new_menu = Some(state);
    }
}
fn update_menu_state(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
) {
    if menu_transition.new_menu.is_some() {
        if bullet_board.stable() {
            menu_state.set(menu_transition.new_menu.unwrap());
            menu_transition.new_menu = None;
        }
    }
}
