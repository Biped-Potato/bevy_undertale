use std::collections::HashMap;
use std::time::Duration;

use bevy::audio::Volume;
use bevy::prelude::*;

use crate::game::loading::loading::SoundAsset;

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (play_sounds))
            .insert_resource(SoundPlayer {
                sounds: Vec::new(),
                looped_sounds: Vec::new(),
                looped_queue: Vec::new(),
            });
    }
}
#[derive(Clone)]
pub struct Sound {
    pub src: Option<Handle<AudioSource>>,
    pub volume: f32,
    pub position: Option<Vec2>,
}
impl Default for Sound {
    fn default() -> Self {
        Sound {
            src: None,
            volume: 1.,
            position: None,
        }
    }
}
#[derive(Resource)]
pub struct SoundPlayer {
    pub sounds: Vec<Sound>,
    pub looped_sounds: Vec<Sound>,
    pub looped_queue: Vec<Sound>,
}
impl SoundPlayer {
    pub fn play_sound_once(&mut self, sound: SoundAsset, position: Vec2) {
        self.sounds.push(Sound {
            src: Some(sound.sound),
            volume: sound.volume,
            position: Some(position),
        });
    }
    pub fn play_sound_once_local(&mut self, sound : SoundAsset) {
        self.sounds.push(Sound {
            src : Some(sound.sound),
            volume : sound.volume,
            position: None,
        });
    }
    pub fn play_sound_looped(&mut self, sound: SoundAsset) {
        self.looped_queue.push(Sound {
            src: Some(sound.sound),
            volume: sound.volume,
            position: None,
        });
    }
}
#[derive(Component)]
pub struct Song {}
#[derive(Component)]
pub struct SoundLife {
    pub life: f32,
}
pub fn play_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sound: ResMut<SoundPlayer>,
    mut audios: Res<Assets<AudioSource>>,
) {
    for i in 0..sound.sounds.len() {
        let mut spatial = false;
        let mut position = Vec2::ZERO;
        if sound.sounds[i].position.is_some() {
            spatial = true;
            position = sound.sounds[i].position.unwrap();
        }
        let e = commands
            .spawn(Transform {
                translation: Vec3::new(position.x, position.y, 0.),
                ..default()
            })
            .insert(AudioPlayer::new(sound.sounds[i].src.clone().unwrap()))
            .insert(
                PlaybackSettings::DESPAWN
                    .with_volume(Volume::Linear(sound.sounds[i].volume as f32))
                    .with_spatial(spatial),
            )
            .insert(Name::new("Sound"))
            .id();
    }
    for i in 0..sound.looped_queue.len() {
        
        let e = commands
            .spawn(Transform { ..default() })
            .insert(AudioPlayer::new(sound.looped_queue[i].src.clone().unwrap()))
            .insert(
                PlaybackSettings::LOOP
                    .with_volume(Volume::Linear(sound.looped_queue[i].volume as f32)),
            ).insert(Song{})
            .id();
        //sound.looped_sounds.push(element);
    }
    if sound.sounds.len() != 0 {
        sound.sounds.clear();
    }
    if sound.looped_queue.len() != 0 {
        sound.looped_queue.clear();
    }
}

fn set_song_volume(
    mut song_query : Query<&mut AudioSink, With<Song>>,
) {
    if let Ok((mut a)) = song_query.single_mut() {
        a.set_volume(Volume::Linear(0.6));
    }
}