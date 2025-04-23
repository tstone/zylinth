use std::fmt::Debug;
use std::ops::Deref;

use rand::rand_core::le;

use crate::TilePoint;

use super::dir::TileDir;
use super::line::TileLine;
use super::rect::TileRect;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TileRegion<T: Default + Clone + Debug + PartialEq + Eq> {
    pub rect: TileRect,
    pub region_type: T,
    pub exits: TileExits,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TileExits {
    left: Vec<TileLine>,
    right: Vec<TileLine>,
    up: Vec<TileLine>,
    down: Vec<TileLine>,
}

#[allow(unused)]
impl<T> TileRegion<T>
where
    T: Clone + Default + Debug + PartialEq + Eq,
{
    pub fn new(region_type: T, rect: TileRect) -> TileRegion<T> {
        TileRegion {
            region_type,
            rect,
            ..Default::default()
        }
    }

    /// Create a new region based on an exit. Will return None if out of range (0)
    pub fn from_exit(dir: TileDir, exit: TileLine, length: usize, region_type: T) -> Option<Self> {
        match dir {
            TileDir::Left => {
                let x0 = exit.from.x.saturating_sub(length);
                let x1 = exit.to.x;
                if x1 - x0 == length {
                    Some(TileRegion {
                        rect: TileRect::new(x0, exit.from.y, x1, exit.to.y),
                        region_type,
                        ..Default::default()
                    })
                } else {
                    None
                }
            }
            TileDir::Right => Some(TileRegion {
                rect: TileRect::new(exit.from.x, exit.from.y, exit.to.x + length, exit.to.y),
                region_type,
                ..Default::default()
            }),
            TileDir::Up => {
                let y0 = exit.from.y.saturating_sub(length);
                let y1 = exit.to.y;
                if y1 - y0 == length {
                    Some(TileRegion {
                        rect: TileRect::new(exit.from.x, y0, exit.to.x, y1),
                        region_type,
                        ..Default::default()
                    })
                } else {
                    None
                }
            }
            TileDir::Down => Some(TileRegion {
                rect: TileRect::new(exit.from.x, exit.from.y, exit.to.x, exit.to.y + length),
                region_type,
                ..Default::default()
            }),
        }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        let width = self.width();
        let height = self.height();
        self.rect.min = TilePoint::new(x, y);
        self.rect.max = TilePoint::new(x + width, y + height);
    }

    pub fn move_all(regions: &mut Vec<TileRegion<T>>, delta_x: i32, delta_y: i32) {
        // TODO
    }

    /// Add an exit line for the entire length of a direction
    pub fn add_full_length_exit(&mut self, dir: TileDir) {
        match dir {
            TileDir::Left => self.exits.left.push(TileLine {
                from: self.top_left(),
                to: self.bottom_left(),
            }),
            TileDir::Right => self.exits.right.push(TileLine {
                from: self.top_right(),
                to: self.bottom_right(),
            }),
            TileDir::Up => self.exits.up.push(TileLine {
                from: self.top_left(),
                to: self.top_right(),
            }),
            TileDir::Down => self.exits.down.push(TileLine {
                from: self.bottom_left(),
                to: self.bottom_right(),
            }),
        }
    }

    /// Add all edges as exits
    pub fn add_all_exits(&mut self) {
        TileDir::vec().iter().map(|d| {
            self.add_full_length_exit(*d);
        });
    }

    // TODO:

    /// Checks if the exit is within the bounds of this region AND that it does not overlap existing exist
    // pub fn is_exit_allowed(&self, dir: TileDir, exit: TileLine) -> bool {
    //     let in_bounds = match dir {
    //         TileDir::Left => {
    //             let top_left = self.top_left();
    //             exit.from.y == exit.to.y
    //                 && exit.from.y == top_left.y
    //                 && top_left.x <= exit.from.x
    //                 && self.top_right().x >= exit.to.x
    //         } // TODO: other directions
    //     };

    //     if !in_bounds {
    //         return false;
    //     }

    //     let existing = false;
    //     // TODO: does it overlap with existing
    // }

    /// Returns the remaining space on which an exit could be
    // pub fn get_remaining_possible_exits(&self, dir: TileDir) -> Vec<TileLine> {
    //     // TODO
    // }

    // TODO: might not need this if the ingridents are methods on their own
    /// Add a random exit to the given side
    pub fn add_rnd_exit(&mut self, dir: TileDir, min_length: usize, max_length: usize) {
        let edge_len = match dir {
            TileDir::Left | TileDir::Right => self.height(),
            TileDir::Up | TileDir::Down => self.width(),
        };
        // TODO: also need to exclude exits that already exist
        let len = edge_len.saturating_sub(min_length);
    }
}

impl<T> Deref for TileRegion<T>
where
    T: Clone + Debug + Default + PartialEq + Eq,
{
    type Target = TileRect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}
