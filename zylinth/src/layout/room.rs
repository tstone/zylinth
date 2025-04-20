use std::ops::Deref;
use std::usize;

use super::functional_tiles::UtilityTile;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use tilegen::{TileDir, TileRect};

pub struct Room {
    pub grid: Vec<Vec<Option<UtilityTile>>>,
    pub rect: TileRect,
    pub x: usize,
    pub y: usize,
}

impl Room {
    /// Generate a randomly sized rectangular room
    /// start_area = area where the room can randomly begin (top left)
    pub fn gen_rect(
        dir: &TileDir,
        start_area: &TileRect,
        min_width: usize,
        max_width: usize,
        min_height: usize,
        max_height: usize,
        rng: &mut ChaCha8Rng,
    ) -> Self {
        let width = (min_width..=max_width).choose(rng).unwrap();
        let height = (min_height..=max_height).choose(rng).unwrap();
        let rnd_x = (start_area.min.x as usize..=start_area.max.x as usize)
            .choose(rng)
            .unwrap();
        let rnd_y = (start_area.min.y as usize..=start_area.max.y as usize)
            .choose(rng)
            .unwrap();

        let x = match dir {
            TileDir::Left | TileDir::Up => rnd_x.saturating_sub(width),
            _ => rnd_x,
        };
        let y = match dir {
            TileDir::Up | TileDir::Left => rnd_y.saturating_sub(height),
            _ => rnd_y,
        };

        Self {
            grid: vec![vec![Some(UtilityTile::Floor); height]; width],
            rect: TileRect::new(x, y, x + width, y + height),
            x,
            y,
        }
    }

    /// Copy this room's tiles into a grid
    pub fn copy_grid_into(&self, dest: &mut Vec<Vec<Vec<Option<UtilityTile>>>>, layer: usize) {
        let top_left = self.rect.top_left();

        for x in 0..self.grid.len() {
            let dest_x = x + top_left.x;
            if dest_x < dest.len() {
                for y in 0..self.grid[x].len() {
                    let dest_y = top_left.y + y;
                    if dest_y < dest[x].len() {
                        dest[dest_x][dest_y][layer] = self.grid[x][y];
                    }
                }
            }
        }
    }
}

impl Deref for Room {
    type Target = TileRect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

impl AsRef<TileRect> for Room {
    fn as_ref(&self) -> &TileRect {
        &self.rect
    }
}

/// Calculate what percent the grid is filled
pub fn measure_density(grid: &Vec<Vec<Vec<Option<UtilityTile>>>>, layer: usize) -> f32 {
    let mut count: u32 = 0;
    let mut total: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y][layer].is_some() {
                count += 1;
            }
            total += 1;
        }
    }

    count as f32 / total as f32
}
