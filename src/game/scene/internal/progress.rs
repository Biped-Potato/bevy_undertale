use bevy::prelude::*;

pub struct ProgressPlugin;
impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progress>();
    }
}

#[derive(Resource)]
pub struct Progress {
    pub turns: i32,
    pub health: i32,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            turns: 0,
            health: 1,
        }
    }
}
