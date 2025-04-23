use super::grid::TileGrid;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct TileContext<'a, T: Clone + PartialEq + Eq> {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub tile: &'a Option<T>,
    pub(crate) grid: &'a TileGrid<T>,
}

#[allow(unused)]
impl<'a, T> TileContext<'a, T>
where
    T: Clone + PartialEq + Eq,
{
    pub fn grid_width(&self) -> usize {
        self.grid.width()
    }

    pub fn grid_height(&self) -> usize {
        self.grid.height()
    }

    pub fn grid_depth(&self) -> usize {
        self.grid.depth()
    }

    pub(crate) fn is_valid(&self, x: i32, y: i32, z: i32) -> bool {
        x >= 0
            && y >= 0
            && z >= 0
            && (x as usize) < self.grid_width()
            && (y as usize) < self.grid_height()
            && (z as usize) < self.grid_depth()
    }

    /// If this tile is actually on the grid or not
    pub fn is_in_bounds(&self) -> bool {
        self.is_valid(self.x, self.y, self.z)
    }

    /// Get context from another position, e.g. 1,-1 returns the bottom right corner
    pub fn get(&self, x_delta: i32, y_delta: i32, z_delta: i32) -> TileContext<'a, T> {
        let new_x = self.x + x_delta;
        let new_y = self.y + y_delta;
        let new_z = self.z + z_delta;

        TileContext {
            x: new_x,
            y: new_y,
            z: new_z,
            tile: if self.is_valid(new_x, new_y, new_z) {
                &self.grid[new_x as usize][new_y as usize][new_z as usize]
            } else {
                &None
            },
            grid: self.grid,
        }
    }

    pub fn up(&self) -> TileContext<'a, T> {
        self.get(0, -1, 0)
    }

    pub fn down(&self) -> TileContext<'a, T> {
        self.get(0, 1, 0)
    }

    pub fn left(&self) -> TileContext<'a, T> {
        self.get(-1, 0, 0)
    }

    pub fn right(&self) -> TileContext<'a, T> {
        self.get(1, 0, 0)
    }

    pub fn top_left(&self) -> TileContext<'a, T> {
        self.get(-1, -1, 0)
    }

    pub fn top_right(&self) -> TileContext<'a, T> {
        self.get(1, -1, 0)
    }

    pub fn bottom_left(&self) -> TileContext<'a, T> {
        self.get(-1, 1, 0)
    }

    pub fn bottom_right(&self) -> TileContext<'a, T> {
        self.get(1, 1, 0)
    }

    pub fn above(&self) -> TileContext<'a, T> {
        self.get(0, 0, 1)
    }

    pub fn below(&self) -> TileContext<'a, T> {
        self.get(0, 0, -1)
    }

    /// Get context at an absolute layer
    pub fn layer(&self, layer: usize) -> TileContext<'a, T> {
        let dist_to_zero = self.grid_depth() as i32 - self.z;
        self.get(0, 0, dist_to_zero + layer as i32)
    }
}

impl<'a, T> PartialEq<T> for TileContext<'a, T>
where
    T: Clone + PartialEq + Eq,
{
    fn eq(&self, other: &T) -> bool {
        *self.tile == Some(other.clone())
    }
}

impl<'a, T> PartialEq<Option<T>> for TileContext<'a, T>
where
    T: Clone + PartialEq + Eq,
{
    fn eq(&self, other: &Option<T>) -> bool {
        self.tile == other
    }
}
