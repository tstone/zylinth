use tilegen::TileGrid;

use crate::map::puzzle::{DoorControl, Puzzle};
use crate::map::tuesday::{TuesdayTile, TuesdayTile::*};

pub fn starter_room() -> Puzzle<TuesdayTile> {
    let mut grid = TileGrid::empty(11, 6, 1);
    grid.add_layer();

    grid[0][0][0] = Some(Floor);
    grid[1][0][0] = Some(Floor);
    grid[2][0][0] = None;
    grid[3][0][0] = Some(Floor);
    grid[4][0][0] = Some(Floor);
    grid[5][0][0] = Some(Floor);
    grid[6][0][0] = Some(Floor);
    grid[7][0][0] = Some(Floor);
    grid[7][0][1] = Some(Resoursce1);
    grid[8][0][0] = None;
    grid[9][0][0] = Some(Floor);
    grid[10][0][0] = Some(Floor);

    grid[0][1][0] = Some(Floor);
    grid[1][1][0] = Some(Floor);
    grid[2][1][0] = None;
    grid[3][1][0] = Some(Floor);
    grid[4][1][0] = Some(Floor);
    grid[5][1][0] = Some(Floor);
    grid[6][1][0] = Some(Floor);
    grid[7][1][0] = Some(Floor);
    grid[8][1][0] = None;
    grid[9][1][0] = Some(Floor);
    grid[10][1][0] = Some(Floor);

    grid[0][2][0] = Some(Floor);
    grid[1][2][0] = Some(Floor);
    grid[2][2][0] = None;
    grid[3][2][0] = None;
    grid[4][2][0] = None;
    grid[5][2][0] = Some(Floor);
    grid[5][2][1] = Some(DoorFrame(1));
    grid[6][2][1] = Some(PanelDisabled(1));
    grid[6][2][0] = None;
    grid[7][2][0] = None;
    grid[8][2][0] = None;
    grid[9][2][0] = Some(Floor);
    grid[10][2][0] = Some(Floor);

    grid[0][3][0] = Some(Floor);
    grid[1][3][0] = Some(Floor);
    grid[2][3][0] = Some(Floor);
    grid[3][3][0] = Some(Floor);
    grid[4][3][0] = Some(Floor);
    grid[5][3][0] = Some(Floor);
    grid[6][3][0] = Some(Floor);
    grid[7][3][0] = Some(Floor);
    grid[8][3][0] = Some(Floor);
    grid[9][3][0] = Some(Floor);
    grid[10][3][0] = Some(Floor);

    grid[0][4][0] = Some(Floor);
    grid[1][4][0] = Some(Floor);
    grid[2][4][0] = Some(Floor);
    grid[3][4][0] = Some(Floor);
    grid[3][4][1] = Some(SwitchLeft(1));
    grid[4][4][0] = Some(Floor);
    grid[5][4][0] = Some(Floor);
    grid[6][4][0] = Some(Floor);
    grid[7][4][0] = Some(Floor);
    grid[8][4][0] = Some(Floor);
    grid[9][4][0] = Some(Floor);
    grid[10][4][0] = Some(Floor);

    grid[0][5][0] = Some(Floor);
    grid[1][5][0] = Some(Floor);
    grid[2][5][0] = Some(Floor);
    grid[3][5][0] = Some(Floor);
    grid[4][5][0] = Some(Floor);
    grid[5][5][0] = Some(Floor);
    grid[6][5][0] = Some(Floor);
    grid[7][5][0] = Some(Floor);
    grid[8][5][0] = Some(Floor);
    grid[9][5][0] = Some(Floor);
    grid[10][5][0] = Some(Floor);

    Puzzle {
        grid,
        door_controls: vec![DoorControl {
            doors: vec![1],
            switches: vec![1],
        }],
    }
}
