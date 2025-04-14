use bevy::{color::palettes::tailwind::GREEN_500, prelude::*};

use bevy_lit::prelude::PointLight2d;
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

use crate::layout::floor_plan::perlin_dog_bone;
use crate::layout::{cosmic_legacy::decorate, fixer::floor_fixer, tilemap::render_tilemap};

use super::{
    cosmic_legacy::CosmicLegacyTile, floor_plan::perlin_room, shadowizer::shadowize,
    wall_wrap::wrap_walls,
};

pub fn generate_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // TODO: make room type generates
    // - biology lab
    // - hydroponics lab
    // - library
    // - sewers
    // - office

    // needs fixes:
    // 16931032955856955107 - weird top left corners
    // 4952264456829212967 - shadow left transition is wrong
    // 12594041454820947593 don't keep 1 tile islands

    let seed = random_range(0..u64::MAX);
    debug!("Using seed: {seed}");
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    // TODO: randomize size a little
    let width: u32 = 40;
    let height: u32 = 12;

    let floor = perlin_dog_bone(width as usize, height as usize, &mut rng);
    let floor_fixed = floor_fixer(floor, &mut rng);
    let walled = wrap_walls(floor_fixed, &mut rng);
    let bg_decorations = decorate(&walled, &mut rng);
    let shadow_walls = shadowize(walled, &mut rng);

    render_tilemap(
        shadow_walls,
        &CosmicLegacyTile::to_utility_tileset(&asset_server, &mut texture_atlas_layouts),
        Transform::from_xyz(0.0, 0.0, -1.0),
        &mut commands,
        &mut rng,
    );
    render_tilemap(
        bg_decorations,
        &CosmicLegacyTile::to_cosmic_tileset(&asset_server, &mut texture_atlas_layouts),
        Transform::from_xyz(0.0, 0.0, 2.0),
        &mut commands,
        &mut rng,
    );
}

pub fn spot_lights(tiles: Query<(&CosmicLegacyTile, Entity)>, mut commands: Commands) {
    for (tile, entity) in tiles.iter() {
        if *tile == CosmicLegacyTile::AlienTop {
            commands.entity(entity).insert((PointLight2d {
                color: Color::from(GREEN_500),
                radius: 40.0,
                intensity: 4.0,
                falloff: 8.0,
                ..default()
            },));
        }
    }
}
