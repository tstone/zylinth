#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UtilityTile {
    #[default]
    Test1,
    Test2,
    Test3,
    Empty,
    WallOutlineTop,
    WallOutlineInnerCornerRight,
    WallOutlineInnerCornerLeft,
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
