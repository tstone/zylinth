use bevy::log::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Debug;
use std::{collections::HashSet, hash::Hash};

pub trait Replaceable {
    fn is_empty(self: Self) -> bool;
}

#[derive(Clone, Debug)]
pub struct Replacement<S: Replaceable, D: Replaceable> {
    pub desc: &'static str,
    pub target: S,

    // conditions
    pub above: HashSet<S>,
    pub below: HashSet<S>,
    pub on_left: HashSet<S>,
    pub on_right: HashSet<S>,
    pub on_top_left: HashSet<S>,
    pub on_top_right: HashSet<S>,
    pub on_bottom_left: HashSet<S>,
    pub on_bottom_right: HashSet<S>,

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

impl<S, D> Default for Replacement<S, D>
where
    S: Replaceable + Default,
    D: Replaceable + Default,
{
    fn default() -> Self {
        Self {
            desc: Default::default(),
            target: Default::default(),
            above: Default::default(),
            below: Default::default(),
            on_left: Default::default(),
            on_right: Default::default(),
            on_top_left: Default::default(),
            on_top_right: Default::default(),
            on_bottom_left: Default::default(),
            on_bottom_right: Default::default(),
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
                    // above conditions
                    let above_tile = get_tile_above(x, y, source);
                    let above = match above_tile {
                        _ if replacement.above.len() == 0 => true,
                        None if set_contains_empty(&replacement.above) => true,
                        None if replacement.above.len() > 0 => false,
                        Some(t) => replacement.above.contains(&t),
                        _ => true,
                    };
                    // below conditions
                    let below_tile = get_tile_below(x, y, source);
                    let below = match below_tile {
                        _ if replacement.below.len() == 0 => true,
                        None if set_contains_empty(&replacement.below) => true,
                        None if replacement.below.len() > 0 => false,
                        Some(t) => replacement.below.contains(&t),
                        _ => true,
                    };
                    // on left conditions
                    let left_tile = get_tile_left_of(x, y, source);
                    let left_of = match left_tile {
                        _ if replacement.on_left.len() == 0 => true,
                        None if set_contains_empty(&replacement.on_left) => true,
                        None if replacement.on_left.len() > 0 => false,
                        Some(t) => replacement.on_left.contains(&t),
                        _ => true,
                    };
                    // on right conditions
                    let right_tile = get_tile_right_of(x, y, source);
                    let right_of = match right_tile {
                        _ if replacement.on_right.len() == 0 => true,
                        None if set_contains_empty(&replacement.on_right) => true,
                        None if replacement.on_right.len() > 0 => false,
                        Some(t) => replacement.on_right.contains(&t),
                        _ => true,
                    };
                    // top left conditions
                    let top_left_tile = get_top_left_tile(x, y, source);
                    let top_left_of = match top_left_tile {
                        _ if replacement.on_top_left.len() == 0 => true,
                        None if set_contains_empty(&replacement.on_top_left) => true,
                        None if replacement.on_top_left.len() > 0 => false,
                        Some(t) => replacement.on_top_left.contains(&t),
                        _ => true,
                    };
                    // top right conditions
                    let top_right_tile = get_top_right_tile(x, y, source);
                    let top_right_of = match top_right_tile {
                        _ if replacement.on_top_right.len() == 0 => true,
                        None if set_contains_empty(&replacement.on_top_right) => true,
                        None if replacement.on_top_right.len() > 0 => false,
                        Some(t) => replacement.on_top_right.contains(&t),
                        _ => true,
                    };
                    // bottom left conditions
                    let bottom_left_tile = get_bottom_left_tile(x, y, source);
                    let bottom_left_of = match bottom_left_tile {
                        _ if replacement.on_bottom_left.len() == 0 => true,
                        None if set_contains_empty(&replacement.on_bottom_left) => true,
                        None if replacement.on_bottom_left.len() > 0 => false,
                        Some(t) => replacement.on_bottom_left.contains(&t),
                        _ => true,
                    };
                    // bottom right conditions
                    let bottom_right_tile = get_bottom_right_tile(x, y, source);
                    let bottom_right_of = match bottom_right_tile {
                        _ if replacement.on_bottom_right.len() == 0 => true,
                        None if set_contains_empty(&replacement.on_bottom_right) => true,
                        None if replacement.on_bottom_right.len() > 0 => false,
                        Some(t) => replacement.on_bottom_right.contains(&t),
                        _ => true,
                    };

                    trace!(
                        "({x},{y}) {} -- above: {:?} {above}, below: {:?} {below}, left: {:?} {left_of}, right: {:?} {right_of}",
                        replacement.desc, above_tile, below_tile, left_tile, right_tile,
                    );
                    trace!(
                        "({x},{y}) {} -- top-left: {:?} {top_left_of}, top-right: {:?} {top_right_of}, bottom-left: {:?} {bottom_left_of}, bottom-right: {:?} {bottom_right_of}",
                        replacement.desc,
                        top_left_tile,
                        top_right_tile,
                        bottom_left_tile,
                        bottom_right_tile,
                    );

                    if above
                        && below
                        && left_of
                        && right_of
                        && top_left_of
                        && top_right_of
                        && bottom_left_of
                        && bottom_right_of
                    {
                        if replacement.chance == 1.0 || rng.random_bool(replacement.chance) {
                            // apply replacements
                            dest[x][y] = Some(replacement.replacement);

                            // replace above
                            match (replacement.replacement_above, above_tile) {
                                (Some(above), Some(_)) => dest[x][y - 1] = Some(above),
                                _ => {}
                            }
                            // replace below
                            match (replacement.replacement_below, below_tile) {
                                (Some(below), Some(_)) => dest[x][y + 1] = Some(below),
                                _ => {}
                            }
                            // replace left
                            match (replacement.replacement_on_left, left_tile) {
                                (Some(left), Some(_)) => dest[x - 1][y] = Some(left),
                                _ => {}
                            }
                            // replace right
                            match (replacement.replacement_on_right, right_tile) {
                                (Some(right), Some(_)) => dest[x + 1][y] = Some(right),
                                _ => {}
                            }
                            // replace top-left
                            match (replacement.replacement_top_left, top_left_tile) {
                                (Some(tl), Some(_)) => dest[x - 1][y - 1] = Some(tl),
                                _ => {}
                            }
                            // replace top-right
                            match (replacement.replacement_top_right, top_right_tile) {
                                (Some(tr), Some(_)) => dest[x + 1][y - 1] = Some(tr),
                                _ => {}
                            }
                            // replace bottom-left
                            match (replacement.replacement_bottom_left, bottom_left_tile) {
                                (Some(bl), Some(_)) => dest[x - 1][y + 1] = Some(bl),
                                _ => {}
                            }
                            // replace bottom-right
                            match (replacement.replacement_bottom_right, bottom_right_tile) {
                                (Some(br), Some(_)) => dest[x + 1][y + 1] = Some(br),
                                _ => {}
                            }
                        }

                        // skip remaining constraints since this one matched
                        continue;
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
