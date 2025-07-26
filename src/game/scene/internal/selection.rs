use bevy::prelude::*;

use crate::game::{
    animation::animation::Animator, data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, player::player::Player, scene::internal::{
        bullet_board::BulletBoard, decisions::update_decisions, helpers::{despawn::DespawnInMenu, menu_item::MenuItem}, menu::MenuState, menu_transition::MenuTransition, stats::{HealthBar, HealthBarType}, text::TextBox
    }, sound::sound::SoundPlayer, state::state::AppState
};

#[derive(Resource)]
pub struct MenuSelect {
    selection: i32,
    selections: Vec<MenuOption>,
    button_width: f32,
    button_height: f32,
    spacing: f32,
}

impl MenuSelect {
    pub fn cycle(&mut self, dir: i32) {
        self.selection = (self.selection + dir).rem_euclid(self.selections.len() as i32);
    }
    pub fn get_option(&mut self) -> MenuOption {
        self.selections[self.selection as usize].clone()
    }
}
#[derive(Default, PartialEq, Component, Clone, Eq, Hash)]
pub enum MenuOption {
    #[default]
    Fight,
    Act,
    Item,
    Mercy,
}
pub struct MenuSelectPlugin;
impl Plugin for MenuSelectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuSelect {
            button_height: 42.,
            button_width: 110.,
            spacing: 45.0,
            selection: 0,
            selections: vec![
                MenuOption::Fight,
                MenuOption::Act,
                MenuOption::Item,
                MenuOption::Mercy,
            ],
        })
        .add_systems(OnEnter(AppState::Level), spawn_buttons)
        .add_systems(
            OnEnter(MenuState::Selection),
            (refresh_textbox).run_if(in_state(AppState::Level)),
        )
        .add_systems(PreStartup, init_bullet_board_size)
        .add_systems(
            Update,
            (update_selection, update_buttons).after(update_decisions).run_if(in_state(MenuState::Selection)),
        );
    }
}

pub fn init_bullet_board_size(mut bullet_board: ResMut<BulletBoard>) {
    bullet_board.set_absolute(565.0, 130.0, Vec2::new(0., -80.));
}

fn refresh_textbox(mut commands: Commands, text_box: Res<TextBox>) {
    commands.run_system(text_box.refresh_text.unwrap());
}
fn update_selection(
    mut menu: ResMut<MenuSelect>,
    input: Res<ButtonInput<KeyCode>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut text_box: ResMut<TextBox>,
    mut sounds : ResMut<SoundPlayer>,
    asset_manager : Res<AssetManager>,
) {
    if input.just_pressed(KeyCode::ArrowLeft) {
        menu.cycle(-1);
        sounds.play_sound_once_local(asset_manager.sounds["move_menu"].clone());
    }
    if input.just_pressed(KeyCode::ArrowRight) {
        menu.cycle(1);
        sounds.play_sound_once_local(asset_manager.sounds["move_menu"].clone());
    }
    if input.just_pressed(KeyCode::KeyZ) {
        menu_transition.new_state(MenuState::Decision);
        sounds.play_sound_once_local(asset_manager.sounds["select"].clone());
        text_box.clear_box();
    }
}

fn update_buttons(
    menu: Res<MenuSelect>,
    data: Res<Data>,
    mut button_query: Query<(&mut MenuOption, &mut Animator, &mut Sprite, &mut Transform)>,
    mut player_query: Query<(&mut PhysicsComponent), With<Player>>,
) {
    let selection = &menu.selections[menu.selection as usize];
    for (mut menu_o, mut animator, mut sprite, mut t) in button_query.iter_mut() {
        animator.current_animation = "inactive".to_string();
        sprite.color = Color::srgb(1.0, 0.5, 39. / 255.);
        if *selection == *menu_o {
            animator.current_animation = "hover".to_string();
            sprite.color = Color::srgb(1.0, 1.0, 64. / 255.);
            if let Ok(mut p) = player_query.single_mut() {
                p.position.y = t.translation.y;
                p.position.x = t.translation.x - menu.button_width / 2.0
                    + 8.0
                    + data.game.player.sprite_size_x / 2.0;
            }
        }
    }
}
pub fn spawn_buttons(
    menu: Res<MenuSelect>,
    mut bullet_board: ResMut<BulletBoard>,
    mut commands: Commands,
    asset_manager: Res<AssetManager>,
    data: Res<Data>,
) {
    let mut current_pos = -bullet_board.width / 2.0 - bullet_board.border + menu.button_width / 2.0;
    let mut sprites = vec![
        "sprites/fightbutton.png",
        "sprites/actbutton.png",
        "sprites/itembutton.png",
        "sprites/mercybutton.png",
    ];

    let mut spacing = vec![43.0, 50.0, 45.0, 0.0];
    for i in 0..menu.selections.len() {
        commands.spawn((
            Transform {
                translation: Vec3::new(current_pos.floor(), -213.0, 0.),
                ..Default::default()
            },
            Sprite {
                image: asset_manager.images[sprites[i]].clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: asset_manager.atlases["button"].clone(),
                    index: 0,
                }),
                ..Default::default()
            },
            Animator {
                current_animation: "inactive".to_string(),
                animation_bank: asset_manager.animations["button"].clone(),
                ..Default::default()
            },
            menu.selections[i].clone(),
            MenuItem,
        ));
        current_pos += spacing[i] + menu.button_width;
    }
}
