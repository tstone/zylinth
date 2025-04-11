use super::replacement::Replaceable;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UtilityTile {
    #[default]
    Test1,
    Test2,
    Test3,
    Empty,
    WallOutlineTop,
    WallOutlineInnerCornerTopLeft,
    WallOutlineInnerCornerTopRight,
    WallOutlineInnerCornerBottomLeft,
    WallOutlineInnerCornerBottomRight,
    WallOutlineTopLeft,
    WallOutlineTopRight,
    WallTop,
    WallLeft,
    WallRight,
    WallBottom,
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
