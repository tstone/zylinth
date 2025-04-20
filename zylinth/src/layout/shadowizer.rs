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
        ReplacementRule::to(FloorShadowInnerCorner, |src, _dest| {
            *src == Floor && src.up() == WallTopLower && (
                src.left() == WallLeft || (src.left() == WallTopUpper || src.left() == WallTopLower) || src.left() == WallInnerCornerTopLeft
            )
        }),
        // outer corner
        ReplacementRule::to(FloorShadowOuterCorner, |src, _dest| {
            *src == Floor && src.up() == Floor && src.left() == Floor
            && (src.top_left() == WallTopUpper || src.top_left() == WallTopLower)
        }),
        // top transition
        ReplacementRule::to(FloorShadowTopTransition, |src, _dest| {
            *src == Floor &&  src.up() == WallTopLower && src.left() == Floor && src.top_left() == Floor
        }),
        // left transition
        ReplacementRule::to(FloorShadowLeftTransition, |src, _dest| {
            *src == Floor && src.up() == Floor && src.top_left() == Floor && (
                src.left() == WallLeft || src.left() == WallInnerCornerTopRight
            )
        }),
        // top
        ReplacementRule::to(FloorShadowTop, |src, _dest| {
            *src == Floor && src.up() == WallTopLower
        }),
        // left
        ReplacementRule::to(FloorShadowLeft, |src, _dest| {
            *src == Floor && (
                src.left() == WallLeft
                || src.left() == WallTopUpper
                || src.left() == WallTopLower
                || src.left() == WallInnerCornerTopRight
                || src.left() == WallInnerCornerBottomRight
            )
        }),
    ];
}
