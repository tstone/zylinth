use super::functional_tiles::UtilityTile;
use bevy::prelude::*;

pub fn find_player_start_tile(grid: &Vec<Vec<Option<UtilityTile>>>) -> Option<(usize, usize)> {
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
                        return Some((x, y));
                    }
                }
            }
        }
    }
    None
}
