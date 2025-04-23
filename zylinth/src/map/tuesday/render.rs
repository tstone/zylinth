use rand::prelude::*;

use crate::map::functional_tiles::UtilityTile;

use super::TuesdayTile;

/// Translate the generic utility tiles into Cosmic Legacy tiles
pub fn utility_to_tuesday(utility: UtilityTile, _rng: &mut impl Rng) -> TuesdayTile {
    match utility {
        UtilityTile::Wall => {
            TuesdayTile::WallPanelMiddle
            // TODO: rand alt pieces
        }
        UtilityTile::WallLeft => TuesdayTile::WallPanelLeft,
        UtilityTile::WallRight => TuesdayTile::WallPanelRight,
        UtilityTile::WallBorderTopLeft => TuesdayTile::WallTopLeft,
        UtilityTile::WallBorderTopRight => TuesdayTile::WallTopRight,
        UtilityTile::WallBorderTop => TuesdayTile::WallTop,
        UtilityTile::WallBorderLeft => TuesdayTile::WallLeft,
        UtilityTile::WallBorderRight => TuesdayTile::WallRight,
        UtilityTile::WallBorderBottom => TuesdayTile::WallBottom,
        UtilityTile::WallBorderBottomLeft => TuesdayTile::WallBottomLeft,
        UtilityTile::WallBorderBottomRight => TuesdayTile::WallBottomRight,
        UtilityTile::WallBorderInnerCornerTopLeft => TuesdayTile::WallInnerCornerTopLeft,
        UtilityTile::WallBorderInnerCornerBottomLeft => TuesdayTile::WallInnerCornerBottomLeft,
        UtilityTile::WallBorderInnerCornerTopRight => TuesdayTile::WallInnerCornerTopRight,
        UtilityTile::WallBorderInnerCornerBottomRight => TuesdayTile::WallInnerCornerBottomRight,
        UtilityTile::WallBorderAllCorner => TuesdayTile::WallAllCorner,
        UtilityTile::WallBorderDoubleLeftCorner => TuesdayTile::WallDoubleLeftCorner,
        UtilityTile::WallBorderDoubleRightCorner => TuesdayTile::WallDoubleRightCorner,
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
        _ => TuesdayTile::Test,
    }
}
