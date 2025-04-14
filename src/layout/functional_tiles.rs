use bevy::ecs::component::Component;

use super::impassable::IsImpassable;
use super::replacement::Replaceable;

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UtilityTile {
    #[default]
    Test1,
    Test2,
    Test3,
    Empty,
    WallTopmost,
    WallInnerCornerTopLeft,
    WallInnerCornerTopRight,
    WallInnerCornerBottomLeft,
    WallInnerCornerBottomRight,
    WallTopUpper,
    WallTopLower,
    WallLeft,
    WallRight,
    WallBottom,
    WallTopLeft,
    WallTopRight,
    WallBottomRight,
    WallBottomLeft,
    Floor,
    FloorShadowLeft,
    FloorShadowTop,
    FloorShadowLeftTransition,
    FloorShadowTopTransition,
    FloorShadowOuterCorner,
    FloorShadowInnerCorner,
}

impl Replaceable for UtilityTile {
    fn is_empty(self: Self) -> bool {
        return self == UtilityTile::Empty;
    }
}

impl IsImpassable for UtilityTile {
    fn is_impassable(&self) -> bool {
        *self == Self::WallTopmost
            || *self == Self::WallInnerCornerTopLeft
            || *self == Self::WallInnerCornerTopRight
            || *self == Self::WallInnerCornerBottomLeft
            || *self == Self::WallInnerCornerBottomRight
            || *self == Self::WallTopUpper
            || *self == Self::WallLeft
            || *self == Self::WallRight
            || *self == Self::WallBottom
            || *self == Self::WallTopLeft
            || *self == Self::WallTopRight
            || *self == Self::WallBottomRight
            || *self == Self::WallBottomLeft
    }
}
