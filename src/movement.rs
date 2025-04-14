use bevy::prelude::*;

use super::collision::*;
use crate::collision::Collidable;
use crate::layout::{Impassable, Tile};

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            speed: 50.0,
        }
    }
}

impl Velocity {
    pub fn starting_speed(speed: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            speed,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_movement);
    }
}

fn character_movement(
    mut query: Query<(&Velocity, &mut Transform, &GlobalTransform, &Collidable)>,
    impassable_tiles: Query<(&Tile, &GlobalTransform), With<Impassable>>,
) {
    'outer: for (vel, mut transform, global_transform, collidable) in query.iter_mut() {
        let translation = &mut transform.translation;
        let old_x = global_transform.translation().x;
        let old_y = global_transform.translation().y;

        let movement_x = vel.x * TIME_STEP * vel.speed;
        let movement_y = vel.y * TIME_STEP * vel.speed;

        let new_x = old_x + movement_x;
        let new_y = old_y + movement_y;

        let collidable_x = Rect::new(
            new_x,
            old_y,
            new_x + collidable.width,
            old_y + collidable.height,
        );
        let collidabley_y = Rect::new(
            old_x,
            new_y,
            old_x + collidable.width,
            new_y + collidable.height,
        );

        // check first that this move is allowed and skip it if not
        let mut intersect_x = false;
        let mut intersect_y = false;
        for (tile, transform) in impassable_tiles.iter() {
            let tile_rect = &tile.into_rect(transform);
            // let intersect_left =
            //     collidable_rect.min.x < tile_rect.max.x && collidable_rect.min.x > tile_rect.min.x;
            // let intersect_right =
            //     collidable_rect.max.x > tile_rect.min.x && tile_rect.min.x > collidable_rect.min.x;
            // let intersect_up =
            //     collidable_rect.max.y > tile_rect.min.y && collidable_rect.max.y < tile_rect.max.y;
            // let intersect_down =
            //     collidable_rect.min.y < tile_rect.max.y && collidable_rect.min.y > tile_rect.min.y;

            // intersect_x = (intersect_left && (intersect_up || intersect_down))
            //     || (intersect_right && (intersect_up || intersect_down));
            // intersect_y = (intersect_up && (intersect_left || intersect_right))
            //     || (intersect_down && (intersect_left || intersect_right));

            // intersect_x = tile_rect.contains(Vec2::new(new_x, old_y));
            // intersect_y = tile_rect.contains(Vec2::new(old_x, new_y));

            debug!(
                "width: {}, height: {}",
                tile_rect.intersect(collidable_x).width(),
                tile_rect.intersect(collidabley_y).height()
            );
            intersect_x = tile_rect.intersect(collidable_x).width() > 0.0;
            intersect_y = tile_rect.intersect(collidabley_y).height() > 0.0;

            if (intersect_x || intersect_y) {
                debug!("intersected tile {:?}", tile);
                break;
            }
        }

        if !intersect_x {
            translation.x += movement_x;
        }
        if !intersect_y {
            translation.y += movement_y;
        }
    }
}
