use super::{functional_tiles::UtilityTile, functional_tiles::UtilityTile::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use tilegen::*;

// Sometimes a randomly generated room has features that can't be properly rendered into tiles
// The "fixer" removes these via constraints

pub fn fix_floor(grid: &mut TileGrid<UtilityTile>, rng: &mut ChaCha8Rng) {
    grid.apply_layer_replacements(0, FIRST_PASS.to_vec(), rng);
}

lazy_static! {
    static ref FIRST_PASS: Vec<ReplacementRule<UtilityTile>> = vec![
        // one tile cut
        ReplacementRule {
            condition: |src, _| {
                *src == None && (
                    // bottom
                    (src.up() == Floor && src.left() == Floor && src.right() == Floor) ||
                    // left
                    (src.up() == Floor && src.right() == Floor && src.down() == Floor) ||
                    // top
                    (src.left() == Floor && src.right() == Floor && src.down() == Floor) ||
                    // right
                    (src.left() == Floor && src.up() == Floor && src.down() == Floor)
                )
            },
            replacements: vec![Replacement::this(Floor)],
            ..Default::default()
        },

        // one tile wart
        ReplacementRule::to_none(Floor, |src, _| {
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
