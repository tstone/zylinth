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
    padding: u8,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let total_width = maze.width * room_width;
    let total_height = maze.height * room_height;
    let mut grid: Vec<Vec<Option<UtilityTile>>> =
        vec![vec![None; total_height as usize]; total_width as usize];
    let mut rooms: Vec<Vec<RoomDefinition>> =
        vec![vec![RoomDefinition::default(); maze.height as usize]; maze.width as usize];

    let room_width = room_width + padding as u16;
    let room_height = room_height + padding as u16;

    // start with a grid of rooms
    for maze_x in 0..maze.width {
        let x_offset = (maze_x * room_width) as usize;
        let lesser_room_width = room_width as usize - 2 - padding as usize;

        for maze_y in 0..maze.height {
            let y_offset = (maze_y * room_height) as usize;
            let leser_room_height = room_height as usize - 4 - padding as usize;
            let rnd_width = ((lesser_room_width - 3)..=lesser_room_width)
                .choose(rng)
                .unwrap();
            let rnd_height = ((leser_room_height - 3)..=leser_room_height)
                .choose(rng)
                .unwrap();
            let room = rect_room(rnd_width, rnd_height);

            // TODO: Add random cuts

            let x_rnd_offset: i32 = (-2..2).choose(rng).unwrap();
            let y_rnd_offset: i32 = (-1..3).choose(rng).unwrap();
            let x_offset = cmp::max(0, x_offset as i32 + x_rnd_offset) as usize;
            let y_offset = cmp::max(0, y_offset as i32 + y_rnd_offset) as usize;

            // save actual width/height to use later
            rooms[maze_x as usize][maze_y as usize] = RoomDefinition {
                width: rnd_width,
                height: rnd_height,
                center: (x_offset + (rnd_width / 2), y_offset + (rnd_height / 2)),
            };

            // copy room into grid
            for x in 0..rnd_width {
                let rm_x = x_offset + x;
                for y in 0..rnd_height {
                    let rm_y = y_offset + y;
                    if rm_x < total_width as usize && rm_y < total_height as usize {
                        grid[rm_x][rm_y] = room[x][y];
                    }
                }
            }
        }
    }

    // add hallways
    // TODO: insert hallways

    grid
}

fn define_hall(
    from_index: &u32,
    to_index: &u32,
    maze: &Maze,
    rooms: &Vec<Vec<RoomDefinition>>,
    rng: &mut ChaCha8Rng,
) -> HallDefinition {
    let from = maze.node_to_grid_coords(*from_index);
    let to = maze.node_to_grid_coords(*to_index);
    let from_room_def = &rooms[from.0 as usize][from.1 as usize];
    let to_room_def = &rooms[to.0 as usize][to.1 as usize];

    let max_room_width = cmp::max(from_room_def.width, to_room_def.width);
    let max_room_height = cmp::max(from_room_def.height, to_room_def.height);
    let half_width = max_room_width as usize / 2;
    let half_height = max_room_height as usize / 2;
    trace!("half width {half_width}, half height {half_height}");
    trace!(
        "from grid {:?}, from center {:?} / to grid {:?}, to center {:?}, ",
        from, from_room_def.center, to, to_room_def.center
    );

    let mut def = HallDefinition::default();

    if from.0 == to.0 {
        // hallway is horizontal
        def.width = cmp::min(*vec![3, 4, 5].choose(rng).unwrap() as usize, half_width);
        def.height = ((max_room_height as f32) * 0.66) as usize;

        // TODO: pivot the Y coord instead of the X coord

        if from_room_def.center.0 < to_room_def.center.0 {
            // hallway is left to right
            def.start_x = from_room_def.center.0.saturating_sub(def.width / 2)
        } else {
            // hallway is right to left
            def.start_x = to_room_def.center.0.saturating_sub(def.width / 2)
        };
    } else {
        // hallway is vertical
        def.width = ((max_room_width as f32) * 0.66) as usize;
        def.height = cmp::min(*vec![3, 4, 5].choose(rng).unwrap() as usize, half_width);

        if from_room_def.center.1 < to_room_def.center.1 {
            // hallway is up to down
            def.start_y = from_room_def.center.1.saturating_sub(def.height / 2)
        } else {
            // hallway is down to up
            def.start_y = to_room_def.center.1.saturating_sub(def.height / 2)
        };
    }

    // randomly offset the hallway
    // if hall_x > 1 {
    //     hall_x = (hall_x as i32 + (-1..=1).choose(rng).unwrap())
    //         .try_into()
    //         .unwrap_or(0);
    // }
    // if hall_x > 1 {
    //     hall_y = (hall_y as i32 + (-1..=1).choose(rng).unwrap())
    //         .try_into()
    //         .unwrap_or(0);
    // }

    def
}

#[derive(Debug, Default, Clone)]
struct RoomDefinition {
    width: usize,
    height: usize,
    center: (usize, usize),
}

#[derive(Debug, Default, Clone)]
struct HallDefinition {
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
}
