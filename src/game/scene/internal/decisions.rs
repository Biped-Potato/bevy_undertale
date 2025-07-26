use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*, text::TextBounds};

use crate::game::{
    data::data::Data,
    loading::loading::AssetManager,
    physics::physics_object::PhysicsComponent,
    player::player::Player,
    scene::internal::{
        bullet_board::{self, BulletBoard},
        helpers::menu_item::MenuItem,
        menu::MenuState,
        menu_transition::MenuTransition,
        progress::Progress,
        selection::{MenuOption, MenuSelect},
        text::TextBox,
    }, sound::sound::SoundPlayer,
};

pub struct DecisionPlugin;
impl Plugin for DecisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Decisions>()
            .add_systems(
                OnEnter(MenuState::Decision),
                init_decision_menu.before(update_decisions),
            )
            .add_systems(
                Update,
                (
                    update_decisions,
                    update_decision_spawning.before(update_decisions),
                )
                    .run_if(in_state(MenuState::Decision)),
            )
            .add_systems(
                Update,
                (update_decision_display.before(update_decisions).after(update_decision_spawning)).run_if(in_state(MenuState::Decision)),
            )
            .add_systems(Startup, init_decisions);
    }
}
#[derive(Clone)]
pub struct Decision {
    pub display: String,
    pub system: Option<SystemId>,

    pub hover: Option<SystemId>,

    pub submenu: Option<DecisionMenu>,
}
#[derive(Default, Clone)]
pub struct DecisionMenu {
    pub left_column: Vec<Decision>,
    pub right_column: Vec<Decision>,
}
#[derive(Default, Clone)]
pub struct DecisionEntities {
    pub left_column: Vec<Entity>,
    pub right_column: Vec<Entity>,
}
#[derive(Resource)]
pub struct Decisions {
    pub menu: HashMap<MenuOption, DecisionMenu>,
    pub decision_menu: Option<DecisionMenu>,
    pub menu_entities: DecisionEntities,
    pub remove_decisions: Option<SystemId>,

    pub switch_menu: bool,
    pub submenu: bool,
    pub selection: i32,
    pub side: i32,

    pub spacing: f32,
    pub increment: f32,
}
impl Decisions {
    fn reset_selections(&mut self) {
        self.selection = 0;
        self.side = 0;
        self.menu_entities.left_column.clear();
        self.menu_entities.right_column.clear();
    }
    fn enter_menu(&mut self, menu: DecisionMenu) {
        self.decision_menu = Some(menu);
        self.switch_menu = true;
        self.reset_selections();
    }
    pub fn spawn_decision(
        &mut self,
        mut commands: &mut Commands,
        bullet_board: &Res<BulletBoard>,
        position: Vec2,
        text_font: TextFont,
        decision: Decision,
    ) -> Entity {
        let menu = self.decision_menu.clone().unwrap();
        let parent = commands
            .spawn(Transform::from_translation(
                bullet_board.position.extend(0.0),
            ))
            .id();

        let e = commands
            .spawn((
                Text2d::new("* ".to_string() + decision.display.as_str()),
                TextBounds::from(Vec2::new(bullet_board.width, bullet_board.height)),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                Name::new("decision"),
                text_font.clone(),
                Transform::from_translation((position).extend(1.0)),
                DecisionMarker {},
                MenuItem,
            ))
            .id();
        commands.entity(parent).add_child(e);
        return e;
    }
    pub fn vertical_cycle(&mut self, i: i32) {
        let decision_menu = self.decision_menu.as_ref().unwrap();
        let mut column_size = decision_menu.left_column.len();
        if self.side == 1 {
            column_size = decision_menu.right_column.len();
        }
        self.selection = (self.selection + i).rem_euclid(column_size as i32);
    }
    pub fn horizontal_cycle(&mut self, i: i32) {
        let decision_menu = self.decision_menu.as_ref().unwrap();
        if decision_menu.right_column.len() == 0 {
            self.side = 0;
        } else {
            self.side = (self.side + i).rem_euclid(2);
        }

        self.vertical_cycle(0);
    }
    pub fn get_decision(&mut self) -> (Decision, Entity) {
        let decision_menu = self.decision_menu.as_ref().unwrap();
        let entity_menu = &self.menu_entities;
        let mut decisions = &decision_menu.left_column;
        let mut entities = &entity_menu.left_column;
        if self.side == 1 {
            decisions = &decision_menu.right_column;
            entities = &entity_menu.right_column;
        }
        (
            decisions[self.selection as usize].clone(),
            entities[self.selection as usize],
        )
    }
}

impl Decision {
    pub fn new(display: String, system: SystemId) -> Decision {
        return Decision {
            display: display,
            system: Some(system),
            submenu: None,
            hover: None,
        };
    }
    pub fn new_with_hover(display: String, system: SystemId, hover: SystemId) -> Decision {
        return Decision {
            display: display,
            system: Some(system),
            submenu: None,
            hover: Some(hover),
        };
    }
    pub fn new_with_menu(display: String, submenu: Option<DecisionMenu>) -> Decision {
        return Decision {
            display: display,
            system: None,
            submenu: submenu,
            hover: None,
        };
    }
}

pub fn remove_decisions(
    mut commands: Commands,
    decision_query: Query<(Entity), With<DecisionMarker>>,
) {
    for (e) in decision_query.iter() {
        commands.entity(e).despawn();
    }
}
fn init_decision_menu(mut menu_select: ResMut<MenuSelect>, mut decisions: ResMut<Decisions>) {
    decisions.increment = 32.;
    decisions.spacing = 256.;

    let option = menu_select.get_option();
    let menu = decisions.menu[&option].clone();
    decisions.enter_menu(menu);
}

#[derive(Component)]
pub struct DecisionMarker {}
fn update_decision_display(
    mut decisions: ResMut<Decisions>,
    mut decision_query: Query<(&mut Transform), Without<Player>>,
    mut player_query: Query<(&mut PhysicsComponent, &mut Player)>,
    mut b_board: Res<BulletBoard>,
    data: Res<Data>,
) {
    let d = decisions.get_decision();
    if let Ok((mut physics, mut player)) = player_query.single_mut() {
        if let Ok(mut t) = decision_query.get_mut(d.1) {
            physics.position.x = data.game.player.sprite_size_x / 2.0 + b_board.position.x
                - b_board.width / 2.0
                + 27.0
                + decisions.spacing * decisions.side as f32;
            physics.position.y =
                -data.game.player.sprite_size_y / 2.0 + b_board.position.y + b_board.height / 2.0
                    - 23.0
                    - decisions.increment * decisions.selection as f32;
        }
    }
}
pub fn update_decisions(
    mut commands: Commands,
    mut decisions: ResMut<Decisions>,
    mut menu_select: ResMut<MenuSelect>,
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut text_box: ResMut<TextBox>,
    data: Res<Data>,
    progress: Res<Progress>,
    mut sounds : ResMut<SoundPlayer>,
    asset_manager : Res<AssetManager>,
) {
    if decisions.decision_menu.is_some() {
        let mut vertical = 0;
        let mut horizontal = 0;
        let decision = decisions.get_decision();
        if decision.0.hover.is_some() {
            commands.run_system(decision.0.hover.unwrap());
        }
        if keys.just_pressed(KeyCode::KeyZ) {
            if decision.0.submenu.is_some() {
                decisions.enter_menu(decision.0.submenu.unwrap());
                decisions.submenu = true;
            } else {
                commands.run_system(decision.0.system.unwrap());
            }
            sounds.play_sound_once_local(asset_manager.sounds["select"].clone());
        } else if keys.just_pressed(KeyCode::KeyX) {
            if decisions.submenu {
                decisions.submenu = false;
                let option = menu_select.get_option();
                let menu = decisions.menu[&option].clone();
                decisions.enter_menu(menu);
            } else {
                commands.run_system(decisions.remove_decisions.unwrap());
                menu_transition.new_state(MenuState::Selection);
            }
        } else {
            if keys.just_pressed(KeyCode::ArrowLeft) {
                horizontal -= 1;
                sounds.play_sound_once_local(asset_manager.sounds["move_menu"].clone());
            }
            if keys.just_pressed(KeyCode::ArrowRight) {
                horizontal += 1;
                sounds.play_sound_once_local(asset_manager.sounds["move_menu"].clone());
            }

            if keys.just_pressed(KeyCode::ArrowUp) {
                vertical -= 1;
                sounds.play_sound_once_local(asset_manager.sounds["move_menu"].clone());
            }
            if keys.just_pressed(KeyCode::ArrowDown) {
                vertical += 1;
                sounds.play_sound_once_local(asset_manager.sounds["move_menu"].clone());
            }
            decisions.vertical_cycle(vertical);
            decisions.horizontal_cycle(horizontal);
        }
    }
}

fn update_decision_spawning(
    mut commands: Commands,
    mut decisions: ResMut<Decisions>,
    bullet_board: Res<BulletBoard>,
    asset_manager: Res<AssetManager>,
) {
    if decisions.switch_menu {
        decisions.switch_menu = false;
        let text_font = TextFont {
            font: asset_manager.fonts["fonts/DTM-Mono.ttf"].clone(),
            font_size: 26.0,
            font_smoothing: bevy::text::FontSmoothing::None,
            ..Default::default()
        };
        commands.run_system(decisions.remove_decisions.unwrap());

        let menu = decisions.decision_menu.clone().unwrap();

        for i in 0..menu.left_column.len() {
            let mut pos = Vec2::new(14.1 + 49., -16. - decisions.increment * i as f32);
            let e = decisions.spawn_decision(
                &mut commands,
                &bullet_board,
                pos,
                text_font.clone(),
                menu.left_column[i].clone(),
            );
            decisions.menu_entities.left_column.push(e);
        }
        for i in 0..menu.right_column.len() {
            let mut pos = Vec2::new(
                14.1 + 49. + decisions.spacing,
                -16. - decisions.increment * i as f32,
            );
            let e = decisions.spawn_decision(
                &mut commands,
                &bullet_board,
                pos,
                text_font.clone(),
                menu.right_column[i].clone(),
            );
            decisions.menu_entities.right_column.push(e);
        }
    }
}

fn init_decisions(mut decisions: ResMut<Decisions>) {}
