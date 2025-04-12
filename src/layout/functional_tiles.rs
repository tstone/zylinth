use super::replacement::Replaceable;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
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
    WallTop,
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
