use tilegen::TileGrid;

use super::tuesday::{TuesdayTile, TuesdayTile::*};

pub fn test_room() -> TileGrid<TuesdayTile> {
    let mut grid = TileGrid::empty(6, 6, 1);
    grid[0][0][0] = None;
    grid[1][0][0] = Some(Floor);
    grid[2][0][0] = Some(Floor);
    grid[3][0][0] = Some(Floor);
    grid[4][0][0] = Some(Floor);
    grid[5][0][0] = None;

    grid[0][1][0] = None;
    grid[1][1][0] = Some(Floor);
    grid[2][1][0] = Some(Floor);
    grid[3][1][0] = Some(Floor);
    grid[4][1][0] = Some(Floor);
    grid[5][1][0] = None;

    grid[0][2][0] = None;
    grid[1][2][0] = None;
    grid[2][2][0] = Some(Floor);
    grid[3][2][0] = Some(Floor);
    grid[4][2][0] = None;
    grid[5][2][0] = None;

    grid[0][3][0] = Some(Floor);
    grid[1][3][0] = None;
    grid[2][3][0] = Some(Floor);
    grid[3][3][0] = None;
    grid[4][3][0] = None;
    grid[5][3][0] = Some(Floor);

    grid[0][4][0] = Some(Floor);
    grid[1][4][0] = Some(Floor);
    grid[2][4][0] = Some(Floor);
    grid[3][4][0] = Some(Floor);
    grid[4][4][0] = Some(Floor);
    grid[5][4][0] = Some(Floor);

    grid[0][5][0] = Some(Floor);
    grid[1][5][0] = None;
    grid[2][5][0] = Some(Floor);
    grid[3][5][0] = None;
    grid[4][5][0] = None;
    grid[5][5][0] = Some(Floor);

    grid
}
