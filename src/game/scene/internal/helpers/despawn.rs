use bevy::{ecs::system::SystemId, prelude::*};

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_despawn);
    }
}

#[derive(Component)]
pub struct DespawnInTime {
    pub effect: Option<SystemId>,
    pub time: f32,
}

fn update_despawn(
    mut commands: Commands,
    mut despawn_query: Query<(&mut DespawnInTime, Entity)>,
    time: Res<Time<Fixed>>,
) {
    for (mut d, e) in despawn_query.iter_mut() {
        d.time -= time.delta_secs();
        if d.time <= 0. {
            if d.effect.is_some() {
                commands.run_system(d.effect.unwrap());
            }
            commands.entity(e).despawn();
        }
    }
}
