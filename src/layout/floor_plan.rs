use super::functional_tiles::UtilityTile;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[allow(unused)]
pub fn rect_room(width: usize, height: usize) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; height]; width];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            grid[x][y] = Some(UtilityTile::Floor);
        }
    }
    grid
}

#[allow(unused)]
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

#[allow(unused)]
pub fn perlin_room(
    width: usize,
    height: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; height]; width];
    let perlin = Perlin::new(rng.random::<u32>());
    let mut count: u32 = 0;

    for x in 0..width {
        let x_ratio = x as f64 / width as f64;
        for y in 0..height {
            let y_ratio = y as f64 / height as f64;
            let val = perlin.get([x_ratio, y_ratio]);
            if val > 0.0 {
                grid[x][y] = Some(UtilityTile::Floor);
                count += 1;
            } else {
                grid[x][y] = None;
            }
        }
    }

    // if less than 50% of the room generated, try again
    let total = width * height;
    if (count as f32 / total as f32) < 0.5 {
        return perlin_room(width, height, rng);
    }
    grid

    // TODO: add constraint for single block missing
    // TODO: implement culling of islands
}
