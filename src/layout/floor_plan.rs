use super::functional_tiles::UtilityTile;
use super::maze::Maze;
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

pub fn from_maze(
    maze: &Maze,
    room_width: u16,
    room_height: u16,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let total_width = maze.width * room_width;
    let total_height = maze.height * room_height;
    let mut grid: Vec<Vec<Option<UtilityTile>>> =
        vec![vec![None; total_height as usize]; total_width as usize];

    // start with a grid of rooms
    for maze_x in 0..maze.width {
        let x_offset = (maze_x * room_width) as usize;
        let lesser_room_width = room_width as usize - 2;

        for maze_y in 0..maze.height {
            let y_offset = (maze_y * room_height) as usize;
            let leser_room_height = room_height as usize - 4;
            let room = if rng.random_bool(0.5) {
                perlin_room(lesser_room_width, leser_room_height, rng)
            } else {
                rect_room(lesser_room_width, leser_room_height)
            };

            // copy room into grid
            for x in 0..lesser_room_width {
                for y in 0..leser_room_height {
                    grid[x + x_offset][y + y_offset] = room[x][y];
                }
            }
        }
    }

    // add hallways
    for (from_id, to_id) in maze.edges.iter() {
        let from = maze.node_to_grid_coords(*from_id);
        let to = maze.node_to_grid_coords(*to_id);
        let half_room = (room_width / 2) as u32;
        let from_center = (
            (from.0 * room_width as u32 + half_room) as usize,
            (from.1 * room_height as u32 + half_room) as usize,
        );
        let to_center = (
            (to.0 * room_width as u32 + half_room) as usize,
            (to.1 * room_height as u32 + half_room) as usize,
        );

        println!("from {:?} to {:?}", from_center, to_center);

        let hall_x = if from_center.0 < to_center.0 {
            from_center.0 - 1
        } else {
            to_center.0 - 1
        };
        let hall_y = if from_center.1 < to_center.1 {
            from_center.1 - 1
        } else {
            to_center.1 - 1
        };
        let hall_width = if from_center.0 == to_center.0 {
            *vec![3, 4, 5].choose(rng).unwrap() as usize
        } else {
            room_width as usize
        };
        let hall_height = if from_center.1 == to_center.1 {
            *vec![3, 4, 5].choose(rng).unwrap() as usize
        } else {
            room_height as usize
        };

        for x in hall_x..(hall_x + hall_width) {
            for y in hall_y..(hall_y + hall_height) {
                grid[x][y] = Some(UtilityTile::Floor);
            }
        }
    }

    grid
}
