use std::fmt::Debug;

use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Debug)]
pub struct Tileset<T> {
    pub render: fn(T, &mut ChaCha8Rng) -> usize,
    pub tile_width: u8,
    pub tile_height: u8,
    pub image: Handle<Image>,
}
