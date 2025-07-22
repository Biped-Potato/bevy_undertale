use bevy::prelude::*;

use crate::game::scene::internal::{
    bullet_board::BulletBoardPlugin,
    decisions::DecisionPlugin,
    dodging::DodgingPlugin,
    fight::FightPlugin,
    menu_transition::MenuTransitionPlugin,
    selection::{MenuOption, MenuSelectPlugin},
    stats::StatsPlugin,
    text::TextBoxPlugin,
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins(MenuSelectPlugin)
            .add_plugins(BulletBoardPlugin)
            .add_plugins(StatsPlugin)
            .add_plugins(DecisionPlugin)
            .add_plugins(TextBoxPlugin)
            .add_plugins(DodgingPlugin)
            .add_plugins(MenuTransitionPlugin)
            .add_plugins(FightPlugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    Selection,

    Decision,
    Text,
    Fight,
    Dodging,

    ERROR,
}

impl MenuState {
    pub fn from_option(o: MenuOption) -> MenuState {
        match o {
            MenuOption::Act => {
                return MenuState::Decision;
            }
            MenuOption::Fight => {
                return MenuState::Fight;
            }
            MenuOption::Item => {
                return MenuState::Decision;
            }
            MenuOption::Mercy => {
                return MenuState::Decision;
            }
            _ => {
                return MenuState::ERROR;
            }
        }
    }
}
