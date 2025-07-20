use bevy::{ecs::system::SystemId, prelude::*, text::TextBounds};

use crate::game::{camera::render_layers::RenderLayerStorage, data::data::Data, loading::loading::AssetManager, scene::{bullet_board::{spawn_bullet_board, BulletBoard, BulletBoardFill}, progress::Progress}, state::state::AppState};


pub struct TextBoxPlugin;
impl Plugin for TextBoxPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<TextBox>()
        .add_systems(OnEnter(AppState::Level),spawn_text.after(spawn_bullet_board))
        .add_systems(FixedUpdate,update_text.run_if(in_state(AppState::Level)));
    }
}

#[derive(Resource)]
pub struct TextBox {
    pub text : String,
    pub timer : f32,
    pub velocity : f32,
    pub entity : Option<Entity>,
    pub refresh_text : Option<SystemId>,
}

impl FromWorld for TextBox {
    fn from_world(world : &mut World) -> Self {
        let refresh_text = world.register_system(refresh_text);

        Self {
            refresh_text : Some(refresh_text),
            text : "".to_string(),
            timer : 0.,
            velocity : 30.0,
            entity : None,
        }
    }
}
fn refresh_text(
    asset_manager : Res<AssetManager>,
    data : Res<Data>,
    progress : Res<Progress>,
    mut text_box : ResMut<TextBox>,
) {
    let dialogue_list = &data.game.battle.dialogues;
    let dialogue_name = &dialogue_list[progress.turns as usize];
    text_box.set_text("* ".to_string() + asset_manager.dialogue_storage[dialogue_name].dialogue[0].clone().as_str());
}
impl TextBox {
    pub fn clear_box(&mut self) {
        self.text = "".to_string();
    }
    pub fn set_text(&mut self,text : String) {
        self.text = text;
        self.timer = 0.;
    }
    
}
#[derive(Component)]
pub struct TextBoxText;

fn update_text(
    mut writer : Text2dWriter,
    mut text_box : ResMut<TextBox>,
    time : Res<Time<Fixed>>,
) {
    if text_box.entity.is_some() {
        text_box.timer += time.delta_secs();
        let mut length = (text_box.velocity * text_box.timer) as i32;
        length = i32::clamp(length, 0, text_box.text.len() as i32);
        let s = &text_box.text;
        let display = &s[0..(length as usize)];
        *writer.text(text_box.entity.unwrap(), 0) = display.to_string();
    }
}

fn spawn_text(
    mut commands : Commands,
    mut bullet_board : ResMut<BulletBoard>,
    mut text_box : ResMut<TextBox>,
    asset_manager : Res<AssetManager>,
    render_layers : Res<RenderLayerStorage>,
) {
    commands.run_system(text_box.refresh_text.unwrap());
    let text_font = TextFont {
        font: asset_manager.fonts["fonts/DTM-Mono.ttf"].clone(),
        font_size: 26.0,
        font_smoothing : bevy::text::FontSmoothing::None,
        ..Default::default()
    };
    let mut pos = Vec2::new(14.1, -16.)+bullet_board.position;
    let e =commands.spawn((
        Text2d::new(""),
        TextBounds::from(Vec2::new(bullet_board.width,bullet_board.height)),
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        Name::new("text"),
        text_font,
        Transform::from_translation((pos).extend(1.0)),
        TextBoxText,
    )).id();
    text_box.entity = Some(e);
}