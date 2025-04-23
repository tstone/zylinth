use bevy::ecs::component::Component;

use super::IsImpassable;

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UtilityTile {
    #[default]
    Test,
    Wall,
    WallLeft,
    WallRight,
    WallBorderInnerCornerTopLeft,
    WallBorderInnerCornerTopRight,
    WallBorderInnerCornerBottomLeft,
    WallBorderInnerCornerBottomRight,
    WallBorderTop,
    WallBorderLeft,
    WallBorderRight,
    WallBorderBottom,
    WallBorderTopLeft,
    WallBorderTopRight,
    WallBorderBottomRight,
    WallBorderBottomLeft,
    WallBorderDoubleLeftCorner,
    WallBorderDoubleRightCorner,
    WallBorderDoubleHorizontal,
    WallBorderDoubleVertical,
    WallBorderAllCorner,
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
        *self == Self::WallBorderInnerCornerTopLeft
            || *self == Self::WallBorderInnerCornerTopRight
            || *self == Self::WallBorderInnerCornerBottomLeft
            || *self == Self::WallBorderInnerCornerBottomRight
            || *self == Self::Wall
            || *self == Self::WallLeft
            || *self == Self::WallRight
            || *self == Self::WallBorderLeft
            || *self == Self::WallBorderRight
            || *self == Self::WallBorderBottom
            || *self == Self::WallBorderTopLeft
            || *self == Self::WallBorderTopRight
            || *self == Self::WallBorderBottomRight
            || *self == Self::WallBorderBottomLeft
    }
}
