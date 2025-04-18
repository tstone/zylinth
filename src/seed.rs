use bevy::prelude::*;
use rand::random_range;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        let seed = random_range(0..u64::MAX);
        // let seed = 1;
        debug!("Using rng seed: {seed}");
        app.insert_resource(RngSeed(seed));
    }
}

#[derive(Resource)]
pub struct RngSeed(pub u64);
