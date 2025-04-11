use super::{functional_tiles::UtilityTile, modifications::padding, replacement::*};
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn wrap_walls(input: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    let padded = padding(input, 2, 1, 0, 1);
    replace_tiles(
        replace_tiles(padded, FIRST_PASS.to_vec()),
        SECOND_PASS.to_vec(),
    )
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement> = vec![
        // top-left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            on_bottom_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // top-right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            on_bottom_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // top
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::Floor]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // left
        Replacement {
            target: UtilityTile::Empty,
            on_right: HashSet::from([UtilityTile::Floor]),
            on_left: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
        // right
        Replacement {
            target: UtilityTile::Empty,
            on_left: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
    ];

    static ref SECOND_PASS: Vec<Replacement> = vec![
        // 2 story wall
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::Wall]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::Wall,
            ..Default::default()
        },
    ];
}
