use bevy::{color::palettes::tailwind::GREEN_500, prelude::*};

use bevy_lit::prelude::PointLight2d;
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

use crate::layout::{
    cosmic_legacy::decorate,
    fixer::floor_fixer,
    tilemap::{TilemapConfig, render_utility_tilemap},
};

use super::{
    cosmic_legacy::CosmicLegacyTile, floor_plan::perlin_room, shadowizer::shadowize,
    wall_wrap::wrap_walls,
};

pub fn generate_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // needs fixes:
    // 16931032955856955107 - weird top left corners
    // 4952264456829212967 - shadow left transition is wrong
    let seed = random_range(0..u64::MAX);
    debug!("Using seed: {seed}");
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    // TODO: randomize size a little
    let width: u32 = 20;
    let height: u32 = 14;

    let floor = perlin_room(width as usize, height as usize, &mut rng);
    let floor_fixed = floor_fixer(floor, &mut rng);
    let walled = wrap_walls(floor_fixed, &mut rng);
    // let background_decorations = decorate(&walled, &mut rng);
    let shadow_walls = shadowize(walled, &mut rng);

    let room_tilemap = TilemapConfig {
        width: shadow_walls.len() as u32,
        height: shadow_walls[0].len() as u32,
        tileset: CosmicLegacyTile::from_utility_tileset(asset_server, texture_atlas_layouts),
    };

    render_utility_tilemap(
        shadow_walls,
        room_tilemap,
        Transform::from_xyz(0.0, 0.0, 10.0),
        commands,
        &mut rng,
    );
}

// TODO: store both the utility tile and the actual tile for spot rendering
// TODO: update sprite spot rendering

pub fn spot_lights() {}

// pub fn spot_lights(
//     sprites: Query<(&TileTextureIndex, &TilePos, &TilemapId)>,
//     tilemaps: Query<(&TilemapGridSize, &TilemapType, &Transform)>,
//     mut commands: Commands,
// ) {
//     for (idx, pos, tilemap_id) in sprites.iter() {
//         if idx.0 == (CosmicLegacyTile::AlienTop as u32) {
//             let (grid_size, map_type, tilemap_transform) = tilemaps.get(tilemap_id.0).unwrap();
//             let center = pos.center_in_world(grid_size, map_type);
//             commands.spawn((
//                 PointLight2d {
//                     color: Color::from(GREEN_500),
//                     radius: 40.0,
//                     intensity: 4.0,
//                     falloff: 8.0,
//                     ..default()
//                 },
//                 Transform::from_xyz(
//                     tilemap_transform.translation.x + center.x,
//                     tilemap_transform.translation.y + (center.y - (grid_size.y / 2.)),
//                     tilemap_transform.translation.z - 1.0,
//                 ),
//             ));
//         }
//     }
// }
