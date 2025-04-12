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
            ctx.above == Some(WallTop) && (
                ctx.left == Some(WallLeft) || ctx.left == Some(WallTop) || ctx.left == Some(WallInnerCornerTopLeft)
            )
        }),
        // outer corner
        Replacement::from_to(Floor, FloorShadowOuterCorner, |ctx| {
            ctx.above == Some(Floor) && ctx.left == Some(Floor) && ctx.top_left == Some(WallTop)
        }),
        // top transition
        Replacement::from_to(Floor, FloorShadowTopTransition, |ctx| {
            ctx.above == Some(WallTop) && ctx.left == Some(Floor) && ctx.top_left == Some(Floor)
        }),
        // left transition
        Replacement::from_to(Floor, FloorShadowLeftTransition, |ctx| {
            ctx.above == Some(Floor) && ctx.top_left == Some(Floor) && (
                ctx.left == Some(WallLeft) || ctx.left == Some(WallInnerCornerTopRight)
            )
        }),
        // top
        Replacement::from_to(Floor, FloorShadowTop, |ctx| {
            ctx.above == Some(WallTop)
        }),
        // left
        Replacement::from_to(Floor, FloorShadowLeft, |ctx| {
            ctx.left == Some(WallLeft) || ctx.left == Some(WallTop) || ctx.left == Some(WallInnerCornerTopRight)
        }),
    ];
}
