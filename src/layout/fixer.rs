use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

// Sometimes a randomly generated room has features that can't be properly rendered into tiles
// The "fixer" removes these via constraints

pub fn floor_fixer(
    grid: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let mut fixed = replace_tiles(&grid, FIRST_PASS.to_vec(), grid.clone(), rng);

    for x in 0..fixed.len() {
        for y in 0..fixed[x].len() {
            if fixed[x][y] == Some(Empty) {
                fixed[x][y] = None;
            }
        }
    }

    fixed
}

lazy_static! {
    static ref FIRST_PASS: Vec<Replacement<UtilityTile, UtilityTile>> = vec![
        // one tile vertical gap
        // Replacement::from_to(Empty, Floor, |ctx| {
        //     ctx.above() == Floor && ctx.below() == Floor && ctx.left() == None && ctx.right() == None
        // }),
        // one tile cut bottom
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.above() == Floor && ctx.left() == Floor && ctx.right() == Floor
        }),
        // one tile cut left
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.above() == Floor && ctx.right() == Floor && ctx.below() == Floor
        }),
        // one tile cut top
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.left() == Floor && ctx.right() == Floor && ctx.below() == Floor
        }),
        // one tile cut right
        Replacement::from_to(Empty, Floor, |ctx| {
            ctx.left() == Floor && ctx.above() == Floor && ctx.below() == Floor
        }),
        // one tile wart top
        Replacement::from_to(Floor, Empty, |ctx| {
            ctx.left() == None && ctx.above() == None && ctx.right() == None
        }),
        // one tile wart bottom
        Replacement::from_to(Floor, Empty, |ctx| {
            ctx.left() == None && ctx.below() == None && ctx.right() == None
        }),
        // one tile wart left
        Replacement::from_to(Floor, Empty, |ctx| {
            ctx.left() == None && ctx.above() == None && ctx.below() == None
        }),
        // one tile wart right
        Replacement::from_to(Floor, Empty, |ctx| {
            ctx.right() == None && ctx.above() == None && ctx.below() == None
        }),
        // avoid tightly packed corners
        Replacement {
            target: Floor,
            replacement: Floor,
            replacement_bottom_left: Some(Empty),
            condition: |ctx| {
                ctx.top_right() == Floor && ctx.bottom_left() == Floor &&
                ctx.below() == None && ctx.left() == None
            },
            ..Default::default()
        },
    ];
}
