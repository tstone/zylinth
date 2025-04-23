use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use tilegen::*;

pub fn shadowize(grid: &mut TileGrid<UtilityTile>, rng: &mut ChaCha8Rng) {
    grid.apply_layer_replacements(0, FIRST_PASS.to_vec(), rng)
}

lazy_static! {
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // inner corner
        ReplacementRule::to(FloorShadowInnerCorner, |src, _| {
            *src == Floor && src.up() == Wall && (
                src.left() == WallBorderLeft || (src.left() == Wall || src.left() == Wall) || src.left() == WallBorderInnerCornerTopLeft
            )
        }),
        // outer corner
        ReplacementRule::to(FloorShadowOuterCorner, |src, _| {
            *src == Floor && src.up() == Floor && src.left() == Floor
            && (src.top_left() == Wall || src.top_left() == Wall)
        }),
        // top transition
        ReplacementRule::to(FloorShadowTopTransition, |src, _| {
            *src == Floor &&  src.up() == Wall && src.left() == Floor && src.top_left() == Floor
        }),
        // left transition
        ReplacementRule::to(FloorShadowLeftTransition, |src, _| {
            *src == Floor && src.up() == Floor && src.top_left() == Floor && (
                src.left() == WallBorderLeft || src.left() == WallBorderInnerCornerTopRight
            )
        }),
        // top
        ReplacementRule::to(FloorShadowTop, |src, _| {
            *src == Floor && src.up() == Wall
        }),
        // left
        ReplacementRule::to(FloorShadowLeft, |src, _| {
            *src == Floor && (
                src.left() == WallBorderLeft
                || src.left() == Wall
                || src.left() == Wall
                || src.left() == WallBorderInnerCornerTopRight
                || src.left() == WallBorderInnerCornerBottomRight
            )
        }),
    ];
}
