use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

pub fn shadowize(
    grid: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    replace_tiles(&grid, FIRST_PASS.to_vec(), grid.clone(), rng)
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // inner corner
        Replacement::from_to(Floor, FloorShadowInnerCorner, |ctx| {
            ctx.above() == WallTopLower && (
                ctx.left() == WallLeft || (ctx.left() == WallTopUpper || ctx.left() == WallTopLower) || ctx.left() == WallInnerCornerTopLeft
            )
        }),
        // outer corner
        Replacement::from_to(Floor, FloorShadowOuterCorner, |ctx| {
            ctx.above() == Floor && ctx.left() == Floor && (ctx.top_left() == WallTopUpper || ctx.top_left() == WallTopLower)
        }),
        // top transition
        Replacement::from_to(Floor, FloorShadowTopTransition, |ctx| {
            ctx.above() == WallTopLower && ctx.left() == Floor && ctx.top_left() == Floor
        }),
        // left transition
        Replacement::from_to(Floor, FloorShadowLeftTransition, |ctx| {
            ctx.above() == Floor && ctx.top_left() == Floor && (
                ctx.left() == WallLeft || ctx.left() == WallInnerCornerTopRight
            )
        }),
        // top
        Replacement::from_to(Floor, FloorShadowTop, |ctx| {
            ctx.above() == WallTopLower
        }),
        // left
        Replacement::from_to(Floor, FloorShadowLeft, |ctx| {
            ctx.left() == WallLeft
            || ctx.left() == WallTopUpper
            || ctx.left() == WallTopLower
            || ctx.left() == WallInnerCornerTopRight
            || ctx.left() == WallInnerCornerBottomRight
        }),
    ];
}
