use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use tilegen::*;

// Sometimes a randomly generated room has features that can't be properly rendered into tiles
// The "fixer" removes these via constraints

pub fn fix_floor(grid: &mut TileGrid<UtilityTile>, rng: &mut ChaCha8Rng) {
    grid.apply_layer_replacements(0, FIRST_PASS.to_vec(), rng);
    grid.apply_layer_replacements(0, SECOND_PASS.to_vec(), rng);
}

lazy_static! {
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // force at least 4 tiles distance horizontally between rooms
        ReplacementRule {
            condition: |src, _dest| {
                *src == Floor && src.down() == None && src.down().down().down().down() == Floor
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
            condition: |src, _dest| {
                *src == Floor && (
                    (src.bottom_left() == None && src.top_right() == None && src.left() == Floor && src.right() == Floor) ||
                    (src.bottom_right() == None && src.top_left() == None && src.left() == Floor && src.right() == Floor)
                )
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
            condition: |src, _dest| {
                *src == Floor && src.down() == None && (
                    (src.left() == None && src.get(-1, 4, 0) == Floor) ||
                    (src.left() == None && src.get(-2, 3, 0) == Floor) ||
                    (src.right() == None && src.get(1, 4, 0) == Floor) ||
                    (src.right() == None && src.get(2, 3, 0) == Floor)
                )
            },
            replacements: vec![Replacement::new(0, 0, 0, None)],
            ..Default::default()
        },

        // eliminate one column hallways corners
        ReplacementRule {
            condition: |src, _dest| {
                *src == Floor && src.down() == Floor && (
                    (src.right() == None && src.bottom_left() == None) ||
                    (src.left() == None && src.bottom_right() == None)
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
        ReplacementRule::none_to(Floor, |src, _dest| {
            // bottom
            (src.up() == Floor && src.left() == Floor && src.right() == Floor) ||
            // left
            (src.up() == Floor && src.right() == Floor && src.down() == Floor) ||
            // top
            (src.left() == Floor && src.right() == Floor && src.down() == Floor) ||
            // right
            (src.left() == Floor && src.up() == Floor && src.down() == Floor)
        }),

        // one tile wart
        ReplacementRule::to_none(Floor, |src, _dest| {
            *src == Floor && (
                // top
                (src.left() == None && src.up() == None && src.right() == None) ||
                // bottom
                (src.left() == None && src.down() == None && src.right() == None) ||
                // left
                (src.left() == None && src.up() == None && src.down() == None) ||
                // right
                (src.right() == None && src.up() == None && src.down() == None)
            )
        }),
    ];
}
