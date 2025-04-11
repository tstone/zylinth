use super::{functional_tiles::UtilityTile, modifications::padding, replacement::*};
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn wrap_walls(input: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    let padded = padding(input, 3, 1, 1, 1);

    // return replace_tiles(padded, FIRST_PASS.to_vec());

    // 2 height walls
    let walled = replace_tiles(
        replace_tiles(padded, FIRST_PASS.to_vec()),
        SECOND_PASS.to_vec(),
    );
    // return walled;
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
            on_bottom_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // inner top-right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineTopLeft, UtilityTile::Empty]),
            below: HashSet::from([UtilityTile::Floor]),
            on_left: HashSet::from([UtilityTile::Floor]),
            on_bottom_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // inner bottom left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineTopRight, UtilityTile::Empty]),
            on_left: HashSet::from([UtilityTile::Floor]),
            on_bottom_left: HashSet::from([UtilityTile::Floor]),
            below: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // inner bottom right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallRight]),
            on_right: HashSet::from([UtilityTile::Floor]),
            on_bottom_right: HashSet::from([UtilityTile::Floor]),
            below: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallTop,
            ..Default::default()
        },
        // inner top left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Floor]),
            on_left: HashSet::from([UtilityTile::Floor]),
            on_top_left: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallOutlineInnerCornerTopLeft,
            ..Default::default()
        },
        // inner top right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Floor]),
            on_top_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::WallOutlineInnerCornerTopRight,
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
        // bottom outer left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineInnerCornerTopRight]),
            replacement: UtilityTile::WallBottomLeft,
            ..Default::default()
        },
        // bottom outer right
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineInnerCornerTopLeft]),
            on_left: HashSet::from([UtilityTile::WallBottom, UtilityTile::WallBottomLeft, UtilityTile::WallOutlineInnerCornerTopLeft]),
            replacement: UtilityTile::WallBottomRight,
            ..Default::default()
        },
    ];

    static ref SECOND_PASS: Vec<Replacement> = vec![
        // move outline top left up one
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
        // move outline top right up one
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
        // inner bottom left
        Replacement {
            target: UtilityTile::WallOutlineTopRight,
            on_left: HashSet::from([UtilityTile::WallTop]),
            on_bottom_left: HashSet::from([UtilityTile::WallTop]),
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallOutlineInnerCornerBottomLeft,
            ..Default::default()
        },
        // inner bottom right
        Replacement {
            target: UtilityTile::WallOutlineTopLeft,
            on_right: HashSet::from([UtilityTile::WallTop]),
            on_bottom_right: HashSet::from([UtilityTile::WallTop]),
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallOutlineInnerCornerBottomRight,
            ..Default::default()
        },
        // move caps up a third time to be on top of 2nd wall
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
        Replacement {
            target: UtilityTile::WallOutlineInnerCornerTopRight,
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallTop,
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
            replacement: UtilityTile::WallOutlineInnerCornerBottomRight,
            ..Default::default()
        },
        // inner bottom-left corner
        Replacement {
            target: UtilityTile::WallRight,
            on_left: HashSet::from([UtilityTile::Floor]),
            above: HashSet::from([UtilityTile::WallRight, UtilityTile::WallOutlineTopRight]),
            below: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::WallOutlineInnerCornerBottomLeft,
            ..Default::default()
        },
    ];
}
