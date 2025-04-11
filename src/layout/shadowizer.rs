use lazy_static::lazy_static;
use std::collections::HashSet;

use super::{functional_tiles::UtilityTile, replacement::*};

// TODO: make a real room generator
// - generates floors first
// - wraps floor tiles in walls

pub fn basic_room(width: usize, height: usize) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; height]; width];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if x == 0 || y == 0 || x == (width - 1) || y == (height - 1) {
                grid[x][y] = Some(UtilityTile::Wall);
            } else if x == 3 && y > 3 {
                grid[x][y] = Some(UtilityTile::Wall);
            } else if x == width - 2 && y == 2 {
                grid[x][y] = Some(UtilityTile::Wall);
            } else {
                grid[x][y] = Some(UtilityTile::Floor);
            }
        }
    }
    grid
}

pub fn shadowize(grid: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    replace_tiles(
        replace_tiles(grid, FIRST_PASS.to_vec()),
        SECOND_PASS.to_vec(),
    )
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement> = vec![
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::Wall]),
            on_left: HashSet::from([UtilityTile::Wall]),
            replacement: UtilityTile::FloorShadowInnerCorner,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::FloorShadowLeft]),
            on_left: HashSet::from([UtilityTile::FloorShadowTop]),
            replacement: UtilityTile::FloorShadowOuterCorner,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::Wall]),
            replacement: UtilityTile::FloorShadowTop,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::Floor,
            on_left: HashSet::from([UtilityTile::Wall]),
            replacement: UtilityTile::FloorShadowLeft,
            ..Default::default()
        }
    ];
    static ref SECOND_PASS: Vec<Replacement> = vec![
        Replacement {
            target: UtilityTile::FloorShadowTop,
            on_right: HashSet::from([
                UtilityTile::FloorShadowTop,
                UtilityTile::FloorShadowOuterCorner,
                UtilityTile::Wall
            ]),
            on_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::FloorShadowTopTransition,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::FloorShadowLeft,
            above: HashSet::from([UtilityTile::Floor]),
            below: HashSet::from([
                UtilityTile::FloorShadowLeft,
                UtilityTile::FloorShadowOuterCorner,
                UtilityTile::Wall
            ]),
            replacement: UtilityTile::FloorShadowLeftTransition,
            ..Default::default()
        },
    ];
}
