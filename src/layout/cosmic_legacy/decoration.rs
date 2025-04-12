use super::CosmicLegacyTile;
use crate::layout::functional_tiles::UtilityTile;
use crate::layout::functional_tiles::UtilityTile::*;
use crate::layout::replacement::{Replacement, replace_tiles};
use lazy_static::lazy_static;
use rand_chacha::ChaCha8Rng;

pub fn decorate(
    input: &Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<CosmicLegacyTile>>> {
    let width = input.len();
    let mut decorations: Vec<Vec<Option<CosmicLegacyTile>>> = vec![vec![]; width];

    for x in 0..width {
        let height = input[x].len();
        for _ in 0..height {
            decorations[x].push(None);
        }
    }

    replace_tiles(&input, VERT_DECORATIONS.to_vec(), decorations, rng)
}

lazy_static! {
    static ref VERT_DECORATIONS: Vec<Replacement<UtilityTile, CosmicLegacyTile>> = vec![
        // locker
        Replacement {
            target: UtilityTile::WallTop,
            replacement: CosmicLegacyTile::LockerClosedTop,
            replacement_below: Some(CosmicLegacyTile::LockerClosedBottom),
            condition: |ctx| {
                ctx.below == Some(Floor) && (
                    ctx.left == Some(WallTop) && ctx.right == Some(WallTop)
                )
            },
            chance: 0.2,
            ..Default::default()
        },
        // alien
        Replacement {
            target: UtilityTile::WallTop,
            replacement: CosmicLegacyTile::AlienTop,
            replacement_below: Some(CosmicLegacyTile::AlienBottom),
            condition: |ctx| {
                ctx.below == Some(Floor) && (
                    ctx.left == Some(WallTop) && ctx.right == Some(WallTop)
                )
            },
            chance: 0.2,
            ..Default::default()
        },
        // bookshelf
        Replacement {
            target: UtilityTile::WallTop,
            replacement: CosmicLegacyTile::BookcaseTop,
            replacement_below: Some(CosmicLegacyTile::BookcaseBottom),
            condition: |ctx| {
                ctx.below == Some(Floor) && (
                    ctx.left == Some(WallTop) && ctx.right == Some(WallTop)
                )
            },
            chance: 0.2,
            ..Default::default()
        },
    ];
}
