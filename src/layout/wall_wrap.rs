use super::{functional_tiles::UtilityTile, replacement::*};
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn wrap_walls(grid: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    replace_tiles(grid, FIRST_PASS.to_vec())
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement> = vec![
        // top
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Floor, UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // bottom
        Replacement {
            target: UtilityTile::Floor,
            below: HashSet::from([UtilityTile::Empty]),
            above: HashSet::from([UtilityTile::Floor, UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // left
        Replacement {
            target: UtilityTile::Floor,
            on_left: HashSet::from([UtilityTile::Empty]),
            on_right: HashSet::from([UtilityTile::Floor, UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // right
        Replacement {
            target: UtilityTile::Floor,
            on_right: HashSet::from([UtilityTile::Empty]),
            on_left: HashSet::from([UtilityTile::Floor, UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // bottom-right corner
        Replacement {
            target: UtilityTile::Floor,
            on_right: HashSet::from([UtilityTile::Empty]),
            on_left: HashSet::from([UtilityTile::Floor, UtilityTile::Empty, UtilityTile::Wall]),
            above: HashSet::from([UtilityTile::Wall]),
            below: HashSet::from([UtilityTile::Floor, UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // // top-left inner corner
        // Replacement {
        //     target: UtilityTile::Floor,
        //     on_left: HashSet::from([UtilityTile::Wall]),
        //     above: HashSet::from([UtilityTile::Wall]),
        //     on_top_left: HashSet::from([UtilityTile::Empty]),
        //     replacement: UtilityTile::Wall,
        //     ..Default::default()
        // },
        // // top-right inner corner
        // Replacement {
        //     target: UtilityTile::Floor,
        //     on_right: HashSet::from([UtilityTile::Wall]),
        //     above: HashSet::from([UtilityTile::Wall]),
        //     on_top_right: HashSet::from([UtilityTile::Empty]),
        //     replacement: UtilityTile::Wall,
        //     ..Default::default()
        // },
    ];
}
