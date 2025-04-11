// TODO: generate floors
// Wrap in walls
// then proceed

use super::functional_tiles::UtilityTile;

pub fn rect_room(width: usize, height: usize) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; height]; width];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            grid[x][y] = Some(UtilityTile::Floor);
        }
    }
    grid
}

pub fn l_room(
    width: usize,
    height: usize,
    cut_w: usize,
    cut_h: usize,
) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; height]; width];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if x > cut_w || y > cut_h {
                grid[x][y] = Some(UtilityTile::Floor);
            }
        }
    }
    grid
}
