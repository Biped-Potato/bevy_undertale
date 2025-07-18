use bevy::prelude::*;

use crate::game::{
    camera::render_layers::RenderLayerStorage,
    data::data::Data,
    loading::loading::AssetManager,
    physics::physics_object::PhysicsComponent,
    player::player::{Player, player_movement},
    scene::menu::MenuState,
    state::state::AppState,
};

pub struct BulletBoardPlugin;
impl Plugin for BulletBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletBoard {
            width: 155.,
            height: 130.,

            target_width: 155.,
            target_height: 130.,

            position: Vec2::ZERO,
            target_position: Vec2::ZERO,
            border: 5.0,

            expansion_rate: 4.0,
            movement_rate: 4.0,
        })
        .add_systems(OnEnter(AppState::Level), spawn_bullet_board)
        .add_systems(FixedPreUpdate, update_bullet_board);
    }
}

#[derive(Resource)]
pub struct BulletBoard {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,

    pub target_width: f32,
    pub target_height: f32,
    pub target_position: Vec2,

    pub border: f32,

    //how fast the dimensions of the box expands each frame
    pub expansion_rate: f32,
    //how fast the position of the box moves each frame
    pub movement_rate: f32,
}
impl BulletBoard {
    pub fn set_absolute(&mut self, width: f32, height: f32, position: Vec2) {
        self.width = width;
        self.height = height;
        self.position = position;

        self.target_width = width;
        self.target_height = height;
        self.target_position = position;
    }
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
    fn get_border_position(&mut self, border: &BulletBoardBorder) -> Vec2 {
        match border {
            BulletBoardBorder::Right => self.get_right(),
            BulletBoardBorder::Left => self.get_left(),
            BulletBoardBorder::Top => self.get_top(),
            BulletBoardBorder::Bottom => self.get_bottom(),
        }
    }
    fn get_border_scale(&mut self, border: &BulletBoardBorder) -> Vec2 {
        match border {
            BulletBoardBorder::Right => self.get_hor_size(),
            BulletBoardBorder::Left => self.get_hor_size(),
            BulletBoardBorder::Top => self.get_vert_size(),
            BulletBoardBorder::Bottom => self.get_vert_size(),
        }
    }
    fn spawn_border(
        &mut self,
        commands: &mut Commands,
        asset_manager: &Res<AssetManager>,
        render_layers: &Res<RenderLayerStorage>,
        border: BulletBoardBorder,
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
                scale: Vec3::new(scale.x, scale.y, 1.),
                ..Default::default()
            },
            border,
            render_layers.pre.clone(),
        ));
    }
}
#[derive(Component, Default, PartialEq)]
pub enum BulletBoardBorder {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}
fn spawn_bullet_board(
    mut commands: Commands,
    mut bullet_board: ResMut<BulletBoard>,
    asset_manager: Res<AssetManager>,
    render_layers: Res<RenderLayerStorage>,
) {
    bullet_board.spawn_border(
        &mut commands,
        &asset_manager,
        &render_layers,
        BulletBoardBorder::Right,
    );
    bullet_board.spawn_border(
        &mut commands,
        &asset_manager,
        &render_layers,
        BulletBoardBorder::Left,
    );
    bullet_board.spawn_border(
        &mut commands,
        &asset_manager,
        &render_layers,
        BulletBoardBorder::Top,
    );
    bullet_board.spawn_border(
        &mut commands,
        &asset_manager,
        &render_layers,
        BulletBoardBorder::Bottom,
    );
}
pub fn move_towards_vec(start: Vec2, end: Vec2, rate: f32) -> Vec2 {
    let direction = Vec2::normalize_or_zero(end - start);
    let magnitude = f32::clamp(rate, 0.0, Vec2::length(end - start));
    return direction * magnitude;
}

pub fn move_towards(start: f32, end: f32, rate: f32) -> f32 {
    return f32::signum(end - start) * f32::clamp(rate, 0.0, f32::abs(end - start));
}

fn update_bullet_board(
    mut bullet_board: ResMut<BulletBoard>,
    mut border_query: Query<(&mut BulletBoardBorder, &mut Transform)>,
) {
    bullet_board.width += move_towards(
        bullet_board.width,
        bullet_board.target_width,
        bullet_board.expansion_rate,
    );
    bullet_board.height += move_towards(
        bullet_board.height,
        bullet_board.target_height,
        bullet_board.expansion_rate,
    );

    let position = bullet_board.position;
    let target_position = bullet_board.target_position;
    let movement_rate = bullet_board.movement_rate;
    bullet_board.position += move_towards_vec(position, target_position, movement_rate);

    for (mut b, mut t) in border_query.iter_mut() {
        let pos = bullet_board.get_border_position(&b);
        t.translation.x = pos.x;
        t.translation.y = pos.y;
        let scale = bullet_board.get_border_scale(&b);
        t.scale.x = scale.x;
        t.scale.y = scale.y;
    }
}