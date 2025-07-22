use bevy::prelude::*;

pub struct PhysicsLogicPlugin;
impl Plugin for PhysicsLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, snap_objects);
    }
}

#[derive(Component)]
pub struct PhysicsComponent {
    pub position: Vec2,
}

fn snap_objects(mut query: Query<(&mut Transform, &mut PhysicsComponent)>) {
    for (mut t, mut p) in query.iter_mut() {
        t.translation.x = p.position.x.floor();
        t.translation.y = p.position.y.floor();
    }
}

impl PhysicsComponent {
    pub fn new(position: Vec2) -> PhysicsComponent {
        PhysicsComponent { position: position }
    }
}
