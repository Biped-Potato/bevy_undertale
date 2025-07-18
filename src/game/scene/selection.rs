use bevy::prelude::*;

use crate::game::{animation::animation::Animator, loading::loading::AssetManager, scene::bullet_board::BulletBoard, state::state::AppState};

#[derive(Resource)]
pub struct MenuSelect {
    selection : i32,
    selections : Vec<MenuOption>,
}

impl MenuSelect {
    fn cycle(&mut self) {
        self.selection = (self.selection + 1) % self.selections.len() as i32;
    }
}
#[derive(Default, PartialEq, Component, Clone)]
pub enum MenuOption {
    #[default]
    Fight,
    Act,
    Item,
    Mercy
}
pub struct MenuSelectPlugin;
impl Plugin for MenuSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MenuSelect {
                selection : 0,
                selections : vec![MenuOption::Fight, MenuOption::Act, MenuOption::Item, MenuOption::Mercy]
            })
            .add_systems(OnEnter(AppState::Level), spawn_buttons)
            .add_systems(PreStartup, init_bullet_board_size);
    }
}

pub fn init_bullet_board_size(
    mut bullet_board : ResMut<BulletBoard>,
) {
    bullet_board.set_absolute(565.0, 130.0, Vec2::new(0.,-80.));
}

pub fn spawn_buttons(
    menu : Res<MenuSelect>,
    mut bullet_board : ResMut<BulletBoard>,
    mut commands : Commands,
    asset_manager : Res<AssetManager>,
) {
    let button_width = 110.0;
    let button_height = 42.0;
    let spacing = 45.0;
    let mut current_pos = -bullet_board.width / 2.0  - bullet_board.border + button_width / 2.0;
    let mut sprites = vec![
        "sprites/fightbutton.png",
        "sprites/actbutton.png",
        "sprites/itembutton.png",
        "sprites/mercybutton.png"
    ];
    for i in 0..menu.selections.len() {
        commands.spawn(
            (
                Transform {
                    translation : Vec3::new(current_pos.round(),-213.0,0.),
                    ..Default::default()
                },
                Sprite {
                    image : asset_manager.images[sprites[i]].clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: asset_manager.atlases["button"].clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Animator {
                    current_animation : "inactive".to_string(),
                    animation_bank : asset_manager.animations["button"].clone(),
                    ..Default::default()
                },
                menu.selections[i].clone()
            )
        );
        current_pos += spacing + button_width;
    }
    
}