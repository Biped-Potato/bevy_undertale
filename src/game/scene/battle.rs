use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::scene::{decisions::{remove_decisions, Decision, DecisionMenu, Decisions}, menu::MenuPlugin, progress::ProgressPlugin, selection::MenuOption};

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuPlugin,
            ProgressPlugin
        ));
    }
}

impl FromWorld for Decisions {
    fn from_world(world: &mut World) -> Self {
        let mut menu = HashMap::new();

        let mut fight_menu = DecisionMenu::default();
        let mut act_menu = DecisionMenu::default();
        let mut item_menu = DecisionMenu::default();
        let mut mercy_menu = DecisionMenu::default();

        fight_menu.left_column.push(Decision::new("Dummy".to_string(),world.register_system(start_fight)));

        let mut act_sub_menu = DecisionMenu::default();
        
        act_sub_menu.left_column.push(Decision::new("Check".to_string(),world.register_system(check)));
        act_sub_menu.right_column.push(Decision::new("Talk".to_string(),world.register_system(talk)));

        act_menu.left_column.push(Decision::new_with_menu("Dummy".to_string(),Some(act_sub_menu)));


        menu.insert(MenuOption::Fight,fight_menu);
        menu.insert(MenuOption::Act, act_menu);
        menu.insert(MenuOption::Item,item_menu);
        menu.insert(MenuOption::Mercy, mercy_menu);
        
        
        Self {
            remove_decisions : Some(world.register_system(remove_decisions)),
            menu : menu,
            decision_menu : None,
            menu_entities : default(),
            side : 0,
            selection : 0,
            switch_menu : false,
            submenu : false,
            increment : 0.,
            spacing : 0.,
        }
    }
}

fn start_fight() {

}

fn talk() {

}

fn check() {

}