use std::ops::{Deref, DerefMut};

use rand::Rng;

use super::replacement_rule::ReplacementRule;
use super::replacer::replace_tiles;

#[derive(Debug, Clone)]
pub struct TileGrid<T: Clone + PartialEq + Eq> {
    pub tiles: Vec<Vec<Vec<Option<T>>>>,
}

#[allow(unused)]
impl<T> TileGrid<T>
where
    T: Clone + PartialEq + Eq,
{
    pub fn empty(width: usize, height: usize, depth: usize) -> Self {
        let mut grid: Vec<Vec<Vec<Option<T>>>> = vec![vec![vec![None; depth]; height]; width];
        Self::new(grid)
    }

    pub fn new(grid: Vec<Vec<Vec<Option<T>>>>) -> Self {
        TileGrid { tiles: grid }
    }

    pub fn width(&self) -> usize {
        self.tiles.len()
    }

    pub fn height(&self) -> usize {
        if self.width() > 0 {
            self.tiles[0].len()
        } else {
            0
        }
    }

    pub fn depth(&self) -> usize {
        if self.height() > 0 {
            self.tiles[0][0].len()
        } else {
            0
        }
    }

    /// Wrap grid in padding
    pub fn pad(&self, top: u8, right: u8, bottom: u8, left: u8) -> TileGrid<T> {
        let width = self.width() + left as usize + right as usize;
        let height = self.height() + top as usize + bottom as usize;
        let depth = self.depth();
        let mut output = vec![vec![vec![None; depth]; height]; width];

        for x in 0..width {
            if x >= left as usize && x < (width - right as usize) {
                let input_x = x - left as usize;
                for y in 0..height {
                    for z in 0..depth {
                        if y >= top as usize && y < (height - bottom as usize) {
                            let input_y = y - top as usize;
                            output[x][y][z] = self.tiles[input_x][input_y][z].clone();
                        }
                    }
                }
            }
        }

        TileGrid::new(output)
    }

    /// Append a blank layer
    pub fn add_layer(&mut self) {
        let height = self.height();
        let depth = self.depth();
        for x in 0..self.width() {
            for y in 0..height {
                for _ in 0..depth {
                    self.tiles[x][y].push(None);
                }
            }
        }
    }

    /// Apply a set of replacement rules to a specific layer
    pub fn apply_layer_replacements(
        &mut self,
        target_layer: usize,
        rules: Vec<ReplacementRule<T>>,
        rng: &mut impl Rng,
    ) {
        replace_tiles(self, target_layer, rules, rng);
    }
}

impl<T> Deref for TileGrid<T>
where
    T: Clone + PartialEq + Eq,
{
    type Target = Vec<Vec<Vec<Option<T>>>>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl<T> DerefMut for TileGrid<T>
where
    T: Clone + PartialEq + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}
