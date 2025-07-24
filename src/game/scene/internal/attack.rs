use bevy::{ecs::system::SystemId, prelude::*};
#[derive(Clone)]
pub struct Attack {
    pub enter_attack: Option<SystemId>,
    pub init_attack: Option<SystemId>,
    pub attack: Option<SystemId>,
    pub exit_attack: Option<SystemId>,
}

impl Attack {
    pub fn enter(&mut self, commands: &mut Commands) {
        commands.run_system(self.enter_attack.unwrap());
    }
}
