use std::fmt::Debug;
use std::ops::Deref;

use super::dir::TileDir;
use super::line::TileLine;
use super::rect::TileRect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileRegion<T: Clone + Debug + PartialEq + Eq> {
    pub rect: TileRect,
    pub region_type: T,
    pub exits: Vec<TileLine>,
}

#[allow(unused)]
impl<T> TileRegion<T>
where
    T: Clone + Debug + PartialEq + Eq,
{
    pub fn new(region_type: T, rect: TileRect, exits: Vec<TileLine>) -> TileRegion<T> {
        TileRegion {
            region_type,
            rect,
            exits,
        }
    }

    /// Add an exit line for the entire length of a direction
    pub fn add_full_length_exit(&mut self, dir: TileDir) {
        match dir {
            TileDir::Left => self.exits.push(TileLine {
                a: self.top_left(),
                b: self.bottom_left(),
            }),
            TileDir::Right => self.exits.push(TileLine {
                a: self.top_right(),
                b: self.bottom_right(),
            }),
            TileDir::Up => self.exits.push(TileLine {
                a: self.top_left(),
                b: self.top_right(),
            }),
            TileDir::Down => self.exits.push(TileLine {
                a: self.bottom_left(),
                b: self.bottom_right(),
            }),
        }
    }

    /// Add all edges as exits
    pub fn add_all_exits(&mut self) {
        TileDir::vec().iter().map(|d| {
            self.add_full_length_exit(*d);
        });
    }
}

impl<T> Deref for TileRegion<T>
where
    T: Clone + Debug + PartialEq + Eq,
{
    type Target = TileRect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}
