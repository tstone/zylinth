use super::{functional_tiles::UtilityTile, modifications::padding, replacement::*};
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn wrap_walls(input: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    let padded = padding(input, 3, 1, 1, 1);
    // 2 height walls
    let walled = replace_tiles(
        replace_tiles(padded, FIRST_PASS.to_vec()),
        SECOND_PASS.to_vec(),
    );
    // top cap
    replace_tiles(walled, THIRD_PASS.to_vec())
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement> = vec![
        // top-left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            on_bottom_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallOutlineTopLeft,
            ..Default::default()
        },
        // top-right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Empty]),
            on_bottom_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallOutlineTopRight,
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
        // bottom
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallBottom,
            ..Default::default()
        },
        // bottom left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallLeft]),
            replacement: UtilityTile::WallBottomLeft,
            ..Default::default()
        },
        // bottom right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallRight]),
            on_left: HashSet::from([UtilityTile::WallBottom, UtilityTile::WallBottomLeft]),
            replacement: UtilityTile::WallBottomRight,
            ..Default::default()
        },
    ];

    static ref SECOND_PASS: Vec<Replacement> = vec![
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallOutlineTopLeft]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallOutlineTopLeft,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::WallOutlineTopLeft,
            above: HashSet::from([UtilityTile::WallOutlineTopLeft]),
            replacement: UtilityTile::WallLeft,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallOutlineTopRight]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallOutlineTopRight,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::WallOutlineTopRight,
            above: HashSet::from([UtilityTile::WallOutlineTopRight]),
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
        //
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallOutlineTopLeft]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallOutlineTopLeft,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::WallOutlineTopLeft,
            above: HashSet::from([UtilityTile::WallOutlineTopLeft]),
            replacement: UtilityTile::WallLeft,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallOutlineTopRight]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallOutlineTopRight,
            ..Default::default()
        },
        Replacement {
            target: UtilityTile::WallOutlineTopRight,
            above: HashSet::from([UtilityTile::WallOutlineTopRight]),
            replacement: UtilityTile::WallRight,
            ..Default::default()
        },
        // Top cap
        Replacement {
            target: UtilityTile::Empty,
            below: HashSet::from([UtilityTile::WallTop]),
            above: HashSet::from([UtilityTile::Empty]),
            replacement: UtilityTile::WallOutlineTop,
            ..Default::default()
        },
        // inner bottom-right corner
        Replacement {
            target: UtilityTile::WallLeft,
            on_left: HashSet::from([UtilityTile::WallOutlineTop, UtilityTile::WallOutlineTopLeft]),
            above: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineTopLeft]),
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallOutlineInnerCornerRight,
            ..Default::default()
        },
        // inner bottom-left corner
        Replacement {
            target: UtilityTile::WallRight,
            on_left: HashSet::from([UtilityTile::Floor]),
            above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineTopRight]),
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallOutlineInnerCornerLeft,
            ..Default::default()
        },
    ];
}
