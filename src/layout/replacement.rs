use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

pub trait Replaceable {
    fn is_empty(self: Self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileContext<'a, T: Copy + PartialEq + Eq> {
    #[allow(unused)]
    pub x: i32,
    #[allow(unused)]
    pub y: i32,

    tile: &'a Option<T>,
    grid: &'a Vec<Vec<Option<T>>>,
}

impl<'a, T> TileContext<'a, T>
where
    T: Copy + PartialEq + Eq,
{
    pub fn grid_width(&self) -> usize {
        self.grid.len()
    }

    pub fn grid_height(&self) -> usize {
        if self.grid.len() > 0 {
            self.grid[0].len()
        } else {
            0
        }
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.grid_width() && (y as usize) < self.grid_height()
    }

    /// If this tile is actually on the grid or not
    pub fn is_in_bounds(&self) -> bool {
        self.is_valid(self.x, self.y)
    }

    /// Get context from another position, e.g. 1,-1 returns the bottom right corner
    pub fn get(&self, x_delta: i32, y_delta: i32) -> TileContext<'a, T> {
        let new_x = self.x + x_delta;
        let new_y = self.y + y_delta;

        TileContext {
            x: new_x,
            y: new_y,
            tile: if self.is_valid(new_x, new_y) {
                &self.grid[new_x as usize][new_y as usize]
            } else {
                &None
            },
            grid: self.grid,
        }
    }

    pub fn above(&self) -> TileContext<'a, T> {
        self.get(0, -1)
    }

    pub fn below(&self) -> TileContext<'a, T> {
        self.get(0, 1)
    }

    pub fn left(&self) -> TileContext<'a, T> {
        self.get(-1, 0)
    }

    pub fn right(&self) -> TileContext<'a, T> {
        self.get(1, 0)
    }

    pub fn top_left(&self) -> TileContext<'a, T> {
        self.get(-1, -1)
    }

    pub fn top_right(&self) -> TileContext<'a, T> {
        self.get(1, -1)
    }

    pub fn bottom_left(&self) -> TileContext<'a, T> {
        self.get(-1, 1)
    }

    pub fn bottom_right(&self) -> TileContext<'a, T> {
        self.get(1, 1)
    }
}

impl<'a, T> PartialEq<T> for TileContext<'a, T>
where
    T: Copy + PartialEq + Eq,
{
    fn eq(&self, other: &T) -> bool {
        *self.tile == Some(*other)
    }
}

impl<'a, T> PartialEq<Option<T>> for TileContext<'a, T>
where
    T: Copy + PartialEq + Eq,
{
    fn eq(&self, other: &Option<T>) -> bool {
        self.tile == other
    }
}

impl<'a, T> Deref for TileContext<'a, T>
where
    T: Copy + PartialEq + Eq,
{
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        self.tile
    }
}

#[derive(Clone, Debug)]
pub struct Replacement<S: Replaceable + Copy + PartialEq + Eq, D: Replaceable> {
    pub target: S,
    pub condition: fn(&TileContext<S>) -> bool,

    // replacements
    pub replacement: D,
    pub replacement_above: Option<D>,
    pub replacement_below: Option<D>,
    pub replacement_left: Option<D>,
    pub replacement_right: Option<D>,
    pub replacement_top_left: Option<D>,
    pub replacement_top_right: Option<D>,
    pub replacement_bottom_left: Option<D>,
    pub replacement_bottom_right: Option<D>,

    pub chance: f64,
}

impl<S, D> Replacement<S, D>
where
    S: Replaceable + Default + Copy + PartialEq + Eq,
    D: Replaceable + Default,
{
    pub fn from_to(target: S, replacement: D, condition: fn(&TileContext<S>) -> bool) -> Self {
        Replacement {
            target,
            replacement,
            condition,
            ..Default::default()
        }
    }
}

impl<S, D> Default for Replacement<S, D>
where
    S: Replaceable + Default + Copy + PartialEq + Eq,
    D: Replaceable + Default,
{
    fn default() -> Self {
        Self {
            target: Default::default(),
            condition: |_| false,
            replacement: Default::default(),
            replacement_above: Default::default(),
            replacement_below: Default::default(),
            replacement_left: Default::default(),
            replacement_right: Default::default(),
            replacement_top_left: Default::default(),
            replacement_top_right: Default::default(),
            replacement_bottom_left: Default::default(),
            replacement_bottom_right: Default::default(),
            chance: 1.0,
        }
    }
}

/// Given a source grid, and a set of constraints, update the destination grid
/// Note that source and destination grid MUST have the same size for this to work
pub fn replace_tiles<
    S: PartialEq + Eq + Copy + Hash + Replaceable + Debug,
    D: PartialEq + Eq + Copy + Hash + Replaceable + Debug,
>(
    source: &Vec<Vec<Option<S>>>,
    replacements: Vec<Replacement<S, D>>,
    mut dest: Vec<Vec<Option<D>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<D>>> {
    let width = dest.len();
    for x in 0..width {
        let height = dest[x].len();
        for y in 0..height {
            for replacement in replacements.iter() {
                if source[x][y] == Some(replacement.target)
                    || (source[x][y] == None && replacement.target.is_empty())
                {
                    let tile_ctx = TileContext {
                        x: x as i32,
                        y: y as i32,
                        grid: &source,
                        tile: &source[x][y],
                    };

                    if (replacement.condition)(&tile_ctx) {
                        if replacement.chance == 1.0 || rng.random_bool(replacement.chance) {
                            // apply replacements
                            dest[x][y] = Some(replacement.replacement);

                            // replace above
                            match (replacement.replacement_above, tile_ctx.above().tile) {
                                (Some(above), Some(_)) => dest[x][y - 1] = Some(above),
                                _ => {}
                            }
                            // replace below
                            match (replacement.replacement_below, tile_ctx.below().tile) {
                                (Some(below), Some(_)) => dest[x][y + 1] = Some(below),
                                _ => {}
                            }
                            // replace left
                            match (replacement.replacement_left, tile_ctx.left().tile) {
                                (Some(left), Some(_)) => dest[x - 1][y] = Some(left),
                                _ => {}
                            }
                            // replace right
                            match (replacement.replacement_right, tile_ctx.right().tile) {
                                (Some(right), Some(_)) => dest[x + 1][y] = Some(right),
                                _ => {}
                            }
                            // replace top-left
                            match (replacement.replacement_top_left, tile_ctx.top_left().tile) {
                                (Some(tl), Some(_)) => dest[x - 1][y - 1] = Some(tl),
                                _ => {}
                            }
                            // replace top-right
                            match (replacement.replacement_top_right, tile_ctx.top_right().tile) {
                                (Some(tr), Some(_)) => dest[x + 1][y - 1] = Some(tr),
                                _ => {}
                            }
                            // replace bottom-left
                            match (
                                replacement.replacement_bottom_left,
                                tile_ctx.bottom_left().tile,
                            ) {
                                (Some(bl), Some(_)) => dest[x - 1][y + 1] = Some(bl),
                                _ => {}
                            }
                            // replace bottom-right
                            match (
                                replacement.replacement_bottom_right,
                                tile_ctx.bottom_right().tile,
                            ) {
                                (Some(br), Some(_)) => dest[x + 1][y + 1] = Some(br),
                                _ => {}
                            }

                            // skip remaining constraints since this one matched
                            break;
                        }
                    }
                }
            }
        }
    }
    dest
}

#[allow(unused)]
pub(crate) fn is_edge(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x == (width - 1) || y == (height - 1)
}
