use bevy::prelude::*;

pub struct TimestepPlugin;
impl Plugin for TimestepPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(30.0));
    }
}
