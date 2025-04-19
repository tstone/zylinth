use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*, replacement::*};
use bevy::log::*;
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

// Sometimes a randomly generated room has features that can't be properly rendered into tiles
// The "fixer" removes these via constraints

pub fn fix_floor(grid: &mut Vec<Vec<Vec<Option<UtilityTile>>>>, rng: &mut ChaCha8Rng) {
    replace_tiles(grid, 0, FIRST_PASS.to_vec(), rng);
    replace_tiles(grid, 0, SECOND_PASS.to_vec(), rng);
}

lazy_static! {
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // force at least 4 tiles distance horizontally between rooms
        ReplacementRule {
            target: Some(Floor),
            condition: |ctx| {
                ctx.down() == None && ctx.down().down().down().down() == Floor
            },
            replacements: vec![
                Replacement::new(0, 0, 0, None),
                Replacement::new(0, 1, 0, None),
                Replacement::new(0, 2, 0, None),
                Replacement::new(0, 3, 0, None),
            ],
            ..Default::default()
        },
        // eliminate one square corners (top left/bottom right)
        ReplacementRule {
            target: Some(Floor),
            condition: |ctx| {
                (ctx.bottom_left() == None && ctx.top_right() == None && ctx.left() == Floor && ctx.right() == Floor) ||
                (ctx.bottom_right() == None && ctx.top_left() == None && ctx.left() == Floor && ctx.right() == Floor)
            },
            replacements: vec![
                Replacement::top_left(Floor),
                Replacement::top_right(Floor),
                Replacement::bottom_left(Floor),
                Replacement::bottom_right(Floor),
            ],
            ..Default::default()
        },
    ];

    static ref SECOND_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // TODO: see if this can be moved to first pass:
        // space out bottom corners
        ReplacementRule {
            target: Some(Floor),
            condition: |ctx| {
                ctx.down() == None && (
                    (ctx.left() == None && ctx.get(-1, 4, 0) == Floor) ||
                    (ctx.left() == None && ctx.get(-2, 3, 0) == Floor) ||
                    (ctx.right() == None && ctx.get(1, 4, 0) == Floor) ||
                    (ctx.right() == None && ctx.get(2, 3, 0) == Floor)
                )
            },
            replacements: vec![Replacement::new(0, 0, 0, None)],
            ..Default::default()
        },

        // eliminate one column hallways corners
        ReplacementRule {
            target: Some(Floor),
            condition: |ctx| {
                ctx.down() == Floor && (
                    (ctx.right() == None && ctx.bottom_left() == None) ||
                    (ctx.left() == None && ctx.bottom_right() == None)
                )
            },
            replacements: vec![
                Replacement::left(Floor),
                Replacement::right(Floor),
                Replacement::top_left(Floor),
                Replacement::top_right(Floor),
                Replacement::bottom_left(Floor),
                Replacement::bottom_right(Floor),
            ],
            ..Default::default()
        },

        // one tile cut
        ReplacementRule::none_to(Floor, |ctx| {
            // bottom
            (ctx.up() == Floor && ctx.left() == Floor && ctx.right() == Floor) ||
            // left
            (ctx.up() == Floor && ctx.right() == Floor && ctx.down() == Floor) ||
            // top
            (ctx.left() == Floor && ctx.right() == Floor && ctx.down() == Floor) ||
            // right
            (ctx.left() == Floor && ctx.up() == Floor && ctx.down() == Floor)
        }),

        // one tile wart
        ReplacementRule::from_to_none(Floor, |ctx| {
            // top
            (ctx.left() == None && ctx.up() == None && ctx.right() == None) ||
            // bottom
            (ctx.left() == None && ctx.down() == None && ctx.right() == None) ||
            // left
            (ctx.left() == None && ctx.up() == None && ctx.down() == None) ||
            // right
            (ctx.right() == None && ctx.up() == None && ctx.down() == None)
        }),
    ];
}
