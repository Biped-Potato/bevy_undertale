use bevy::prelude::*;
use serde::Deserialize;

use crate::game::{
    animation::animation::Animation,
    toml::toml::{load_contents, read_toml},
};

#[derive(Resource, Deserialize, Clone, Default)]
pub struct Data {
    pub assets: AssetData,
    pub game: GameData,
}

#[derive(Deserialize, Clone, Default)]
pub struct GameData {
    pub player: PlayerData,
    pub dialogue: DialogueData,
    pub battle: BattleData,
    pub fight_bar: FightBarData,
    pub board_layouts: BoardLayouts,
    pub opponent_data: OpponentData,
}

#[derive(Deserialize, Clone, Default)]
pub struct OpponentData {
    pub height: f32,
    pub width: f32,
    pub health: i32,

    pub at: i32,
    pub df: i32,

    pub death_time: f32,
    pub dust_time : f32,
    pub dust_life : f32,
}
#[derive(Deserialize, Clone, Default)]
pub struct FightBarData {
    pub fade_time: f32,
    pub attack_animation: f32,
}
#[derive(Deserialize, Clone, Default)]
pub struct BattleData {
    pub dialogues: Vec<String>,
}
#[derive(Deserialize, Clone, Default)]
pub struct BoardLayouts {
    pub layouts: Vec<BoardLayout>,
}
#[derive(Deserialize, Clone, Default)]
pub struct BoardLayout {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize, Clone, Default)]
pub struct DialogueData {
    pub dialogues: Vec<DialogueSet>,
}

#[derive(Deserialize, Clone, Default)]
pub struct DialogueSet {
    pub name: String,
    pub dialogue: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct PlayerData {
    pub name: String,
    pub half_size_x: f32,
    pub half_size_y: f32,
    pub columns: i32,
    pub rows: i32,
    pub sprite_size_x: f32,
    pub sprite_size_y: f32,
    pub speed: f32,
    pub sprite: String,
    pub health: i32,
    pub iframes: f32,
    pub attack_speed: f32,
    pub start_turn: i32,

    pub at: i32,
    pub df: i32,
}

#[derive(Deserialize, Clone)]
pub struct TextureAtlasData {
    pub name: String,
    pub size_x: f32,
    pub size_y: f32,
    pub frame_count: i32,
}
#[derive(Deserialize, Clone)]
pub struct SoundData {
    pub name: String,
    pub path: String,
    pub volume: f32,
}

#[derive(Deserialize, Clone, Default)]
pub struct AssetData {
    pub images: Vec<String>,
    pub atlases: Vec<TextureAtlasData>,
    pub sounds: Vec<SoundData>,
    pub animations: Vec<AnimationGroup>,
    pub fonts: Vec<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct AnimationGroup {
    pub name: String,
    pub group: Vec<Animation>,
}
pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Data>().add_systems(Startup, setup_data);
    }
}

pub fn setup_data(mut commands: Commands, mut data_res: ResMut<Data>) {
    //let contents = load_contents("assets/data/data.toml".to_string());

    let asset_contents = include_str!("../../../assets/data/assets.toml").to_string();
    let asset_data: Option<AssetData> = read_toml(asset_contents);

    let contents = include_str!("../../../assets/data/data.toml").to_string();
    let data: Option<GameData> = read_toml(contents);

    log::info!("try loading data");
    if asset_data.is_some() {
        log::info!("got asset data");
        let asset_unwrapped = asset_data.unwrap();
        data_res.assets = asset_unwrapped;
    }
    if data.is_some() {
        log::info!("got game data");
        let data_unwrapped = data.unwrap();
        data_res.game = data_unwrapped;
    }
}
