use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};

use crate::game::scene::selection::MenuOption;

pub struct DecisionPlugin;
impl Plugin for DecisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Decisions>()
            .add_systems(Startup,init_decisions);
    }
}
pub struct Decision {
    pub display : String,
    pub system : SystemId,
}

pub struct DecisionMenu {
    pub left_column : Vec<Decision>,
    pub right_column : Vec<Decision>,
}

#[derive(Resource, Default)]
pub struct Decisions {
    pub menu : HashMap<MenuOption,DecisionMenu>,
}

fn init_decisions(
    mut decisions : ResMut<Decisions>
) {
    
}