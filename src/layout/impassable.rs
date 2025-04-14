use bevy::prelude::*;

pub trait IsImpassable {
    fn is_impassable(&self) -> bool;
}

#[derive(Component)]
pub struct Impassable;

pub fn to_impassable<T: IsImpassable + Clone>(
    grid: Vec<Vec<Option<T>>>,
) -> Vec<Vec<Option<(T, bool)>>> {
    let width = grid.len();
    let mut output: Vec<Vec<Option<(T, bool)>>> = vec![vec![]; width];

    for x in 0..width {
        for y in 0..grid[x].len() {
            let item = grid[x][y].clone().map(|v| {
                let impassable = v.is_impassable();
                (v, impassable)
            });
            output[x].push(item);
        }
    }

    output
}
