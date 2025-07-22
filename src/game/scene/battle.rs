use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};

use crate::game::{
    loading::loading::AssetManager, physics::physics_object::PhysicsComponent, scene::{
        attack::Attack, attacks::{attack_1, enter_attack_1, AttacksPlugin}, bullet_board::BulletBoard, decisions::{remove_decisions, Decision, DecisionMenu, Decisions}, dodging::DodgingPhaseManager, menu::{MenuPlugin, MenuState}, menu_transition::MenuTransition, opponent::{Opponent, OpponentPlugin}, progress::{Progress, ProgressPlugin}, selection::MenuOption, text::TextBox
    }
};

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleEvents>().add_plugins((
            MenuPlugin,
            OpponentPlugin,
            ProgressPlugin,
            AttacksPlugin,
        ));
    }
}

#[derive(Resource)]
pub struct BattleEvents {
    pub events: HashMap<String, SystemId>,
    pub advance_attacks: SystemId,
    pub attacks: Vec<Attack>,
}
impl FromWorld for BattleEvents {
    fn from_world(world: &mut World) -> Self {
        let mut events = HashMap::new();
        let mut attacks = vec![Attack {
            enter_attack: Some(world.register_system(enter_attack_1)),
            attack: Some(world.register_system(attack_1)),
            exit_attack: None,
        }];

        Self {
            advance_attacks: world.register_system(enter_planned_attack),
            events: events,
            attacks: attacks,
        }
    }
}
impl FromWorld for Decisions {
    fn from_world(world: &mut World) -> Self {
        let mut menu = HashMap::new();

        let mut fight_menu = DecisionMenu::default();
        let mut act_menu = DecisionMenu::default();
        let mut item_menu = DecisionMenu::default();
        let mut mercy_menu = DecisionMenu::default();

        fight_menu.left_column.push(Decision::new(
            "Biped Potato".to_string(),
            world.register_system(start_fight),
        ));

        let mut act_sub_menu = DecisionMenu::default();

        act_sub_menu.left_column.push(Decision::new(
            "Check".to_string(),
            world.register_system(check),
        ));
        act_sub_menu.right_column.push(Decision::new(
            "Talk".to_string(),
            world.register_system(talk),
        ));

        act_menu.left_column.push(Decision::new_with_menu(
            "Biped Potato".to_string(),
            Some(act_sub_menu),
        ));

        item_menu.left_column.push(Decision::new(
            "Monster Candy".to_string(),
            world.register_system(item),
        ));

        mercy_menu.left_column.push(Decision::new(
            "Spare".to_string(),
            world.register_system(item),
        ));

        mercy_menu.left_column.push(Decision::new(
            "Flee".to_string(),
            world.register_system(item),
        ));
        menu.insert(MenuOption::Fight, fight_menu);
        menu.insert(MenuOption::Act, act_menu);
        menu.insert(MenuOption::Item, item_menu);
        menu.insert(MenuOption::Mercy, mercy_menu);

        Self {
            remove_decisions: Some(world.register_system(remove_decisions)),
            menu: menu,
            decision_menu: None,
            menu_entities: default(),
            side: 0,
            selection: 0,
            switch_menu: false,
            submenu: false,
            increment: 0.,
            spacing: 0.,
        }
    }
}

fn start_fight(
    mut bullet_board: Res<BulletBoard>,
    mut text_box: ResMut<TextBox>,
    mut menu_transition: ResMut<MenuTransition>,
    asset_manager: Res<AssetManager>,
) {
    menu_transition.new_state(MenuState::Fight);
}

fn talk(
    mut commands: Commands,
    mut decisions: ResMut<Decisions>,
    mut text_box: ResMut<TextBox>,
    mut battle_events: ResMut<BattleEvents>,
    mut menu_transition: ResMut<MenuTransition>,
    progress: Res<Progress>,
    asset_manager: Res<AssetManager>,
) {
    commands.run_system(decisions.remove_decisions.unwrap());
    menu_transition.new_state(MenuState::Text);
    text_box.queue_event(
        asset_manager.dialogue_storage["talk"].clone(),
        battle_events.advance_attacks,
    );
}

pub fn spawn_opponent(
    asset_manager: Res<AssetManager>,
    mut commands : Commands,
) {
    commands.spawn((
        Sprite {
            image : asset_manager.images["sprites/bipedpotato.png"].clone(),
            ..Default::default()
        },
        PhysicsComponent::new(Vec2::ZERO),
        Opponent{}
    ));
}
fn enter_planned_attack(
    mut commands: Commands,
    mut battle_events: ResMut<BattleEvents>,
    mut progress: ResMut<Progress>,
    mut menu_transition: ResMut<MenuTransition>,
    mut bullet_board: ResMut<BulletBoard>,
    mut decisions: ResMut<Decisions>,
    mut dodging_manager : ResMut<DodgingPhaseManager>,
    asset_manager: Res<AssetManager>,
) {
    commands.run_system(decisions.remove_decisions.unwrap());
    menu_transition.new_state(MenuState::Dodging);
    let mut attack = battle_events.attacks[progress.turns as usize].clone();
    attack.enter(&mut commands);
    dodging_manager.attack = attack.attack;
    progress.turns += 1;
}


fn item() {}

fn check() {}
