use rand::prelude::*;

use crate::layout::functional_tiles::UtilityTile;

use super::CosmicLegacyTile;

/// Translate the generic utility tiles into Cosmic Legacy tiles
pub fn utility_to_cosmic(utility: UtilityTile, rng: &mut impl Rng) -> CosmicLegacyTile {
    match utility {
        UtilityTile::Wall | UtilityTile::Wall => {
            if rand::random_bool(0.9) {
                CosmicLegacyTile::Wall
            } else {
                *CosmicLegacyTile::wall_tiles().choose(rng).unwrap()
            }
        }
        UtilityTile::WallBorderTopLeft => CosmicLegacyTile::TopLeftOuterCorner,
        UtilityTile::WallBorderTopRight => CosmicLegacyTile::TopRightOuterCorner,
        UtilityTile::WallBorderLeft => CosmicLegacyTile::TopCapLeft,
        UtilityTile::WallBorderRight => CosmicLegacyTile::TopCapRight,
        UtilityTile::WallBorderBottom => CosmicLegacyTile::TopCapBottomSimple,
        UtilityTile::WallBorderBottomLeft => CosmicLegacyTile::BottomLeftOuterCorner,
        UtilityTile::WallBorderBottomRight => CosmicLegacyTile::BottomRightOuterCorner,
        // UtilityTile::WallTopmost => CosmicLegacyTile::TopCapTopSimple,
        UtilityTile::WallBorderInnerCornerTopLeft => CosmicLegacyTile::TopLeftInnerCorner,
        UtilityTile::WallBorderInnerCornerBottomLeft => CosmicLegacyTile::BottomLeftCorner,
        UtilityTile::WallBorderInnerCornerTopRight => CosmicLegacyTile::TopRightInnerCorner,
        UtilityTile::WallBorderInnerCornerBottomRight => CosmicLegacyTile::BottomRightCorner,
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

        // decoration
        UtilityTile::VertDecorationTop(1) => CosmicLegacyTile::LockerClosedTop,
        UtilityTile::VertDecorationBottom(1) => CosmicLegacyTile::LockerClosedBottom,
        UtilityTile::VertDecorationTop(2) => CosmicLegacyTile::LockerOpenTop,
        UtilityTile::VertDecorationBottom(2) => CosmicLegacyTile::LockerOpenBottom,
        UtilityTile::VertDecorationTop(3) => CosmicLegacyTile::AlienTop,
        UtilityTile::VertDecorationBottom(3) => CosmicLegacyTile::AlienBottom,
        UtilityTile::VertDecorationTop(4) => CosmicLegacyTile::BookcaseTop,
        UtilityTile::VertDecorationBottom(4) => CosmicLegacyTile::BookcaseBottom,

        // test & misc
        UtilityTile::Test => CosmicLegacyTile::DarkestBlue,
        UtilityTile::PlayerStart => CosmicLegacyTile::Floor,

        _ => CosmicLegacyTile::DarkestBlue,
    }
}
