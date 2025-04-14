use super::functional_tiles::UtilityTile;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::cmp;

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

// IDEA: maybe instead of hard-coded shapes, what if there was an "add one room to the right" or "add one room above/below" method
pub fn perlin_dog_bone(
    width: usize,
    height: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let fortyfive_width = cmp::max(1, (width as f32 * 0.45).floor() as usize);
    debug!("45% width {fortyfive_width}");

    let left = perlin_room(fortyfive_width, height, rng);
    let right = perlin_room(fortyfive_width, height, rng);

    let mut combined: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; height]; width];
    // copy in left room
    for x in 0..fortyfive_width {
        for y in 0..height {
            combined[x][y] = left[x][y];
        }
    }
    // copy in right room
    let rem_width = width - fortyfive_width;
    for x in rem_width..width {
        let offset_x = x - rem_width;
        for y in 0..height {
            combined[x][y] = right[offset_x][y];
        }
    }
    // generate hall
    let hall_x = cmp::max(0, (fortyfive_width as f32 * 0.5).floor() as usize);
    let hall_y = cmp::max(0, (height as f32 * 0.25).floor() as usize);
    let hall_height = cmp::max(3, (height as f32 * 0.4).floor() as usize);
    let hall_width = hall_x * 3;
    debug!("hall: x {hall_x},  y {hall_y}, width {hall_width}, height {hall_height}");

    for x in hall_x..(hall_width + hall_x) {
        for y in hall_y..(hall_height + hall_y) {
            combined[x][y] = Some(UtilityTile::Floor);
        }
    }

    combined
}
