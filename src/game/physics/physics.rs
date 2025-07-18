use bevy::prelude::*;

use crate::game::physics::{physics_object::PhysicsLogicPlugin, timestep::TimestepPlugin};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TimestepPlugin, PhysicsLogicPlugin));
    }
}
