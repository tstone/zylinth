use bevy::ecs::component::Component;
use rand::Rng;
use tilegen::TileGrid;

use crate::layout::functional_tiles::UtilityTile;
use crate::layout::{IsImpassable, TileSprite};

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum TuesdayTile {
    // row 1
    WallTopLeftCaution = 0,
    WallTopCaution = 1,
    WallTopRightCaution = 2,
    WallTopLeft = 3,
    WallTop = 4,
    WallTopRight = 5,
    FloorAlt1 = 6,
    FloorAlt2 = 7,
    WallInnerCornerBottomRight = 8,
    WallInnerCornerBottomLeft = 9,

    // row 2
    WallLeftCaution = 11,
    Floor = 12,
    WallRightCaution = 13,
    WallLeft = 14,
    FloorAlt3 = 15,
    WallRight = 16,
    WallPanelMiddleAlt1 = 17,
    WallPanelMiddleALt2 = 18,
    WallInnerCornerTopLeft = 19,
    WallInnerCornerTopRight = 20,

    // row 3
    WallBottomLeftCaution = 22,
    WallBottomCaution = 23,
    WallBottomRightCaution = 24,
    WallBottomLeft = 25,
    WallBottom = 26,
    WallBottomRight = 27,
    WallInnerCornerBottomRightCaution = 28,
    WallInnerCornerBottomLeftCaution = 29,
    WallInnerCornerTopLeftCaution = 30,
    WallInnerCornerTopRightCaution = 31,

    // row 4
    EmptyDecoration1 = 33,
    EmptyDecoration2 = 34,
    DoorFrame = 35,
    PanelDisabled = 36,
    PanelEnabled = 37,
    WallPanelLeft = 38,
    WallPanelMiddle = 39,
    WallPanelRight = 40,
    SwitchLeft = 41,
    SwitchRight = 42,
    Test = 43,

    // row 5
    WallDoubleLeftCorner = 44,
    WallDoubleRightCorner = 45,
    WallAllCorner = 46,
    WallDoubleHorizontal = 47,
    WallDoubleVertical = 48,
    WallPanelSingle = 49,
    WallDoubleCornerTop = 50,
    WallDoubleCornerBottom = 51,
    WallDoubleLower = 52,
    WallDoubleUpper = 53,

    // TODO: this doesn't make sense
    #[default]
    PlayerStart = 999,
}

impl Into<u32> for TuesdayTile {
    fn into(self) -> u32 {
        self as u32
    }
}

impl Into<usize> for TuesdayTile {
    fn into(self) -> usize {
        self as usize
    }
}

impl IsImpassable for TuesdayTile {
    fn is_impassable(&self) -> bool {
        *self == Self::WallBottom
            || *self == Self::WallBottomLeft
            || *self == Self::WallBottomRight
            || *self == Self::WallDoubleHorizontal
            || *self == Self::WallDoubleVertical
            || *self == Self::WallAllCorner
            || *self == Self::WallDoubleRightCorner
            || *self == Self::WallDoubleLeftCorner
            || *self == Self::WallDoubleLower
            || *self == Self::WallDoubleUpper
            || *self == Self::WallDoubleVertical
            || *self == Self::WallDoubleHorizontal
            || *self == Self::WallInnerCornerBottomLeft
            || *self == Self::WallInnerCornerBottomRight
            || *self == Self::WallInnerCornerTopLeft
            || *self == Self::WallInnerCornerTopRight
            || *self == Self::WallLeft
            || *self == Self::WallPanelLeft
            || *self == Self::WallPanelMiddle
            || *self == Self::WallPanelMiddleALt2
            || *self == Self::WallPanelMiddleAlt1
            || *self == Self::WallPanelRight
            || *self == Self::WallPanelSingle
            || *self == Self::WallRight
            || *self == Self::WallTop
            || *self == Self::WallTopLeft
            || *self == Self::WallTopRight
    }
}

impl TuesdayTile {
    #[inline]
    pub fn name() -> &'static str {
        NAME
    }

    pub fn layer_to_tile_sprites(
        grid: &TileGrid<Self>,
        layer: usize,
    ) -> Vec<Vec<Option<TileSprite>>> {
        let mut tilesprites: Vec<Vec<Option<TileSprite>>> = vec![vec![]; grid.len()];

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if let Some(tile) = grid[x][y][layer] {
                    tilesprites[x].push(Some(TileSprite {
                        index: tile.into(),
                        collider: Self::is_impassable(&tile),
                        role: None,
                    }));
                } else {
                    tilesprites[x].push(None);
                }
            }
        }

        tilesprites
    }
}

const NAME: &'static str = "tuesday";
