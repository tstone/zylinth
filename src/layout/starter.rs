use super::functional_tiles::UtilityTile;
use bevy::prelude::*;

pub fn set_player_inside(grid: &Vec<Vec<Option<UtilityTile>>>, commands: &mut Commands) {
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
                        // TODO: move player to this location
                    }
                }
            }
        }
    }
}
