use std::collections::HashSet;

use super::functional_tiles::UtilityTile;

#[derive(Clone, Debug, Default)]
pub struct Replacement {
    pub target: UtilityTile,
    pub replacement: UtilityTile,
    pub above: HashSet<UtilityTile>,
    pub below: HashSet<UtilityTile>,
    pub on_left: HashSet<UtilityTile>,
    pub on_right: HashSet<UtilityTile>,
    pub on_top_left: HashSet<UtilityTile>,
    pub on_top_right: HashSet<UtilityTile>,
    pub on_bottom_left: HashSet<UtilityTile>,
    pub on_bottom_right: HashSet<UtilityTile>,
}

pub fn replace_tiles(
    mut grid: Vec<Vec<Option<UtilityTile>>>,
    replacements: Vec<Replacement>,
) -> Vec<Vec<Option<UtilityTile>>> {
    let width = grid.len();
    for x in 0..width {
        let height = grid[x].len();
        for y in 0..height {
            for replacement in replacements.iter() {
                if grid[x][y] == Some(replacement.target)
                    || (grid[x][y] == None && replacement.target == UtilityTile::Empty)
                {
                    let above = match get_tile_above(x, y, &grid) {
                        _ if replacement.above.len() == 0 => true,
                        None if replacement.above.contains(&UtilityTile::Empty) => true,
                        None if replacement.above.len() > 0 => false,
                        Some(t) => replacement.above.contains(&t),
                        _ => true,
                    };
                    let below = match get_tile_below(x, y, &grid) {
                        _ if replacement.below.len() == 0 => true,
                        None if replacement.below.contains(&UtilityTile::Empty) => true,
                        None if replacement.below.len() > 0 => false,
                        Some(t) => replacement.below.contains(&t),
                        _ => true,
                    };
                    let left_of = match get_tile_left_of(x, y, &grid) {
                        _ if replacement.on_left.len() == 0 => true,
                        None if replacement.on_left.contains(&UtilityTile::Empty) => true,
                        None if replacement.on_left.len() > 0 => false,
                        Some(t) => replacement.on_left.contains(&t),
                        _ => true,
                    };
                    let right_of = match get_tile_right_of(x, y, &grid) {
                        _ if replacement.on_right.len() == 0 => true,
                        None if replacement.on_right.contains(&UtilityTile::Empty) => true,
                        None if replacement.on_right.len() > 0 => false,
                        Some(t) => replacement.on_right.contains(&t),
                        _ => true,
                    };

                    let top_left_of = match get_top_left_tile(x, y, &grid) {
                        _ if replacement.on_top_left.len() == 0 => true,
                        None if replacement.on_top_left.contains(&UtilityTile::Empty) => true,
                        None if replacement.on_top_left.len() > 0 => false,
                        Some(t) => replacement.on_top_left.contains(&t),
                        _ => true,
                    };
                    let top_right_of = match get_top_right_tile(x, y, &grid) {
                        _ if replacement.on_top_right.len() == 0 => true,
                        None if replacement.on_top_right.contains(&UtilityTile::Empty) => true,
                        None if replacement.on_top_right.len() > 0 => false,
                        Some(t) => replacement.on_top_right.contains(&t),
                        _ => true,
                    };
                    let bottom_left_of = match get_bottom_left_tile(x, y, &grid) {
                        _ if replacement.on_bottom_left.len() == 0 => true,
                        None if replacement.on_bottom_left.contains(&UtilityTile::Empty) => true,
                        None if replacement.on_bottom_left.len() > 0 => false,
                        Some(t) => replacement.on_bottom_left.contains(&t),
                        _ => true,
                    };
                    let bottom_right_of = match get_bottom_right_tile(x, y, &grid) {
                        _ if replacement.on_bottom_right.len() == 0 => true,
                        None if replacement.on_bottom_right.contains(&UtilityTile::Empty) => true,
                        None if replacement.on_bottom_right.len() > 0 => false,
                        Some(t) => replacement.on_bottom_right.contains(&t),
                        _ => true,
                    };

                    // println!(
                    //     "({x},{y}) above: {above}, below: {below}, left: {left_of}, right: {right_of}"
                    // );

                    if above
                        && below
                        && left_of
                        && right_of
                        && top_left_of
                        && top_right_of
                        && bottom_left_of
                        && bottom_right_of
                    {
                        grid[x][y] = Some(replacement.replacement);
                        continue;
                    }
                }
            }
        }
    }
    grid
}

pub(crate) fn is_edge(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x == (width - 1) || y == (height - 1)
}

pub(crate) fn get_tile_above(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if y > 0 { grid[x][y - 1] } else { None }
}

pub(crate) fn get_tile_below(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if y < (grid[x].len() - 1) {
        grid[x][y + 1]
    } else {
        None
    }
}

pub(crate) fn get_tile_left_of(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if x > 0 { grid[x - 1][y] } else { None }
}

pub(crate) fn get_tile_right_of(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if x < (grid.len() - 1) {
        grid[x + 1][y]
    } else {
        None
    }
}

pub(crate) fn get_top_left_tile(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if x > 0 && y > 0 {
        grid[x - 1][y - 1]
    } else {
        None
    }
}

pub(crate) fn get_top_right_tile(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if x < (grid.len() - 1) && y > 0 {
        grid[x + 1][y - 1]
    } else {
        None
    }
}

pub(crate) fn get_bottom_left_tile(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if x > 0 && y < (grid[x].len() - 1) {
        grid[x - 1][y + 1]
    } else {
        None
    }
}

pub(crate) fn get_bottom_right_tile(
    x: usize,
    y: usize,
    grid: &Vec<Vec<Option<UtilityTile>>>,
) -> Option<UtilityTile> {
    if x < (grid.len() - 1) && y < (grid[x].len() - 1) {
        grid[x + 1][y + 1]
    } else {
        None
    }
}
