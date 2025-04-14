use bevy::prelude::*;

#[derive(Component)]
pub struct Collidable {
    pub width: f32,
    pub height: f32,
    // TODO: maybe make the shape adjustable or have a Mesh2d for the collision shape
}

pub trait Intersects {
    fn overlap_x(&self, other: &Self) -> f32;
    fn overlap_y(&self, other: &Self) -> f32;
}

impl Intersects for Rect {
    fn overlap_x(&self, other: &Self) -> f32 {
        if other.min.x < self.max.x && self.min.x < other.max.x {
            self.max.x - other.min.x
        } else if self.min.x < other.max.x && other.min.x < self.max.x {
            other.max.x - self.min.x
        } else {
            0.0
        }
    }

    fn overlap_y(&self, other: &Self) -> f32 {
        if other.min.y < self.max.y && self.min.y < other.max.y {
            self.max.y - other.min.y
        } else if self.min.y < other.max.y && other.min.y < self.max.y {
            other.max.y - self.min.y
        } else {
            0.0
        }
    }
}
