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
    wander_factor: f32,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Option<UtilityTile>>> {
    let mut grid: Vec<Vec<Option<UtilityTile>>> = vec![vec![None; total_height]; total_width];
    let bounding = TileRect::new(0, 0, total_width, total_height);
    let origin = TilePoint::new(0, 0);

    // starting region is in the top left-ish of the grid
    let mut previous_region = TileRect::new(0, 0, total_width / 8, total_height / 8);
    let mut dir: TileDir = TileDir::rnd(rng);
    let mut density: f32 = 0.0;

    // IDEA: Can walls or empty regions be inserted into huge open rooms?

    while density < target_density {
        // TODO: there seems to be a bug where sometimes these do not overlap
        debug!(
            "previous region w: {}, h: {} -- {:?}",
            previous_region.width(),
            previous_region.height(),
            previous_region
        );
        let starting_region = match dir {
            TileDir::Left => previous_region.get_left_slice(previous_region.width() / 2),
            TileDir::Right => previous_region.get_right_slice(previous_region.width() / 2),
            TileDir::Up => previous_region.get_top_slice(previous_region.height() / 2),
            TileDir::Down => previous_region.get_bottom_slice(previous_region.height() / 2),
        };

        let room = Room::gen_rect(&starting_region, 6, 14, 6, 10, rng);
        debug!("Generated a room {:?}", room.rect());
        room.copy_grid_into(&mut grid);

        let room_rect = room.rect().clamp(&bounding);

        // Handle direction changes
        if (room_rect.min.x == 0 && dir == TileDir::Left)
            || (room_rect.min.y == 0 && dir == TileDir::Up)
            || (room_rect.max.x > total_width && dir == TileDir::Right)
            || (room_rect.max.y > total_height && dir == TileDir::Down)
            || rng.random_bool(wander_factor as f64)
        {
            let mut allowed_dirs: Vec<TileDir> = Vec::new();
            if room_rect.min.x != 0 && dir != TileDir::Left {
                allowed_dirs.push(TileDir::Left);
            }
            if room_rect.min.y != 0 && dir != TileDir::Up {
                allowed_dirs.push(TileDir::Up);
            }
            if !(room_rect.max.x >= total_width) && dir != TileDir::Right {
                allowed_dirs.push(TileDir::Right);
            }
            if !(room_rect.max.y >= total_height) && dir != TileDir::Down {
                allowed_dirs.push(TileDir::Down);
            }

            let mut distances = allowed_dirs
                .iter()
                .map(|dir| match *dir {
                    TileDir::Left => (*dir, room_rect.left_center().distance(&origin)),
                    TileDir::Right => (*dir, room_rect.right_center().distance(&origin)),
                    TileDir::Up => (*dir, room_rect.top_center().distance(&origin)),
                    TileDir::Down => (*dir, room_rect.bottom_center().distance(&origin)),
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

            dir = TileDir::rnd_weighted(&weights, rng);
            debug!("Changing direction to {:?}", dir);
        }

        // Update for next run
        previous_region = room_rect.clone();
        density = measure_density(&grid);
    }

    grid
}
