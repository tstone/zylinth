use std::collections::HashMap;

use rand::Rng;
use rand::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileDir {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

#[allow(unused)]
impl TileDir {
    /// Get all directions as a list
    pub fn vec() -> Vec<TileDir> {
        vec![TileDir::Left, TileDir::Right, TileDir::Up, TileDir::Down]
    }

    /// Get all directions as a list, except the one given
    pub fn vec_without(dir: &TileDir) -> Vec<TileDir> {
        Self::vec()
            .iter()
            .filter(|d| *d != dir)
            .map(|v| *v)
            .collect::<Vec<_>>()
    }

    /// Randomly pick a direction
    pub fn rnd(rng: &mut impl Rng) -> TileDir {
        *Self::vec().choose(rng).unwrap()
    }

    /// Randomly pick a direction that is not the given value
    pub fn rnd_without(dir: &TileDir, rng: &mut impl Rng) -> TileDir {
        *Self::vec_without(dir).choose(rng).unwrap()
    }

    /// Randomly pick a direction using weights. Only weighted directions are used.
    pub fn rnd_weighted(weights: &HashMap<TileDir, f32>, rng: &mut impl Rng) -> TileDir {
        let rnd = rng.random::<f32>();
        let mut running_weight = 0.0;
        for (dir, weight) in weights {
            running_weight += weight;
            if rnd < running_weight {
                return *dir;
            }
        }
        return TileDir::default();
    }
}

// pub enum RoomShape {
//     ┐
// }
