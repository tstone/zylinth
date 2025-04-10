use std::{collections::HashSet, u32};

use rand::prelude::*;

use crate::costmic_legacy_tiles::{CONSTRAINTS, CosmicLegacyTiles};

pub fn wfc_generate(width: usize, height: usize) -> Vec<Vec<Option<CosmicLegacyTiles>>> {
    let mut grid: Vec<Vec<Option<CosmicLegacyTiles>>> = vec![vec![None; height]; width];
    let mut needs_populated: Vec<(usize, usize)> = Vec::new();
    let mut visited_points: HashSet<(usize, usize)> = HashSet::new();
    let mut rng = rand::rng();

    // let starter_tile = CONSTRAINTS.keys().choose(&mut rng).unwrap().clone();
    // let start_x = rng.random_range(0..width);
    // let start_y = rng.random_range(0..height);
    let starter_tile = CosmicLegacyTiles::Wall;
    let start_x = 0;
    let start_y = 0;
    println!("starting {},{} with {:?}", start_x, start_y, starter_tile);
    visited_points.insert((start_x, start_y));

    grid[start_x][start_y] = Some(starter_tile);
    needs_populated.append(&mut get_neighbors(
        start_x,
        start_y,
        width,
        height,
        &mut visited_points,
    ));

    let mut last: (usize, usize) = (0, 0);
    while needs_populated.len() > 0 {
        // TODO: maybe tie break lowest possibilities based on nearest to last point

        // find the next with the least amount of possibilities
        let mut min = u32::MAX;
        let mut distance = f32::MAX;
        let mut next_index: usize = 0;
        for (index, candidate) in needs_populated.iter().enumerate() {
            let p = get_possibilities_for_square(candidate.0, candidate.1, width, height, &grid);
            if (p.len() as u32) < min {
                min = p.len() as u32;
                next_index = index;
                distance = calc_distance(&last, &candidate);
            } else if (p.len() as u32) == min {
                // check distances
                let candidate_dist = calc_distance(&last, &candidate);
                if candidate_dist < distance {
                    next_index = index;
                    distance = candidate_dist;
                }
            }
        }

        let next = needs_populated.remove(next_index);

        // choose of the potentials
        let possibilities = Vec::from_iter(get_possibilities_for_square(
            next.0, next.1, width, height, &grid,
        ));

        println!(
            "possibilities for {},{}: {:?}",
            next.0, next.1, possibilities
        );

        // try again
        if possibilities.len() == 0 {
            println!("undoing last ({},{})", last.0, last.1);
            visited_points.remove(&last);
            grid[last.0][last.1] = None;
            continue;
        }

        grid[next.0][next.1] = possibilities.choose(&mut rng).map(|t| t.clone());

        // setup for next loop
        let mut neighbors = get_neighbors(next.0, next.1, width, height, &mut visited_points);
        needs_populated.append(&mut neighbors);
        last = next.clone();
    }

    grid
}

fn calc_distance(a: &(usize, usize), b: &(usize, usize)) -> f32 {
    let x = (a.0 as f32 - b.0 as f32).abs();
    let y = (a.1 as f32 - b.1 as f32).abs();
    (x.powf(2.) + y.powf(2.)).sqrt()
}

fn get_neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    visited_points: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut needs_populated: Vec<(usize, usize)> = Vec::new();

    // up
    if y > 0 {
        let point = (x, y - 1);
        if !visited_points.contains(&point) {
            needs_populated.push(point);
        }
        visited_points.insert(point);
    }
    // down
    if y < (height - 1) {
        let point = (x, y + 1);
        if !visited_points.contains(&point) {
            needs_populated.push(point);
        }
        visited_points.insert(point);
    }
    // left
    if x > 0 {
        let point = (x - 1, y);
        if !visited_points.contains(&point) {
            needs_populated.push(point);
        }
        visited_points.insert(point);
    }
    // right
    if x < (width - 1) {
        let point = (x + 1, y);
        if !visited_points.contains(&point) {
            needs_populated.push(point);
        }
        visited_points.insert(point);
    }

    needs_populated
}

fn get_possibilities_for_square(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    grid: &Vec<Vec<Option<CosmicLegacyTiles>>>,
) -> HashSet<CosmicLegacyTiles> {
    let mut possibilities: HashSet<CosmicLegacyTiles> =
        HashSet::from_iter(CONSTRAINTS.keys().map(|t| *t));

    // up
    if y > 0 {
        match grid[x][y - 1] {
            None => {}
            Some(tile) => match CONSTRAINTS.get(&tile) {
                Some(constraints) if constraints.down.len() > 0 => {
                    let set = HashSet::from_iter(constraints.down.clone());
                    possibilities = possibilities
                        .intersection(&set)
                        .into_iter()
                        .map(|t| *t)
                        .collect();
                    println!(
                        "({x},{y}): {:?} @ ({x},{}) up looking down: {:?}",
                        tile,
                        y - 1,
                        possibilities
                    );
                }
                _ => {}
            },
        }
    }
    // down
    if y < (height - 1) {
        match grid[x][y + 1] {
            None => {}
            Some(tile) => match CONSTRAINTS.get(&tile) {
                Some(constraints) if constraints.up.len() > 0 => {
                    let set = HashSet::from_iter(constraints.up.clone());
                    possibilities = possibilities
                        .intersection(&set)
                        .into_iter()
                        .map(|t| *t)
                        .collect();
                    println!(
                        "({x},{y}) {:?} @ ({x},{}) down looking up: {:?}",
                        tile,
                        y + 1,
                        possibilities
                    );
                }
                _ => {}
            },
        }
    }
    // left
    if x > 0 {
        match grid[x - 1][y] {
            None => {}
            Some(tile) => match CONSTRAINTS.get(&tile) {
                Some(constraints) if constraints.right.len() > 0 => {
                    let set = HashSet::from_iter(constraints.right.clone());
                    possibilities = possibilities
                        .intersection(&set)
                        .into_iter()
                        .map(|t| *t)
                        .collect();
                    println!(
                        "({x},{y}) {:?} @ ({},{y}) left looking right: {:?}",
                        tile,
                        x - 1,
                        possibilities
                    );
                }
                _ => {}
            },
        }
    }
    // right
    if x < (width - 1) {
        match grid[x + 1][y] {
            None => {}
            Some(tile) => match CONSTRAINTS.get(&tile) {
                Some(constraints) if constraints.left.len() > 0 => {
                    let set = HashSet::from_iter(constraints.left.clone());
                    possibilities = possibilities
                        .intersection(&set)
                        .into_iter()
                        .map(|t| *t)
                        .collect();
                    println!(
                        "({x},{y}) {:?} @ ({},{y}) right looking left: {:?}",
                        tile,
                        x + 1,
                        possibilities
                    );
                }
                _ => {}
            },
        }
    }

    possibilities
}
