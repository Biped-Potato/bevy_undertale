use bevy::prelude::*;
use serde::Deserialize;

use crate::game::{
    animation::animation::Animation,
    toml::toml::{load_contents, read_toml},
};

#[derive(Resource, Deserialize, Clone, Default)]
pub struct Data {
    pub player: PlayerData,
    pub assets: AssetData,
    pub dialogue: DialogueData,
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
    pub damage: i32,
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

    let contents = include_str!("../../../assets/data/assets.toml").to_string();

    let data: Option<Data> = read_toml(contents);
    log::info!("try loading data");
    if data.is_some() {
        log::info!("got data");
        let true_data = data.unwrap();
        *data_res = true_data;
    }
}
