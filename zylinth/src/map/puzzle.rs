use tilegen::TileGrid;

use crate::defs::ControlLink;

pub struct Puzzle<T: Clone + PartialEq + Eq> {
    pub grid: TileGrid<T>,
    pub starting_links: Vec<ControlLink>,
}
