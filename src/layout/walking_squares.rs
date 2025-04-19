use std::collections::HashMap;

use super::functional_tiles::UtilityTile;
use super::room::*;
use bevy::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub fn walking_squares(
    total_width: usize,
    total_height: usize,
    target_density: f32,
    branch_factor: f32,
    wander_factor: f32,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; total_height]; total_width];
    let bounding = TileRect::new(0, 0, total_width, total_height);
    let origin_region = bounding
        .get_inner_slice(9, 9)
        .unwrap()
        .rnd_bounded_slice(6, 6, rng)
        .unwrap();

    // starting region is in the top left-ish of the grid
    let mut previous_region = origin_region.clone();
    let mut lookback_region = origin_region.clone();
    let mut dir: TileDir = TileDir::rnd(rng);
    let mut density: f32 = 0.0;

    // IDEA: Can walls or empty regions be inserted into huge open rooms?
    let mut attempts = 0;
    let mut return_to_origin = 0;
    let mut return_to_lookback = 0;
    let mut steps_since_last_dir_change = 0;

    while density < target_density && attempts < 5000 {
        let starting_region = previous_region.get_inner_slice(3, 3);

        // if the starting region is unrealistic for placement, just return to origin
        if starting_region.is_none() {
            trace!("Returning to origin");
            return_to_origin += 1;
            previous_region = origin_region.clone();
            continue;
        }

        let starting_region = starting_region.unwrap();
        trace!(
            "starting region w: {}, h: {} -- {:?}",
            starting_region.width(),
            starting_region.height(),
            starting_region
        );

        let room = Room::gen_rect(&dir, &previous_region, 8, 14, 8, 10, rng);
        let mut change_dir = false;
        match room.intersect(&bounding) {
            None => change_dir = true,
            Some(room_rect) => {
                // Make sure the resulting room is actually large enough to be useful
                if room_rect.width() >= 3 && room_rect.height() >= 3 {
                    trace!("Generated a room {:?}", room_rect);
                    room.copy_grid_into(&mut grid);
                    previous_region = room_rect.clone();
                    density = measure_density(&grid);
                    steps_since_last_dir_change += 1;

                    // Wander about
                    if steps_since_last_dir_change > 3 && rng.random_bool(wander_factor as f64) {
                        return_to_lookback += 1;
                        previous_region = lookback_region.clone();
                        change_dir = true;
                    } else if rng.random_bool(branch_factor as f64) {
                        lookback_region = room_rect.clone();
                    }
                } else {
                    trace!(
                        "Failed to generate a room {:?}. Changing directions",
                        room_rect
                    );
                    change_dir = true;
                }
            }
        }

        if change_dir {
            dir = TileDir::rnd_without(&dir, rng);
            steps_since_last_dir_change = 0;
            trace!("Changing direction to {:?}", dir);
        }

        attempts += 1;
    }

    trace!(
        "Generated walking squares. Attempts: {attempts}, Return to Lookback: {return_to_lookback}, Return to Origin: {return_to_origin}"
    );

    grid
}

fn get_weighted_directions(room_rect: &TileRect, origin: &TilePoint) -> HashMap<TileDir, f32> {
    let mut distances = TileDir::vec()
        .iter()
        .map(|dir| match *dir {
            TileDir::Left => (*dir, room_rect.left_center().distance(origin)),
            TileDir::Right => (*dir, room_rect.right_center().distance(origin)),
            TileDir::Up => (*dir, room_rect.top_center().distance(origin)),
            TileDir::Down => (*dir, room_rect.bottom_center().distance(origin)),
        })
        .collect::<Vec<_>>();

    // sort by nearest to farthest
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    // order by farthest to nearest
    let rev_distances = distances.iter().rev().collect::<Vec<_>>();

    // TODO: maybe instead of weighting by distance, it could instead have equal weights if the
    // point is far enough from the origin
    let mut weight_budget = 1.0;
    let mut weights: HashMap<TileDir, f32> = HashMap::new();
    for (dir, _) in rev_distances {
        weights.insert(*dir, weight_budget / 2.0);
        weight_budget /= 2.0;
    }

    weights
}
