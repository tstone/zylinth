use super::{functional_tiles::UtilityTile, modifications::padding, replacement::*};
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn wrap_walls(input: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    let padded = padding(input, 2, 1, 0, 1);
    // 2 height walls
    replace_tiles(
        replace_tiles(padded, FIRST_PASS.to_vec()),
        SECOND_PASS.to_vec(),
    )
    // top cap
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement> = vec![
        // top-left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            on_bottom_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTopLeft,
            ..Default::default()
        },
        // top-right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            on_bottom_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTopRight,
            ..Default::default()
        },
        // inner top-left
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Floor]),
            on_bottom_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // inner top-right
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::Floor]),
            on_left: HashSet::from([UtilityTile::Floor]),
            on_bottom_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // top
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::Floor]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // left
        Replacement {
            target: UtilityTile::Empty,
            on_right: HashSet::from([UtilityTile::Floor]),
            on_left: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallLeft,
            ..Default::default()
        },
        // right
        Replacement {
            target: UtilityTile::Empty,
            on_left: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallRight,
            ..Default::default()
        },
    ];

    static ref SECOND_PASS: Vec<Replacement> = vec![
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallTopLeft]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallTopLeft,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::WallTopLeft,
            above: HashSet::from([UtilityTile::WallTopLeft]),
            replacement: UtilityTile::WallLeft,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallTopRight]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallTopRight,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::WallTopRight,
            above: HashSet::from([UtilityTile::WallTopRight]),
            replacement: UtilityTile::WallRight,
            ..Default::default()
        },
        // inner-left corner 2nd wall
        Replacement {
            target: UtilityTile::WallLeft,
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // inner-right corner 2nd wall
        Replacement {
            target: UtilityTile::WallRight,
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // 2 story wall
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallTop]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
    ];

    static ref THIRD_PASS: Vec<Replacement> = vec![
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallTopLeft]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallTopLeft,
            ..Default::default()
        },
    ];
}
