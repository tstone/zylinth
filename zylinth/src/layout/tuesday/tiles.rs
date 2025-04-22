use bevy::ecs::component::Component;
use rand::Rng;
use tilegen::TileGrid;

use crate::layout::functional_tiles::UtilityTile;
use crate::layout::{IsImpassable, TileSprite};

use super::utility_to_tuesday;

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum TuesdayTile {
    // 4 x 11, 32x

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
    WallDoubleLower = 10,

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
    WallDoubleUpper = 21,

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
    WallDouble = 32,

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
    Empty = 43,

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
        *self == Self::WallBottom || *self == Self::WallBottomLeft
    }
}

impl TuesdayTile {
    #[inline]
    pub fn name() -> &'static str {
        NAME
    }

    pub fn layer_to_tile_sprite(
        grid: &TileGrid<UtilityTile>,
        layer: usize,
        rng: &mut impl Rng,
    ) -> Vec<Vec<Option<TileSprite>>> {
        let mut tilesprites: Vec<Vec<Option<TileSprite>>> = vec![vec![]; grid.len()];

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if let Some(utility) = grid[x][y][layer] {
                    tilesprites[x].push(Some(TileSprite {
                        index: utility_to_tuesday(utility, rng).into(),
                        collider: UtilityTile::is_impassable(&utility),
                        role: Some(utility),
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
