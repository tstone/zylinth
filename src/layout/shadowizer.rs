use super::{functional_tiles::UtilityTile, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub fn shadowize(
    grid: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let first_pass = replace_tiles(&grid, FIRST_PASS.to_vec(), grid.clone(), rng);
    replace_tiles(&first_pass, SECOND_PASS.to_vec(), first_pass.clone(), rng)
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // inner corner
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::WallTop]),
            on_left: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallTop, UtilityTile::WallOutlineInnerCornerTopLeft]),
            replacement: UtilityTile::FloorShadowInnerCorner,
            ..Default::default()
        },
        // outer corner
        Replacement {
            target: UtilityTile::Floor,
            above: HashSet::from([UtilityTile::FloorShadowLeft, UtilityTile::FloorShadowInnerCorner]),
            on_left: HashSet::from([UtilityTile::FloorShadowTop, UtilityTile::FloorShadowInnerCorner]),
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
            on_left: HashSet::from([UtilityTile::WallLeft, UtilityTile::WallTop, UtilityTile::WallOutlineInnerCornerTopRight]),
            replacement: UtilityTile::FloorShadowLeft,
            ..Default::default()
        }
    ];

    static ref SECOND_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
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
                UtilityTile::WallOutlineInnerCornerBottomLeft,
                UtilityTile::WallOutlineInnerCornerBottomRight,
                UtilityTile::WallOutlineInnerCornerTopLeft,
                UtilityTile::WallOutlineInnerCornerTopRight,
                UtilityTile::WallTop,
                UtilityTile::WallRight,
                UtilityTile::WallLeft,
                UtilityTile::WallBottom,
            ]),
            replacement: UtilityTile::FloorShadowLeftTransition,
            ..Default::default()
        },
    ];
}
