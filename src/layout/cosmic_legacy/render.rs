use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::layout::functional_tiles::UtilityTile;

use super::CosmicLegacyTile;

/// Translate the generic utility tiles into Cosmic Legacy tiles
pub fn utility_to_cosmic(utility: UtilityTile, rng: &mut ChaCha8Rng) -> CosmicLegacyTile {
    match utility {
        UtilityTile::WallTopLower | UtilityTile::WallTopUpper => {
            if rand::random_bool(0.9) {
                CosmicLegacyTile::Wall
            } else {
                *CosmicLegacyTile::wall_tiles().choose(rng).unwrap()
            }
        }
        UtilityTile::WallTopLeft => CosmicLegacyTile::TopLeftOuterCorner,
        UtilityTile::WallTopRight => CosmicLegacyTile::TopRightOuterCorner,
        UtilityTile::WallLeft => CosmicLegacyTile::TopCapLeft,
        UtilityTile::WallRight => CosmicLegacyTile::TopCapRight,
        UtilityTile::WallBottom => CosmicLegacyTile::TopCapBottomSimple,
        UtilityTile::WallBottomLeft => CosmicLegacyTile::BottomLeftOuterCorner,
        UtilityTile::WallBottomRight => CosmicLegacyTile::BottomRightOuterCorner,
        UtilityTile::WallTopmost => CosmicLegacyTile::TopCapTopSimple,
        UtilityTile::WallInnerCornerTopLeft => CosmicLegacyTile::TopLeftInnerCorner,
        UtilityTile::WallInnerCornerBottomLeft => CosmicLegacyTile::BottomLeftCorner,
        UtilityTile::WallInnerCornerTopRight => CosmicLegacyTile::TopRightInnerCorner,
        UtilityTile::WallInnerCornerBottomRight => CosmicLegacyTile::BottomRightCorner,
        UtilityTile::Floor => {
            if rand::random_bool(0.95) {
                CosmicLegacyTile::Floor
            } else {
                *CosmicLegacyTile::floor_tiles().choose(rng).unwrap()
            }
        }
        UtilityTile::FloorShadowLeft => CosmicLegacyTile::FloorShadowLeft,
        UtilityTile::FloorShadowTop => CosmicLegacyTile::FloorShadowTop,
        UtilityTile::FloorShadowLeftTransition => CosmicLegacyTile::FloorShadowLeftFadeUp,
        UtilityTile::FloorShadowTopTransition => CosmicLegacyTile::FloorShadowTopFadeLeft,
        UtilityTile::FloorShadowOuterCorner => CosmicLegacyTile::FloorShadowOuterCorner,
        UtilityTile::FloorShadowInnerCorner => CosmicLegacyTile::FloorShadowInnerCorner,

        UtilityTile::Empty => CosmicLegacyTile::DarkestBlue,
        UtilityTile::Test1 => CosmicLegacyTile::Pink,
        UtilityTile::Test2 => CosmicLegacyTile::LightYellow,
        UtilityTile::Test3 => CosmicLegacyTile::DarkestBlue,
    }
}
