use bevy::{ecs::system::SystemId, prelude::*};

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_despawn,update_opacity));
    }
}

#[derive(Component)]
pub struct DespawnInTime {
    pub effect: Option<SystemId>,
    pub time: f32,
    pub timer : f32,
}
impl DespawnInTime {
    pub fn new(time : f32,effect : Option<SystemId>) ->DespawnInTime {
        DespawnInTime { effect: effect , time: time,timer : time }
    }
}
#[derive(Component)]
pub struct DespawnInMenu;

#[derive(Component)]
pub struct OpacityFromTimer;

fn update_opacity(
    mut commands: Commands,
    mut despawn_query: Query<(&mut DespawnInTime, &mut Sprite, Entity),With<OpacityFromTimer>>,
    time: Res<Time<Fixed>>,
) {
    for (mut d, mut s,e) in despawn_query.iter_mut() {
        s.color.set_alpha(d.timer / d.time);
    }
}

fn update_despawn(
    mut commands: Commands,
    mut despawn_query: Query<(&mut DespawnInTime, Entity)>,
    time: Res<Time<Fixed>>,
) {
    for (mut d, e) in despawn_query.iter_mut() {
        d.timer -= time.delta_secs();
        if d.timer <= 0. {
            if d.effect.is_some() {
                commands.run_system(d.effect.unwrap());
            }
            commands.entity(e).despawn();
        }
    }
}

pub fn despawn_objects(mut query: Query<(&mut DespawnInMenu, Entity)>, mut commands: Commands) {
    for (mut despawn, e) in query.iter_mut() {
        commands.entity(e).despawn();
    }
}
