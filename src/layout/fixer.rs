use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

// Sometimes a randomly generated room has features that can't be properly rendered into tiles
// The "fixer" removes these via constraints

pub fn floor_fixer(
    grid: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    replace_tiles(&grid, FIRST_PASS.to_vec(), grid.clone(), rng)
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // one-tile cut bottom
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.above == Some(Floor) && ctx.left == Some(Floor) && ctx.right == Some(Floor)
        }),
        // one-tile cut left
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.above == Some(Floor) && ctx.right == Some(Floor) && ctx.below == Some(Floor)
        }),
        // one-tile cut top
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.left == Some(Floor) && ctx.right == Some(Floor) && ctx.below == Some(Floor)
        }),
        // one-tile cut right
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.left == Some(Floor) && ctx.above == Some(Floor) && ctx.below == Some(Floor)
        }),
    ];
}
