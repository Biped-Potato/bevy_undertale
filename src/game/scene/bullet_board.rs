use bevy::prelude::*;

use crate::game::{
    camera::render_layers::RenderLayerStorage, data::data::Data, loading::loading::AssetManager, physics::physics_object::PhysicsComponent, player::player::{player_movement, Player}, state::state::AppState
};

pub struct BulletBoardPlugin;
impl Plugin for BulletBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletBoard {
            width: 155.,
            height: 130.,
            position: Vec2::ZERO,
            border: 5.0,
        })
        .add_systems(OnEnter(AppState::Level), spawn_bullet_board)
        .add_systems(FixedUpdate, constrain_player.after(player_movement))
        .add_systems(FixedPreUpdate,update_bullet_board);
    }
}

#[derive(Resource)]
pub struct BulletBoard {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub border: f32,
}
impl BulletBoard {
    fn get_top(&mut self) -> Vec2 {
        Vec2::new(
            self.position.x,
            self.position.y + self.height / 2.0 + self.border / 2.0,
        )
    }
    fn get_bottom(&mut self) -> Vec2 {
        Vec2::new(
            self.position.x,
            self.position.y - self.height / 2.0 - self.border / 2.0,
        )
    }
    fn get_left(&mut self) -> Vec2 {
        Vec2::new(
            self.position.x - self.width / 2.0 - self.border / 2.0,
            self.position.y,
        )
    }
    fn get_right(&mut self) -> Vec2 {
        Vec2::new(
            self.position.x + self.width / 2.0 + self.border / 2.0,
            self.position.y,
        )
    }

    fn get_vert_size(&mut self) -> Vec2 {
        Vec2::new(self.width + self.border * 2.0, self.border)
    }
    fn get_hor_size(&mut self) -> Vec2 {
        Vec2::new(self.border, self.height + self.border * 2.0)
    }
    fn get_border_position(&mut self, border : &BulletBoardBorder) -> Vec2 {
        match border {
            BulletBoardBorder::Right => {
                self.get_right()
            },
            BulletBoardBorder::Left => {
                self.get_left()
            },
            BulletBoardBorder::Top => {
                self.get_top()
            },
            BulletBoardBorder::Bottom => {
                self.get_bottom()
            }
        }
    }
    fn get_border_scale(&mut self, border : &BulletBoardBorder) -> Vec2 {
        match border {
            BulletBoardBorder::Right => {
                self.get_hor_size()
            },
            BulletBoardBorder::Left => {
                self.get_hor_size()
            },
            BulletBoardBorder::Top => {
                self.get_vert_size()
            },
            BulletBoardBorder::Bottom => {
                self.get_vert_size()
            }
        }
    }
    fn spawn_border(
        &mut self,
        commands: &mut Commands,
        asset_manager: &Res<AssetManager>,
        render_layers: &Res<RenderLayerStorage>,
        border : BulletBoardBorder,
    ) {
        let mut scale = self.get_border_scale(&border);
        let mut position = self.get_border_position(&border);
        commands.spawn((
            Sprite {
                image: asset_manager.images["sprites/pixel.png"].clone(),
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                scale : Vec3::new(scale.x,scale.y,1.),
                ..Default::default()
            },
            border,
            render_layers.pre.clone(),
        ));
    }
}
#[derive(Component, Default,PartialEq)]
pub enum BulletBoardBorder {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}
fn spawn_bullet_board(mut commands: Commands,mut bullet_board : ResMut<BulletBoard>,asset_manager : Res<AssetManager>,render_layers : Res<RenderLayerStorage>) {
    bullet_board.spawn_border(&mut commands, &asset_manager, &render_layers, BulletBoardBorder::Right);
    bullet_board.spawn_border(&mut commands, &asset_manager, &render_layers, BulletBoardBorder::Left);
    bullet_board.spawn_border(&mut commands, &asset_manager, &render_layers, BulletBoardBorder::Top);
    bullet_board.spawn_border(&mut commands, &asset_manager, &render_layers, BulletBoardBorder::Bottom);
}
fn update_bullet_board(mut bullet_board : ResMut<BulletBoard>, mut border_query : Query<(&mut BulletBoardBorder, &mut Transform)>) {
    for (mut b, mut t) in border_query.iter_mut() {
        let pos = bullet_board.get_border_position(&b);
        t.translation.x = pos.x;
        t.translation.y = pos.y;
    }
}
fn constrain_player(
    mut bullet_board : ResMut<BulletBoard>,
    mut player_query : Query<(&mut Player,&mut PhysicsComponent)>,
    data : Res<Data>,
) { 
    for (mut player,mut physics) in player_query.iter_mut() {
        if physics.position.x + data.player.sprite_size_x / 2.0  > bullet_board.position.x + bullet_board.width / 2.0 {
            physics.position.x = bullet_board.position.x + bullet_board.width / 2.0 - data.player.sprite_size_x / 2.0;
        }

        if physics.position.x - data.player.sprite_size_x / 2.0  < bullet_board.position.x - bullet_board.width / 2.0 {
            physics.position.x = bullet_board.position.x - bullet_board.width / 2.0 + data.player.sprite_size_x / 2.0;
        }

        if physics.position.y - data.player.sprite_size_y / 2.0  < bullet_board.position.y - bullet_board.height / 2.0 {
            physics.position.y = bullet_board.position.y - bullet_board.height / 2.0 + data.player.sprite_size_y / 2.0;
        }

        if physics.position.y + data.player.sprite_size_y / 2.0  > bullet_board.position.y + bullet_board.height / 2.0 {
            physics.position.y = bullet_board.position.y + bullet_board.height / 2.0 - data.player.sprite_size_y / 2.0;
        }
    }
}
