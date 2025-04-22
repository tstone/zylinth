use rand::prelude::*;

use crate::layout::functional_tiles::UtilityTile;

use super::TuesdayTile;

/// Translate the generic utility tiles into Cosmic Legacy tiles
pub fn utility_to_tuesday(utility: UtilityTile, _rng: &mut impl Rng) -> TuesdayTile {
    match utility {
        UtilityTile::WallTopLower | UtilityTile::WallTopUpper => {
            TuesdayTile::WallPanelMiddle
            // TODO: rand alt pieces
        }
        UtilityTile::WallTopLeft => TuesdayTile::WallTopLeft,
        UtilityTile::WallTopRight => TuesdayTile::WallTopRight,
        UtilityTile::WallLeft => TuesdayTile::WallLeft,
        UtilityTile::WallRight => TuesdayTile::WallRight,
        UtilityTile::WallBottom => TuesdayTile::WallBottom,
        UtilityTile::WallBottomLeft => TuesdayTile::WallBottomLeft,
        UtilityTile::WallBottomRight => TuesdayTile::WallBottomRight,
        UtilityTile::WallTopmost => TuesdayTile::Empty,
        UtilityTile::WallInnerCornerTopLeft => TuesdayTile::WallInnerCornerTopLeft,
        UtilityTile::WallInnerCornerBottomLeft => TuesdayTile::WallInnerCornerBottomLeft,
        UtilityTile::WallInnerCornerTopRight => TuesdayTile::WallInnerCornerTopRight,
        UtilityTile::WallInnerCornerBottomRight => TuesdayTile::WallInnerCornerBottomRight,
        UtilityTile::Floor => {
            TuesdayTile::Floor
            // TODO random alt piece
        }
        UtilityTile::FloorShadowLeft => TuesdayTile::Floor,
        UtilityTile::FloorShadowTop => TuesdayTile::Floor,
        UtilityTile::FloorShadowLeftTransition => TuesdayTile::Floor,
        UtilityTile::FloorShadowTopTransition => TuesdayTile::Floor,
        UtilityTile::FloorShadowOuterCorner => TuesdayTile::Floor,
        UtilityTile::FloorShadowInnerCorner => TuesdayTile::Floor,

        // test & misc
        UtilityTile::PlayerStart => TuesdayTile::Floor,
        _ => TuesdayTile::Empty,
    }
}
