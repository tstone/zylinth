use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Debug;
use std::{collections::HashSet, hash::Hash};

pub trait Replaceable {
    fn is_empty(self: Self) -> bool;
}

#[derive(Debug)]
pub struct TileContext<T> {
    pub x: usize,
    pub y: usize,
    pub above: Option<T>,
    pub below: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
    pub top_left: Option<T>,
    pub top_right: Option<T>,
    pub bottom_left: Option<T>,
    pub bottom_right: Option<T>,
}

#[derive(Clone, Debug)]
pub struct Replacement<S: Replaceable, D: Replaceable> {
    pub target: S,
    pub condition: fn(&TileContext<S>) -> bool,

    // replacements
    pub replacement: D,
    pub replacement_above: Option<D>,
    pub replacement_below: Option<D>,
    pub replacement_on_left: Option<D>,
    pub replacement_on_right: Option<D>,
    pub replacement_top_left: Option<D>,
    pub replacement_top_right: Option<D>,
    pub replacement_bottom_left: Option<D>,
    pub replacement_bottom_right: Option<D>,

    pub chance: f64,
}

impl<S, D> Replacement<S, D>
where
    S: Replaceable + Default,
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
    S: Replaceable + Default,
    D: Replaceable + Default,
{
    fn default() -> Self {
        Self {
            target: Default::default(),
            condition: |_| false,
            replacement: Default::default(),
            replacement_above: Default::default(),
            replacement_below: Default::default(),
            replacement_on_left: Default::default(),
            replacement_on_right: Default::default(),
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
                        x,
                        y,
                        above: get_tile_above(x, y, source),
                        below: get_tile_below(x, y, source),
                        left: get_tile_left_of(x, y, source),
                        right: get_tile_right_of(x, y, source),
                        top_left: get_top_left_tile(x, y, source),
                        top_right: get_top_right_tile(x, y, source),
                        bottom_left: get_bottom_left_tile(x, y, source),
                        bottom_right: get_bottom_right_tile(x, y, source),
                    };

                    if (replacement.condition)(&tile_ctx) {
                        if replacement.chance == 1.0 || rng.random_bool(replacement.chance) {
                            // apply replacements
                            dest[x][y] = Some(replacement.replacement);

                            // replace above
                            match (replacement.replacement_above, tile_ctx.above) {
                                (Some(above), Some(_)) => dest[x][y - 1] = Some(above),
                                _ => {}
                            }
                            // replace below
                            match (replacement.replacement_below, tile_ctx.below) {
                                (Some(below), Some(_)) => dest[x][y + 1] = Some(below),
                                _ => {}
                            }
                            // replace left
                            match (replacement.replacement_on_left, tile_ctx.left) {
                                (Some(left), Some(_)) => dest[x - 1][y] = Some(left),
                                _ => {}
                            }
                            // replace right
                            match (replacement.replacement_on_right, tile_ctx.right) {
                                (Some(right), Some(_)) => dest[x + 1][y] = Some(right),
                                _ => {}
                            }
                            // replace top-left
                            match (replacement.replacement_top_left, tile_ctx.top_left) {
                                (Some(tl), Some(_)) => dest[x - 1][y - 1] = Some(tl),
                                _ => {}
                            }
                            // replace top-right
                            match (replacement.replacement_top_right, tile_ctx.top_right) {
                                (Some(tr), Some(_)) => dest[x + 1][y - 1] = Some(tr),
                                _ => {}
                            }
                            // replace bottom-left
                            match (replacement.replacement_bottom_left, tile_ctx.bottom_left) {
                                (Some(bl), Some(_)) => dest[x - 1][y + 1] = Some(bl),
                                _ => {}
                            }
                            // replace bottom-right
                            match (replacement.replacement_bottom_right, tile_ctx.bottom_right) {
                                (Some(br), Some(_)) => dest[x + 1][y + 1] = Some(br),
                                _ => {}
                            }
                        }

                        // skip remaining constraints since this one matched
                        break;
                    }
                }
            }
        }
    }
    dest
}

fn set_contains_empty<T: PartialEq + Eq + Copy + Replaceable>(set: &HashSet<T>) -> bool {
    set.iter().find(|i| i.is_empty()).is_some()
}

pub(crate) fn is_edge(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x == (width - 1) || y == (height - 1)
}

pub(crate) fn get_tile_above<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if y > 0 { grid[x][y - 1] } else { None }
}

pub(crate) fn get_tile_below<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if y < (grid[x].len() - 1) {
        grid[x][y + 1]
    } else {
        None
    }
}

pub(crate) fn get_tile_left_of<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if x > 0 { grid[x - 1][y] } else { None }
}

pub(crate) fn get_tile_right_of<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if x < (grid.len() - 1) {
        grid[x + 1][y]
    } else {
        None
    }
}

pub(crate) fn get_top_left_tile<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if x > 0 && y > 0 {
        grid[x - 1][y - 1]
    } else {
        None
    }
}

pub(crate) fn get_top_right_tile<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if x < (grid.len() - 1) && y > 0 {
        grid[x + 1][y - 1]
    } else {
        None
    }
}

pub(crate) fn get_bottom_left_tile<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if x > 0 && y < (grid[x].len() - 1) {
        grid[x - 1][y + 1]
    } else {
        None
    }
}

pub(crate) fn get_bottom_right_tile<T: PartialEq + Eq + Copy + Replaceable>(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<T>>>,
) -> Option<T> {
    if x < (grid.len() - 1) && y < (grid[x].len() - 1) {
        grid[x + 1][y + 1]
    } else {
        None
    }
}
