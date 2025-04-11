use lazy_static::lazy_static;
use std::collections::HashSet;

use super::{functional_tiles::UtilityTile, replacement::*};

pub fn shadowize(grid: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    replace_tiles(
        replace_tiles(grid, FIRST_PASS.to_vec()),
        SECOND_PASS.to_vec(),
    )
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement> = vec![
        // inner corner
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::WallTop]),
            on_left: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallOutlineInnerCornerRight]),
            replacement: UtilityTile::FloorShadowInnerCorner,
            ..Default::default()
        },
        // outer corner
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::FloorShadowLeft]),
            on_left: HashSet::from([UtilityTile::FloorShadowTop]),
            replacement: UtilityTile::FloorShadowOuterCorner,
            ..Default::default()
        },
        // top
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::WallTop]),
            replacement: UtilityTile::FloorShadowTop,
            ..Default::default()
        },
        // left
        Replacement {
            target: UtilityTile::Floor,
            on_left: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallTop]),
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
                UtilityTile::WallTop,
                UtilityTile::WallLeft,
                UtilityTile::WallRight,
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
                UtilityTile::WallTop,
                UtilityTile::WallRight,
                UtilityTile::WallLeft,
            ]),
            replacement: UtilityTile::FloorShadowLeftTransition,
            ..Default::default()
        },
    ];
}
