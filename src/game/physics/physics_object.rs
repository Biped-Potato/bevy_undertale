use bevy::prelude::*;

pub struct PhysicsLogicPlugin;
impl Plugin for PhysicsLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_physics)
            .add_systems(PostUpdate, snap_objects);
    }
}

#[derive(Component)]
pub struct PhysicsComponent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub half_hitbox: Vec2,
    pub half_collision_box: Vec2,
}

impl Default for PhysicsComponent {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            half_hitbox: Vec2::ZERO,
            half_collision_box: Vec2::ZERO,
        }
    }
}
impl PhysicsComponent {
    pub fn new(position: Vec2) -> PhysicsComponent {
        PhysicsComponent {
            position: position,
            ..Default::default()
        }
    }
    pub fn new_full(
        position: Vec2,
        velocity: Vec2,
        half_hitbox: Vec2,
        half_collision_box: Vec2,
    ) -> PhysicsComponent {
        PhysicsComponent {
            position: position,
            velocity: velocity,
            half_collision_box: half_collision_box,
            half_hitbox: half_hitbox,
        }
    }
}

fn snap_objects(mut query: Query<(&mut Transform, &mut PhysicsComponent)>) {
    for (mut t, mut p) in query.iter_mut() {
        t.translation.x = p.position.x.floor();
        t.translation.y = p.position.y.floor();
    }
}

fn update_physics(mut query: Query<(&mut PhysicsComponent)>) {
    for (mut p) in query.iter_mut() {
        let v = p.velocity;
        p.position += v;
    }
}
