use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*, text::TextBounds};

use crate::game::{loading::loading::AssetManager, scene::{bullet_board::{self, BulletBoard}, menu::MenuState, selection::{MenuOption, MenuSelect}}};

pub struct DecisionPlugin;
impl Plugin for DecisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Decisions>()
            .add_systems(OnEnter(MenuState::Decision), init_decision_menu)
            .add_systems(Update,(update_decision_spawning,update_decisions).run_if(in_state(MenuState::Decision)))
            .add_systems(Startup,init_decisions);
    }
}
#[derive(Clone)]
pub struct Decision {
    pub display : String,
    pub system : Option<SystemId>,
    pub submenu : Option<DecisionMenu>,
}
#[derive(Default, Clone)]
pub struct DecisionMenu {
    pub left_column : Vec<Decision>,
    pub right_column : Vec<Decision>,
}
#[derive(Default, Clone)]
pub struct DecisionEntities {
    pub left_column : Vec<Entity>,
    pub right_column : Vec<Entity>,
}
#[derive(Resource)]
pub struct Decisions {
    pub menu : HashMap<MenuOption,DecisionMenu>,
    pub decision_menu : Option<DecisionMenu>,
    pub menu_entities : DecisionEntities,

    pub switch_menu : bool,
    pub selection : i32,
    pub side : i32,
}
impl Decisions {
    fn reset_selections(&mut self) {
        self.selection = 0;
        self.side = 0;
        self.menu_entities.left_column.clear();
        self.menu_entities.right_column.clear();
    }
    fn enter_menu(&mut self, menu : DecisionMenu) {
        self.decision_menu = Some(menu);
        self.switch_menu = true;
        self.reset_selections();
    }
    pub fn spawn_decision(
        &mut self,
        mut commands : &mut Commands,
        bullet_board : &Res<BulletBoard>,
        position : Vec2,
        text_font : TextFont,
        i : usize,
    ) -> Entity {
        let menu = self.decision_menu.clone().unwrap();
        commands.spawn((
            Text2d::new("* ".to_string() + menu.left_column[i].display.as_str()),
            TextBounds::from(Vec2::new(bullet_board.width,bullet_board.height)),
            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
            Name::new("decision"),
            text_font.clone(),
            Transform::from_translation((position).extend(1.0)),
            DecisionMarker {},
        )).id()
    }
    pub fn vertical_cycle(&mut self, i : i32) {
        let decision_menu = self.decision_menu.as_ref().unwrap();
        let mut column_size = decision_menu.left_column.len();
        if self.side == 1 {
            column_size = decision_menu.right_column.len();
        }
        self.selection = (self.selection + 1).rem_euclid(column_size as i32);
    }
    pub fn horizontal_cycle(&mut self, i : i32) {
        self.side = (self.side + i) % 2;
        self.vertical_cycle(0);
    }
}

impl Decision {
    pub fn new(display : String, system : SystemId) -> Decision {
        return Decision {
            display : display,
            system : Some(system),
            submenu : None,
        };
    }
    pub fn new_with_menu(display : String,submenu : Option<DecisionMenu>) -> Decision {
        return Decision {
            display : display,
            system : None,
            submenu : submenu,
        };
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
            menu : menu,
            decision_menu : None,
            menu_entities : default(),
            side : 0,
            selection : 0,
            switch_menu : false,
        }
    }
}

fn init_decision_menu(
    mut menu_select : ResMut<MenuSelect>,
    mut decisions : ResMut<Decisions>,
) {
    let option = menu_select.get_option();
    let menu = decisions.menu[&option].clone();
    decisions.enter_menu(menu);
}

#[derive(Component)]
pub struct DecisionMarker {

}

fn update_decisions(
    mut decisions : ResMut<Decisions>,
    keys : Res<ButtonInput<KeyCode>>,
) {
    let mut vertical = 0;
    let mut horizontal = 0;
    if keys.just_pressed(KeyCode::ArrowLeft) {
        horizontal -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        horizontal += 1;
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        vertical -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        vertical += 1;
    }

    decisions.vertical_cycle(vertical);
    decisions.horizontal_cycle(horizontal);
}

fn update_decision_spawning(
    mut commands : Commands,
    mut decisions : ResMut<Decisions>,
    mut decision_query : Query<(&mut DecisionMarker, Entity)>,
    bullet_board : Res<BulletBoard>,
    asset_manager : Res<AssetManager>,
) {
    if decisions.switch_menu {
        decisions.switch_menu = false;
        let text_font = TextFont {
            font: asset_manager.fonts["fonts/DTM-Mono.ttf"].clone(),
            font_size: 26.0,
            font_smoothing : bevy::text::FontSmoothing::None,
            ..Default::default()
        };
        for (mut d, e) in decision_query.iter_mut() {
            commands.entity(e).despawn();
        }

        let menu = decisions.decision_menu.clone().unwrap();
        let increment = 32.0;
        let spacing = 256.0;

        for i in 0..menu.left_column.len() {
            let mut pos = Vec2::new(14.1 + 18., -16. - 32. * i as f32) + bullet_board.position;
            let e = decisions.spawn_decision(&mut commands, &bullet_board, pos, text_font.clone(), i);
            decisions.menu_entities.left_column.push(e);
        }
        for i in 0..menu.right_column.len() {
            let mut pos = Vec2::new(14.1 + 18. + spacing, -16. - 32. * i as f32) + bullet_board.position;
            let e = decisions.spawn_decision(&mut commands, &bullet_board, pos, text_font.clone(), i);
            decisions.menu_entities.right_column.push(e);
        }
    }
}

fn init_decisions(
    mut decisions : ResMut<Decisions>
) {
    
}

fn start_fight() {

}

fn talk() {

}

fn check() {

}