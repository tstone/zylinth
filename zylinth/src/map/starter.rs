use super::tuesday::TuesdayTile;
use bevy::prelude::*;
use tilegen::TileGrid;

pub fn mark_player_start_tile(grid: &mut TileGrid<TuesdayTile>) {
    for x in 0..grid.len() {
        if x > 0 {
            for y in 0..grid[x].len() {
                if y > 0 {
                    // TODO: this should use the replacer system somehow
                    let tile = grid[x as usize][y as usize][0];
                    if tile == Some(TuesdayTile::Floor)
                        && grid[x][y as usize - 1][0] == Some(TuesdayTile::Floor)
                        && grid[x as usize - 1][y][0] == Some(TuesdayTile::Floor)
                        && grid[x as usize - 1][y as usize - 1][0] == Some(TuesdayTile::Floor)
                    {
                        // TODO: find more potential starts
                        let z = grid.depth() - 1;
                        grid[x][y][z] = Some(TuesdayTile::PlayerStart(1));
                        return;
                    }
                }
            }
        }
    }
}
