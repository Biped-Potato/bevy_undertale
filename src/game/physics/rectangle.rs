use bevy::prelude::*;
#[derive(Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl Rectangle {
    pub fn new(_x: f32, _y: f32, _w: f32, _h: f32) -> Self {
        return Rectangle {
            x: _x,
            y: _y,
            w: _w,
            h: _h,
        };
    }
    pub fn new_v(position: Vec2, half_size: Vec2) -> Self {
        return Rectangle {
            x: position.x - half_size.x,
            y: position.y + half_size.y,
            w: half_size.x * 2.,
            h: half_size.y * 2.,
        };
    }
    pub fn top(&self) -> f32 {
        return self.y;
    }
    pub fn bottom(&self) -> f32 {
        return self.y - self.h;
    }
    pub fn right(&self) -> f32 {
        return self.x + self.w;
    }
    pub fn left(&self) -> f32 {
        return self.x;
    }
    pub fn middle_x(&self) -> f32 {
        return self.x + self.w / 2.;
    }
    pub fn middle_y(&self) -> f32 {
        return self.y - self.h / 2.;
    }
    pub fn contains_point(&self, point: Vec2) -> bool {
        if point.x < self.right()
            && point.x > self.left()
            && point.y < self.top()
            && point.y > self.bottom()
        {
            return true;
        }
        return false;
    }
    pub fn contains_rect(&self, rectangle: Rectangle) -> bool {
        if self.contains_point(Vec2::new(rectangle.right(), rectangle.top()))
            && self.contains_point(Vec2::new(rectangle.left(), rectangle.bottom()))
        {
            return true;
        }
        return false;
    }
    pub fn to_vec2(&self) -> Vec2 {
        return Vec2::new(self.x, self.y);
    }
    pub fn get_intersection_depth(rect_a: Rectangle, rect_b: Rectangle) -> Vec2 {
        let half_width_a = rect_a.w / 2.0;
        let half_height_a = rect_a.h / 2.0;
        let half_width_b = rect_b.w / 2.0;
        let half_height_b = rect_b.h / 2.0;

        let center_a = Vec2::new(rect_a.left() + half_width_a, rect_a.top() - half_height_a);
        let center_b = Vec2::new(rect_b.left() + half_width_b, rect_b.top() - half_height_b);

        let distance_x = center_a.x - center_b.x;
        let distance_y = center_a.y - center_b.y;
        let min_distance_x = half_width_a + half_width_b;
        let min_distance_y = half_height_a + half_height_b;

        if (distance_x).abs() >= min_distance_x || (distance_y).abs() >= min_distance_y {
            return Vec2::ZERO;
        }

        let depth_x;
        let depth_y;
        if distance_x > 0. {
            depth_x = min_distance_x - distance_x;
        } else {
            depth_x = -min_distance_x - distance_x;
        }

        if distance_y > 0. {
            depth_y = min_distance_y - distance_y;
        } else {
            depth_y = -min_distance_y - distance_y;
        }
        return Vec2::new(depth_x, depth_y);
    }
    pub fn intersects(&self, other: Rectangle) -> bool {
        if self.right() < other.left()
            || self.left() > other.right()
            || self.bottom() > other.top()
            || self.top() < other.bottom()
        {
            return false;
        }
        return true;
    }
}
