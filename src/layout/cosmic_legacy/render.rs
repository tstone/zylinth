use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::layout::functional_tiles::UtilityTile;

use super::CosmicLegacyTile;

/// Render the generic utility tiles into Cosmic Legacy tiles
pub fn utility_to_cosmic(
    input: Vec<Vec<Option<UtilityTile>>>,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<CosmicLegacyTile>>> {
    let mut output: Vec<Vec<Option<CosmicLegacyTile>>> = vec![vec![]; input.len()];
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let tile = match input[x][y] {
                Some(UtilityTile::WallTop) => {
                    if rand::random_bool(0.9) {
                        Some(CosmicLegacyTile::Wall)
                    } else {
                        CosmicLegacyTile::wall_tiles().choose(rng).copied()
                    }
                }
                Some(UtilityTile::WallOutlineTopLeft) => Some(CosmicLegacyTile::TopLeftOuterCorner),
                Some(UtilityTile::WallOutlineTopRight) => {
                    Some(CosmicLegacyTile::TopRightOuterCorner)
                }
                Some(UtilityTile::WallLeft) => Some(CosmicLegacyTile::TopCapLeft),
                Some(UtilityTile::WallRight) => Some(CosmicLegacyTile::TopCapRight),
                Some(UtilityTile::WallBottom) => Some(CosmicLegacyTile::TopCapBottomSimple),
                Some(UtilityTile::WallBottomLeft) => Some(CosmicLegacyTile::BottomLeftOuterCorner),
                Some(UtilityTile::WallBottomRight) => {
                    Some(CosmicLegacyTile::BottomRightOuterCorner)
                }
                Some(UtilityTile::WallOutlineTop) => Some(CosmicLegacyTile::TopCapTopSimple),
                Some(UtilityTile::WallOutlineInnerCornerTopLeft) => {
                    Some(CosmicLegacyTile::TopLeftInnerCorner)
                }
                Some(UtilityTile::WallOutlineInnerCornerBottomLeft) => {
                    Some(CosmicLegacyTile::BottomLeftCorner)
                }
                Some(UtilityTile::WallOutlineInnerCornerTopRight) => {
                    Some(CosmicLegacyTile::TopRightInnerCorner)
                }
                Some(UtilityTile::WallOutlineInnerCornerBottomRight) => {
                    Some(CosmicLegacyTile::BottomRightCorner)
                }
                Some(UtilityTile::Floor) => {
                    if rand::random_bool(0.95) {
                        Some(CosmicLegacyTile::Floor)
                    } else {
                        CosmicLegacyTile::floor_tiles().choose(rng).copied()
                    }
                }
                Some(UtilityTile::FloorShadowLeft) => Some(CosmicLegacyTile::FloorShadowLeft),
                Some(UtilityTile::FloorShadowTop) => Some(CosmicLegacyTile::FloorShadowTop),
                Some(UtilityTile::FloorShadowLeftTransition) => {
                    Some(CosmicLegacyTile::FloorShadowLeftFadeUp)
                }
                Some(UtilityTile::FloorShadowTopTransition) => {
                    Some(CosmicLegacyTile::FloorShadowTopFadeLeft)
                }
                Some(UtilityTile::FloorShadowOuterCorner) => {
                    Some(CosmicLegacyTile::FloorShadowOuterCorner)
                }
                Some(UtilityTile::FloorShadowInnerCorner) => {
                    Some(CosmicLegacyTile::FloorShadowInnerCorner)
                }
                Some(UtilityTile::Empty) => None,
                Some(UtilityTile::Test1) => Some(CosmicLegacyTile::Pink),
                Some(UtilityTile::Test2) => Some(CosmicLegacyTile::LightYellow),
                Some(UtilityTile::Test3) => Some(CosmicLegacyTile::DarkestBlue),
                None => None,
            };
            output[x].push(tile);
        }
    }
    output
}
