use rand::{prelude::*, random_bool};
use rand_chacha::ChaCha8Rng;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

pub trait Replaceable {
    fn is_empty(self: Self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileContext<'a, T: Copy + PartialEq + Eq> {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    tile: &'a Option<T>,
    grid: &'a Vec<Vec<Vec<Option<T>>>>,
}

impl<'a, T> TileContext<'a, T>
where
    T: Copy + PartialEq + Eq,
{
    pub fn grid_width(&self) -> usize {
        self.grid.len()
    }

    pub fn grid_height(&self) -> usize {
        if self.grid_width() > 0 {
            self.grid[0].len()
        } else {
            0
        }
    }

    pub fn grid_depth(&self) -> usize {
        if self.grid_height() > 0 {
            self.grid[0][0].len()
        } else {
            0
        }
    }

    fn is_valid(&self, x: i32, y: i32, z: i32) -> bool {
        x >= 0
            && y >= 0
            && z >= 0
            && (x as usize) < self.grid_width()
            && (y as usize) < self.grid_height()
            && (z as usize) < self.grid_depth()
    }

    /// If this tile is actually on the grid or not
    pub fn is_in_bounds(&self) -> bool {
        self.is_valid(self.x, self.y, self.z)
    }

    /// Get context from another position, e.g. 1,-1 returns the bottom right corner
    pub fn get(&self, x_delta: i32, y_delta: i32, z_delta: i32) -> TileContext<'a, T> {
        let new_x = self.x + x_delta;
        let new_y = self.y + y_delta;
        let new_z = self.z + z_delta;

        TileContext {
            x: new_x,
            y: new_y,
            z: new_z,
            tile: if self.is_valid(new_x, new_y, new_z) {
                &self.grid[new_x as usize][new_y as usize][new_z as usize]
            } else {
                &None
            },
            grid: self.grid,
        }
    }

    pub fn up(&self) -> TileContext<'a, T> {
        self.get(0, -1, 0)
    }

    pub fn down(&self) -> TileContext<'a, T> {
        self.get(0, 1, 0)
    }

    pub fn left(&self) -> TileContext<'a, T> {
        self.get(-1, 0, 0)
    }

    pub fn right(&self) -> TileContext<'a, T> {
        self.get(1, 0, 0)
    }

    pub fn top_left(&self) -> TileContext<'a, T> {
        self.get(-1, -1, 0)
    }

    pub fn top_right(&self) -> TileContext<'a, T> {
        self.get(1, -1, 0)
    }

    pub fn bottom_left(&self) -> TileContext<'a, T> {
        self.get(-1, 1, 0)
    }

    pub fn bottom_right(&self) -> TileContext<'a, T> {
        self.get(1, 1, 0)
    }

    pub fn above(&self) -> TileContext<'a, T> {
        self.get(0, 0, 1)
    }

    pub fn below(&self) -> TileContext<'a, T> {
        self.get(0, 0, -1)
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
pub struct Replacement<T: Replaceable + Copy + PartialEq + Eq> {
    delta_x: i32,
    delta_y: i32,
    delta_z: i32,
    replacement: Option<T>,
}

impl<T> Replacement<T>
where
    T: Replaceable + Copy + PartialEq + Eq,
{
    pub fn new(delta_x: i32, delta_y: i32, delta_z: i32, replacement: Option<T>) -> Self {
        Self {
            delta_x,
            delta_y,
            delta_z,
            replacement,
        }
    }

    pub fn some(delta_x: i32, delta_y: i32, delta_z: i32, replacement: T) -> Self {
        Self::new(delta_x, delta_y, delta_z, Some(replacement))
    }

    pub fn this(replacement: T) -> Self {
        Self::new(0, 0, 0, Some(replacement))
    }

    pub fn left(replacement: T) -> Self {
        Self::new(-1, 0, 0, Some(replacement))
    }

    pub fn right(replacement: T) -> Self {
        Self::new(1, 0, 0, Some(replacement))
    }

    pub fn up(replacement: T) -> Self {
        Self::new(0, -1, 0, Some(replacement))
    }

    pub fn down(replacement: T) -> Self {
        Self::new(0, 1, 0, Some(replacement))
    }

    pub fn top_left(replacement: T) -> Self {
        Self::new(-1, -1, 0, Some(replacement))
    }

    pub fn top_right(replacement: T) -> Self {
        Self::new(-1, 1, 0, Some(replacement))
    }

    pub fn bottom_left(replacement: T) -> Self {
        Self::new(-1, 1, 0, Some(replacement))
    }

    pub fn bottom_right(replacement: T) -> Self {
        Self::new(1, 1, 0, Some(replacement))
    }

    pub fn above(replacement: T) -> Self {
        Self::new(0, 0, 1, Some(replacement))
    }

    pub fn below(replacement: T) -> Self {
        Self::new(0, 0, 1, Some(replacement))
    }
}

#[derive(Clone, Debug)]
pub struct ReplacementRule<T: Replaceable + Copy + PartialEq + Eq> {
    pub target: T,
    pub condition: fn(&TileContext<T>) -> bool,
    pub replacements: Vec<Replacement<T>>,
    pub chance: f64,
}

impl<T> ReplacementRule<T>
where
    T: Replaceable + Default + Copy + PartialEq + Eq,
{
    pub fn from_to(target: T, replacement: T, condition: fn(&TileContext<T>) -> bool) -> Self {
        ReplacementRule {
            target,
            replacements: vec![Replacement::some(0, 0, 0, replacement)],
            condition,
            ..Default::default()
        }
    }
}

impl<T> Default for ReplacementRule<T>
where
    T: Replaceable + Default + Copy + PartialEq + Eq,
{
    fn default() -> Self {
        Self {
            target: Default::default(),
            condition: |_| false,
            replacements: Default::default(),
            chance: 1.0,
        }
    }
}

/// Given a source grid, and a set of constraints, update the destination grid
/// Note that source and destination grid MUST have the same size for this to work
pub fn replace_tiles<T: PartialEq + Eq + Copy + Hash + Replaceable + Debug>(
    grid: &mut Vec<Vec<Vec<Option<T>>>>,
    layer: usize,
    rules: Vec<ReplacementRule<T>>,
    rng: &mut ChaCha8Rng,
) {
    let width = grid.len();
    for x in 0..width {
        let height = grid[x].len();
        for y in 0..height {
            for rule in rules.iter() {
                if grid[x][y][layer] == Some(rule.target)
                    || (grid[x][y][layer] == None && rule.target.is_empty())
                {
                    // TODO: pass rng in context?
                    let tile_ctx = TileContext {
                        x: x as i32,
                        y: y as i32,
                        z: layer as i32,
                        grid: &grid,
                        tile: &grid[x][y][layer],
                    };

                    // check conditions
                    if (rule.condition)(&tile_ctx) && rng.random_bool(rule.chance) {
                        // apply replacements
                        for replacement in rule.replacements.clone() {
                            let rx = x as i32 + replacement.delta_x;
                            let ry = y as i32 + replacement.delta_y;
                            let rz = layer as i32 + replacement.delta_z;

                            if rx >= 0
                                && ry >= 0
                                && rz >= 0
                                && (rx as usize) < grid.len()
                                && grid.len() > 0
                                && (ry as usize) < grid[0].len()
                                && grid[0].len() > 0
                                && (rz as usize) < grid.len()
                            {
                                grid[rx as usize][ry as usize][rz as usize] =
                                    replacement.replacement;
                            }
                        }

                        // skip remaining constraints since this one matched
                        break;
                    }
                }
            }
        }
    }
}

#[allow(unused)]
pub(crate) fn is_edge(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x == (width - 1) || y == (height - 1)
}
