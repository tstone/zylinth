use bevy::prelude::*;
use tilegen::TileGrid;

use crate::map::{IsImpassable, TileRole, TileSprite};

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
#[repr(u32)]
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
    Resoursce1 = 10,

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
    DoorFrame(u8) = 35,
    PanelDisabled = 36,
    PanelEnabled = 37,
    WallPanelLeft = 38,
    WallPanelMiddle = 39,
    WallPanelRight = 40,
    SwitchLeft(u8) = 41,
    SwitchRight(u8) = 42,
    #[default]
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
    Transparent = 54,

    PlayerStart(u8) = 999,
}

impl Into<u32> for TuesdayTile {
    fn into(self) -> u32 {
        // https://doc.rust-lang.org/reference/items/enumerations.html#casting
        unsafe { *(&self as *const Self as *const u32) }
    }
}

impl Into<usize> for TuesdayTile {
    fn into(self) -> usize {
        let you32: u32 = self.into();
        you32 as usize
    }
}

impl PartialEq<usize> for TuesdayTile {
    fn eq(&self, other: &usize) -> bool {
        let tile_index: usize = (*self).into();
        &tile_index == other
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
                    let role = match tile {
                        Self::DoorFrame(id) => Some(TileRole::Door(id)),
                        Self::SwitchLeft(id) => Some(TileRole::Switch(id, false)),
                        Self::SwitchRight(id) => Some(TileRole::Switch(id, true)),
                        Self::PlayerStart(id) => Some(TileRole::PlayerStart(id)),
                        _ => None,
                    };
                    let index: usize = match tile {
                        Self::PlayerStart(_) => Self::Transparent.into(),
                        Self::SwitchLeft(_) => 12,
                        t => t.into(),
                    };

                    tilesprites[x].push(Some(TileSprite {
                        index,
                        collider: Self::is_impassable(&tile),
                        role,
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
