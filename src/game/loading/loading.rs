use std::collections::HashMap;

use bevy::asset::UntypedHandle;
use bevy::prelude::*;

use crate::game::{
    animation::animation::Animation,
    data::data::{BoardLayout, Data, DialogueSet, setup_data},
    scene::internal::progress::Progress,
    state::state::AppState,
};

pub struct AssetManagerPlugin;
impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets.after(setup_data))
            .add_systems(Update, check_assets.run_if(in_state(AppState::Loading)))
            .init_resource::<AssetManager>()
            .init_state::<AppState>();
    }
}
#[derive(Clone)]
pub struct SoundAsset {
    pub sound: Handle<AudioSource>,
    pub volume: f32,
}
#[derive(Resource, Default)]
pub struct AssetManager {
    pub assets: Vec<UntypedHandle>,
    pub images: HashMap<String, Handle<Image>>,
    pub fonts: HashMap<String, Handle<Font>>,
    pub atlases: HashMap<String, Handle<TextureAtlasLayout>>,
    pub sounds: HashMap<String, SoundAsset>,
    pub animations: HashMap<String, HashMap<String, Animation>>,
    pub dialogue_storage: HashMap<String, DialogueSet>,
    pub board_layouts: HashMap<String, BoardLayout>,
}
impl AssetManager {
    pub fn check_ready(&mut self, asset_server: &Res<AssetServer>) -> bool {
        for i in 0..self.assets.len() {
            if !asset_server.is_loaded(self.assets[i].id()) {
                return false;
            }
        }
        return true;
    }
    pub fn load_asset<T>(&mut self, path: String, asset_server: &Res<AssetServer>) -> Handle<T>
    where
        T: Asset,
    {
        let asset: Handle<T> = asset_server.load(path);
        self.assets.push(asset.clone().untyped());
        return asset;
    }
}

fn load_assets(
    data: Res<Data>,
    mut asset_manager: ResMut<AssetManager>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut progress: ResMut<Progress>,
) {
    progress.turns = data.game.player.start_turn;
    progress.health = data.game.opponent_data.health;

    let images = &data.assets.images;
    let atlases = &data.assets.atlases;
    let sounds = &data.assets.sounds;
    let animations = &data.assets.animations;
    let dialogue = &data.game.dialogue;
    let fonts = &data.assets.fonts;
    let boards = &data.game.board_layouts.layouts;
    for i in 0..fonts.len() {
        let path = fonts[i].clone();
        let handle = asset_manager.load_asset(path.clone(), &asset_server);
        asset_manager.fonts.insert(path.clone(), handle);
        log::info!("loaded font {}", path);
    }

    for i in 0..images.len() {
        let path = images[i].clone();
        let handle = asset_manager.load_asset(path.clone(), &asset_server);
        asset_manager.images.insert(path.clone(), handle);
        log::info!("loaded image {}", path);
    }

    for i in 0..atlases.len() {
        let atlas = &atlases[i];

        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(atlas.size_x as u32, atlas.size_y as u32),
            atlas.frame_count as u32,
            1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        asset_manager
            .atlases
            .insert(atlas.name.clone(), texture_atlas_layout.clone());
        log::info!(
            "loaded {} atlas with size {} {} and frame count {}",
            atlas.name,
            atlas.size_x,
            atlas.size_y,
            atlas.frame_count
        );
    }

    for i in 0..sounds.len() {
        let sound = &sounds[i];

        let audio_source = asset_manager.load_asset(sound.path.clone(), &asset_server);

        asset_manager.sounds.insert(
            sound.name.clone(),
            SoundAsset {
                sound: audio_source.clone(),
                volume: sound.volume,
            },
        );

        log::info!("loaded {} sound", sound.name);
    }

    for i in 0..animations.len() {
        let animation_group = &animations[i];
        let mut animations = HashMap::new();
        for i in 0..animation_group.group.len() {
            let animation = &animation_group.group[i];
            log::info!("loaded {} animation", animation.name);
            animations.insert(animation.name.clone(), animation.clone());
        }
        asset_manager
            .animations
            .insert(animation_group.name.clone(), animations);

        log::info!("loaded {} animation bank", animation_group.name);
    }

    for i in 0..dialogue.dialogues.len() {
        let set = dialogue.dialogues[i].clone();
        asset_manager
            .dialogue_storage
            .insert(set.name.clone(), set.clone());
    }

    for i in 0..boards.len() {
        let board = boards[i].clone();
        asset_manager
            .board_layouts
            .insert(board.name.clone(), board);
    }
}

fn check_assets(
    mut state: ResMut<NextState<AppState>>,
    mut asset_manager: ResMut<AssetManager>,
    asset_server: Res<AssetServer>,
) {
    if asset_manager.check_ready(&asset_server) {
        state.set(AppState::Level);
    }
}
