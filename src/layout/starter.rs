use super::functional_tiles::UtilityTile;
use bevy::prelude::*;

pub fn mark_player_start_tile(grid: &mut Vec<Vec<Option<UtilityTile>>>) {
    for x in 0..grid.len() {
        if x > 0 {
            for y in 0..grid[x].len() {
                if y > 0 {
                    let tile = grid[x as usize][y as usize];
                    if tile == Some(UtilityTile::Floor)
                        && grid[x][y as usize - 1] == Some(UtilityTile::Floor)
                        && grid[x as usize - 1][y] == Some(UtilityTile::Floor)
                        && grid[x as usize - 1][y as usize - 1] == Some(UtilityTile::Floor)
                    {
                        debug!("Identified player start {x},{y}");
                        grid[x][y] = Some(UtilityTile::PlayerStart);
                        return;
                    }
                }
            }
        }
    }
}
