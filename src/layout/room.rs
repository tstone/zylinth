use std::collections::HashMap;
use std::ops::Deref;
use std::usize;

use super::functional_tiles::UtilityTile;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone)]
pub struct TileRect {
    pub min: TilePoint,
    pub max: TilePoint,
}

#[allow(unused)]
impl TileRect {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> TileRect {
        TileRect {
            min: TilePoint::new(x0, y0),
            max: TilePoint::new(x1, y1),
        }
    }

    pub fn width(&self) -> usize {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> usize {
        self.max.y - self.min.y
    }

    pub fn top_left(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.min.y)
    }

    pub fn top_right(&self) -> TilePoint {
        TilePoint::new(self.max.x, self.min.y)
    }

    pub fn bottom_left(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.max.y)
    }

    pub fn bottom_right(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.max.y)
    }

    pub fn center(&self) -> TilePoint {
        TilePoint::new(
            self.min.x + (self.width() / 2),
            self.min.y + (self.height() / 2),
        )
    }

    pub fn left_center(&self) -> TilePoint {
        TilePoint::new(self.min.x, self.min.y + (self.height() / 2))
    }

    pub fn right_center(&self) -> TilePoint {
        TilePoint::new(self.max.x, self.min.y + (self.height() / 2))
    }

    pub fn top_center(&self) -> TilePoint {
        TilePoint::new(self.min.x + (self.width() / 2), self.min.y)
    }

    pub fn bottom_center(&self) -> TilePoint {
        TilePoint::new(self.min.x + (self.width() / 2), self.max.y)
    }

    pub fn get_left_slice(&self, width: usize) -> TileRect {
        TileRect::new(self.min.x, self.min.y, self.min.x + width, self.max.y)
    }

    pub fn get_right_slice(&self, width: usize) -> TileRect {
        TileRect::new(self.max.x - width, self.min.y, self.max.x, self.max.y)
    }

    pub fn get_top_slice(&self, height: usize) -> TileRect {
        TileRect::new(self.min.x, self.min.y, self.max.x, self.min.y + height)
    }

    pub fn get_bottom_slice(&self, height: usize) -> TileRect {
        TileRect::new(self.min.x, self.max.y - height, self.max.x, self.max.y)
    }

    /// Create a new tile rect that is the same shape but subtract the given padding amount
    pub fn get_inner_slice(&self, horz_padding: usize, vert_padding: usize) -> Option<TileRect> {
        if self.width() < (horz_padding * 2) || self.height() < (vert_padding * 2) {
            return None;
        }

        Some(TileRect::new(
            self.min.x + horz_padding,
            self.min.y + vert_padding,
            self.max.x - horz_padding,
            self.max.y - vert_padding,
        ))
    }

    pub fn intersect(&self, other: &TileRect) -> Option<Self> {
        if self.min.x > other.max.x
            || self.max.x < other.min.x
            || self.min.y > other.max.y
            || self.max.y < other.min.y
        {
            None
        } else {
            let x0 = if self.min.x < other.min.x {
                other.min.x
            } else {
                self.min.x
            };
            let y0 = if self.min.y < other.min.y {
                other.min.y
            } else {
                self.min.y
            };
            let x1 = if other.max.x < self.max.x {
                other.max.x
            } else {
                self.max.x
            };
            let y1 = if other.max.y < self.max.y {
                other.max.y
            } else {
                self.max.y
            };
            Some(TileRect::new(x0, y0, x1, y1))
        }
    }

    /// Return a random point inside of this rectangle
    pub fn rnd_point(&self, rng: &mut ChaCha8Rng) -> TilePoint {
        let x = (self.min.x..self.max.x).choose(rng).unwrap();
        let y = (self.min.y..self.max.y).choose(rng).unwrap();
        TilePoint::new(x, y)
    }

    /// Return a random sub region inside of this rectangle
    pub fn rnd_bounded_slice(
        &self,
        width: usize,
        height: usize,
        rng: &mut ChaCha8Rng,
    ) -> Option<TileRect> {
        self.get_inner_slice(width, height).map(|inner| {
            let top_left = inner.rnd_point(rng);
            TileRect::new(
                top_left.x,
                top_left.y,
                top_left.x + width,
                top_left.y + height,
            )
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TilePoint {
    pub x: usize,
    pub y: usize,
}

impl TilePoint {
    pub fn new(x: usize, y: usize) -> TilePoint {
        TilePoint { x, y }
    }

    pub fn distance(&self, other: &TilePoint) -> f32 {
        let a = (other.x as i32 - self.x as i32).pow(2).abs();
        let b = (other.y as i32 - self.y as i32).pow(2).abs();
        (a as f32 + b as f32).sqrt()
    }
}

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

    pub fn copy_grid_into(&self, dest: &mut Vec<Vec<Option<UtilityTile>>>) {
        copy_into_grid(&self.grid, self.rect.top_left(), dest);
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileDir {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

#[allow(unused)]
impl TileDir {
    /// Get all directions as a list
    pub fn vec() -> Vec<TileDir> {
        vec![TileDir::Left, TileDir::Right, TileDir::Up, TileDir::Down]
    }

    /// Get all directions as a list, except the one given
    pub fn vec_without(dir: &TileDir) -> Vec<TileDir> {
        Self::vec()
            .iter()
            .filter(|d| *d != dir)
            .map(|v| *v)
            .collect::<Vec<_>>()
    }

    /// Randomly pick a direction
    pub fn rnd(rng: &mut ChaCha8Rng) -> TileDir {
        *Self::vec().choose(rng).unwrap()
    }

    /// Randomly pick a direction that is not the given value
    pub fn rnd_without(dir: &TileDir, rng: &mut ChaCha8Rng) -> TileDir {
        *Self::vec_without(dir).choose(rng).unwrap()
    }

    /// Randomly pick a direction using weights. Only weighted directions are used.
    pub fn rnd_weighted(weights: &HashMap<TileDir, f32>, rng: &mut ChaCha8Rng) -> TileDir {
        let rnd = rng.random::<f32>();
        let mut running_weight = 0.0;
        for (dir, weight) in weights {
            running_weight += weight;
            if rnd < running_weight {
                return *dir;
            }
        }
        return TileDir::default();
    }
}

// TODO: just move this into Room
/// A method to copy a smaller grid into a larger grid
fn copy_into_grid(
    source: &Vec<Vec<Option<UtilityTile>>>,
    top_left: TilePoint,
    dest: &mut Vec<Vec<Option<UtilityTile>>>,
) {
    for x in 0..source.len() {
        let dest_x = x + top_left.x;
        if dest_x < dest.len() {
            for y in 0..source[x].len() {
                let dest_y = top_left.y + y;
                if dest_y < dest[x].len() {
                    dest[dest_x][dest_y] = source[x][y];
                }
            }
        }
    }
}

/// Calculate what percent the grid is filled
pub fn measure_density(grid: &Vec<Vec<Option<UtilityTile>>>) -> f32 {
    let mut count: u32 = 0;
    let mut total: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y].is_some() {
                count += 1;
            }
            total += 1;
        }
    }

    count as f32 / total as f32
}
