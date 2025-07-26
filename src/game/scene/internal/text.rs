use bevy::{ecs::system::SystemId, prelude::*, text::TextBounds};

use crate::game::{
    camera::render_layers::RenderLayerStorage, data::data::{Data, DialogueSet}, loading::loading::AssetManager, player::player::Player, scene::internal::{
        bullet_board::{spawn_bullet_board, BulletBoard, BulletBoardFill},
        helpers::menu_item::MenuItem,
        menu::MenuState,
        menu_transition::MenuTransition,
        progress::Progress,
    }, sound::sound::SoundPlayer, state::state::AppState
};

pub struct TextBoxPlugin;
impl Plugin for TextBoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextBox>()
            .add_systems(
                OnEnter(AppState::Level),
                spawn_text.after(spawn_bullet_board),
            )
            .add_systems(OnEnter(MenuState::Text), hide_player)
            .add_systems(OnExit(MenuState::Text), show_player)
            .add_systems(Update, update_dialogue.run_if(in_state(MenuState::Text)))
            .add_systems(FixedUpdate, update_text.run_if(in_state(AppState::Level)));
    }
}

#[derive(Resource)]
pub struct TextBox {
    pub text: String,
    pub timer: f32,
    pub velocity: f32,
    pub entity: Option<Entity>,
    pub refresh_text: Option<SystemId>,

    //set these 2 for different actions
    pub dialogue: Option<DialogueSet>,
    pub dialogue_end_event: Option<SystemId>,
    pub dialogue_index: i32,

    pub prev_length : i32,
}

impl TextBox {
    pub fn queue_event(&mut self, dialogue: DialogueSet, event: SystemId) {
        self.dialogue = Some(dialogue);
        self.dialogue_index = 0;
        self.dialogue_end_event = Some(event);
        self.prev_length = 0;
        self.timer = 0.;
    }
}

impl FromWorld for TextBox {
    fn from_world(world: &mut World) -> Self {
        let refresh_text = world.register_system(refresh_text);

        Self {
            prev_length : 0,
            dialogue_index: 0,
            dialogue_end_event: None,
            dialogue: None,
            refresh_text: Some(refresh_text),
            text: "".to_string(),
            timer: 0.,
            velocity: 30.0,
            entity: None,
        }
    }
}
fn refresh_text(
    asset_manager: Res<AssetManager>,
    data: Res<Data>,
    progress: Res<Progress>,
    mut text_box: ResMut<TextBox>,
) {
    let dialogue_list = &data.game.battle.dialogues;
    let dialogue_name = &dialogue_list[progress.turns as usize];
    text_box.prev_length = 0;
    text_box.set_text(
        "* ".to_string()
            + asset_manager.dialogue_storage[dialogue_name].dialogue[0]
                .clone()
                .as_str(),
    );
}
impl TextBox {
    pub fn clear_box(&mut self) {
        self.text = "".to_string();
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.timer = 0.;
    }
}
#[derive(Component)]
pub struct TextBoxText;

fn update_dialogue(
    mut text_box: ResMut<TextBox>,
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_transition: ResMut<MenuTransition>,
    mut commands: Commands,
) {
    if text_box.dialogue.is_some() {
        let i = text_box.dialogue_index;
        let dialogue = text_box.dialogue.clone().unwrap();
        text_box.text = "* ".to_string() + dialogue.dialogue[i as usize].clone().as_str();
        if keys.just_pressed(KeyCode::KeyX) {
            text_box.timer = 1000.0;
        }
        if keys.just_pressed(KeyCode::KeyZ) {
            text_box.timer = 0.;
            text_box.dialogue_index += 1;
        }
        let len = dialogue.dialogue.len();

        if text_box.dialogue_index >= len as i32 {
            text_box.dialogue = None;
            text_box.clear_box();
            commands.run_system(text_box.dialogue_end_event.unwrap());
        }
    }
}
fn hide_player(mut player_query: Query<(&mut Visibility), With<Player>>) {
    if let Ok(mut v) = player_query.single_mut() {
        *v = Visibility::Hidden;
    }
}

fn show_player(mut player_query: Query<(&mut Visibility), With<Player>>) {
    if let Ok(mut v) = player_query.single_mut() {
        *v = Visibility::Visible;
    }
}
fn update_text(
    mut writer: Text2dWriter, mut text_box: ResMut<TextBox>,
    time: Res<Time<Fixed>>,
    mut sounds : ResMut<SoundPlayer>,    
    asset_manager: Res<AssetManager>
) {
    if text_box.entity.is_some() {
        text_box.timer += time.delta_secs();
        let mut length = (text_box.velocity * text_box.timer) as i32;
        length = i32::clamp(length, 0, text_box.text.len() as i32);


        if (length as i32 - text_box.prev_length) > 1 {

            text_box.prev_length = length as i32;
            
            sounds.play_sound_once_local(asset_manager.sounds["text"].clone());
        }
        let s = &text_box.text;
        let display = &s[0..(length as usize)];
        *writer.text(text_box.entity.unwrap(), 0) = display.to_string();
    }
}

fn spawn_text(
    mut commands: Commands,
    mut bullet_board: ResMut<BulletBoard>,
    mut text_box: ResMut<TextBox>,
    asset_manager: Res<AssetManager>,
    render_layers: Res<RenderLayerStorage>,
) {
    commands.run_system(text_box.refresh_text.unwrap());
    let text_font = TextFont {
        font: asset_manager.fonts["fonts/DTM-Mono.ttf"].clone(),
        font_size: 26.0,
        font_smoothing: bevy::text::FontSmoothing::None,
        ..Default::default()
    };
    let mut pos = Vec2::new(14.1, -16.);
    let p = commands
        .spawn(Transform::from_translation(
            bullet_board.position.extend(0.0),
        ))
        .with_children(|builder| {
            let e = builder
                .spawn((
                    Text2d::new(""),
                    TextBounds::from(Vec2::new(bullet_board.width, bullet_board.height)),
                    TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                    Name::new("text"),
                    text_font,
                    Transform::from_translation((pos).extend(1.0)),
                    TextBoxText,
                    MenuItem,
                ))
                .id();
            text_box.entity = Some(e);
        });
}
