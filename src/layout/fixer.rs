use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

// Sometimes a randomly generated room has features that can't be properly rendered into tiles
// The "fixer" removes these via constraints

pub fn fix_floor(grid: &mut Vec<Vec<Vec<Option<UtilityTile>>>>, rng: &mut ChaCha8Rng) {
    replace_tiles(grid, 0, FIRST_PASS.to_vec(), rng);

    // swap 'Empty' for None
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y][0] == Some(Empty) {
                grid[x][y][0] = None;
            }
        }
    }
}

lazy_static! {
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // one tile vertical gap
        // Replacement::from_to(Empty, Floor, |ctx| {
        //     ctx.above() == Floor && ctx.below() == Floor && ctx.left() == None && ctx.right() == None
        // }),
        // one tile cut bottom
        ReplacementRule::from_to(Empty, Floor, |ctx| {
            ctx.up() == Floor && ctx.left() == Floor && ctx.right() == Floor
        }),
        // one tile cut left
        ReplacementRule::from_to(Empty, Floor, |ctx| {
            ctx.up() == Floor && ctx.right() == Floor && ctx.down() == Floor
        }),
        // one tile cut top
        ReplacementRule::from_to(Empty, Floor, |ctx| {
            ctx.left() == Floor && ctx.right() == Floor && ctx.down() == Floor
        }),
        // one tile cut right
        ReplacementRule::from_to(Empty, Floor, |ctx| {
            ctx.left() == Floor && ctx.up() == Floor && ctx.down() == Floor
        }),
        // one tile wart top
        ReplacementRule::from_to(Floor, Empty, |ctx| {
            ctx.left() == None && ctx.up() == None && ctx.right() == None
        }),
        // one tile wart bottom
        ReplacementRule::from_to(Floor, Empty, |ctx| {
            ctx.left() == None && ctx.down() == None && ctx.right() == None
        }),
        // one tile wart left
        ReplacementRule::from_to(Floor, Empty, |ctx| {
            ctx.left() == None && ctx.up() == None && ctx.down() == None
        }),
        // one tile wart right
        ReplacementRule::from_to(Floor, Empty, |ctx| {
            ctx.right() == None && ctx.up() == None && ctx.down() == None
        }),
        // avoid tightly packed corners
        ReplacementRule {
            target: Floor,
            condition: |ctx| {
                ctx.top_right() == Floor && ctx.bottom_left() == Floor &&
                ctx.down() == None && ctx.left() == None
            },
            replacements: vec![
                Replacement::this(Floor),
                Replacement::bottom_left(Floor),
            ],
            ..Default::default()
        },
    ];
}
