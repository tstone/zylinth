#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UtilityTile {
    #[default]
    Test1,
    Test2,
    Test3,
    Empty,
    WallOutlineTop,
    WallOutlineOuterLeft,
    WallOutlineOuterRight,
    WallOutlineTopLeft,
    WallOutlineTopRight,
    WallTop,
    WallLeft,
    WallRight,
    Floor,
    FloorShadowLeft,
    FloorShadowTop,
    FloorShadowLeftTransition,
    FloorShadowTopTransition,
    FloorShadowOuterCorner,
    FloorShadowInnerCorner,
}
