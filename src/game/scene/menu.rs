use bevy::prelude::*;

use crate::game::scene::{bullet_board::BulletBoardPlugin, selection::MenuSelectPlugin};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins(MenuSelectPlugin)
            .add_plugins(BulletBoardPlugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    Selection,
    Fight,
    Act,
    Item,
    Mercy,
    Dodging,
}
