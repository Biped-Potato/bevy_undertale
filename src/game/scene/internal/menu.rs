use bevy::prelude::*;

use crate::game::scene::{
    attacks::AttacksPlugin,
    internal::{
        bullet_board::BulletBoardPlugin,
        death::{enemy_death::EnemyDeathPlugin, restart_screen::RestartPlugin},
        decisions::DecisionPlugin,
        dodging::DodgingPlugin,
        enemy_health::EnemyHealthPlugin,
        fight::FightPlugin,
        health::DamagePlugin,
        helpers::despawn::DespawnPlugin,
        menu_transition::MenuTransitionPlugin,
        opponent::OpponentPlugin,
        progress::ProgressPlugin,
        selection::{MenuOption, MenuSelectPlugin},
        stats::StatsPlugin,
        text::TextBoxPlugin,
    },
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins((
                MenuSelectPlugin,
                BulletBoardPlugin,
                StatsPlugin,
                DecisionPlugin,
                TextBoxPlugin,
                DodgingPlugin,
                MenuTransitionPlugin,
                FightPlugin,
                OpponentPlugin,
                ProgressPlugin,
                AttacksPlugin,
                DamagePlugin,
                DespawnPlugin,
                EnemyHealthPlugin,
                RestartPlugin,
            ))
            .add_plugins((EnemyDeathPlugin));
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

    EnemyDeath,
    Restart,

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
