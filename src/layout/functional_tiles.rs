use bevy::ecs::component::Component;

use super::IsImpassable;

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UtilityTile {
    #[default]
    Test1,
    Test2,
    Test3,
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
    PlayerStart,
    Floor,
    FloorShadowLeft,
    FloorShadowTop,
    FloorShadowLeftTransition,
    FloorShadowTopTransition,
    FloorShadowOuterCorner,
    FloorShadowInnerCorner,
    // Decoration
    VertDecorationTop(u8),
    VertDecorationMiddle(u8),
    VertDecorationBottom(u8),
    HorzDecorationLeft(u8),
    HorzDecorationMiddle(u8),
    HorzDecorationRight(u8),
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
