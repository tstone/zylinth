use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

pub fn shadowize(grid: &mut Vec<Vec<Vec<Option<UtilityTile>>>>, rng: &mut ChaCha8Rng) {
    replace_tiles(grid, 0, FIRST_PASS.to_vec(), rng)
}

lazy_static! {
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // inner corner
        ReplacementRule::from_to(Floor, FloorShadowInnerCorner, |ctx| {
            ctx.up() == WallTopLower && (
                ctx.left() == WallLeft || (ctx.left() == WallTopUpper || ctx.left() == WallTopLower) || ctx.left() == WallInnerCornerTopLeft
            )
        }),
        // outer corner
        ReplacementRule::from_to(Floor, FloorShadowOuterCorner, |ctx| {
            ctx.up() == Floor && ctx.left() == Floor && (ctx.top_left() == WallTopUpper || ctx.top_left() == WallTopLower)
        }),
        // top transition
        ReplacementRule::from_to(Floor, FloorShadowTopTransition, |ctx| {
            ctx.up() == WallTopLower && ctx.left() == Floor && ctx.top_left() == Floor
        }),
        // left transition
        ReplacementRule::from_to(Floor, FloorShadowLeftTransition, |ctx| {
            ctx.up() == Floor && ctx.top_left() == Floor && (
                ctx.left() == WallLeft || ctx.left() == WallInnerCornerTopRight
            )
        }),
        // top
        ReplacementRule::from_to(Floor, FloorShadowTop, |ctx| {
            ctx.up() == WallTopLower
        }),
        // left
        ReplacementRule::from_to(Floor, FloorShadowLeft, |ctx| {
            ctx.left() == WallLeft
            || ctx.left() == WallTopUpper
            || ctx.left() == WallTopLower
            || ctx.left() == WallInnerCornerTopRight
            || ctx.left() == WallInnerCornerBottomRight
        }),
    ];
}
