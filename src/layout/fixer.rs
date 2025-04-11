use super::{functional_tiles::UtilityTile, replacement::*};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

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
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Floor]),
            on_left: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::Floor,
            ..Default::default()
        },
        // one-tile cut left
        Replacement {
            target: UtilityTile::Empty,
            above: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Floor]),
            below: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::Floor,
            ..Default::default()
        },
        // one-tile cut top
        Replacement {
            target: UtilityTile::Empty,
            on_left: HashSet::from([UtilityTile::Floor]),
            on_right: HashSet::from([UtilityTile::Floor]),
            below: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::Floor,
            ..Default::default()
        },
        // one-tile cut right
        Replacement {
            target: UtilityTile::Empty,
            on_left: HashSet::from([UtilityTile::Floor]),
            above: HashSet::from([UtilityTile::Floor]),
            below: HashSet::from([UtilityTile::Floor]),
            replacement: UtilityTile::Floor,
            ..Default::default()
        },
    ];
}
