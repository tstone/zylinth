use bevy::{color::palettes::tailwind::GREEN_500, prelude::*};
use bevy_ecs_tilemap::prelude::*;

use bevy_lit::prelude::PointLight2d;
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

use crate::layout::{
    cosmic_legacy::{decorate, utility_to_cosmic},
    fixer::floor_fixer,
};

use super::{
    cosmic_legacy::CosmicLegacyTile,
    floor_plan::{l_room, perlin_room},
    shadowizer::shadowize,
    wall_wrap::wrap_walls,
};

struct DemoRoom;

pub fn generate_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    // needs fixes:
    // 16931032955856955107 - weird top left corners
    let seed = random_range(0..u64::MAX);
    println!("Using seed: {seed}");
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    // TODO: randomize size a little
    let width: u32 = 14;
    let height: u32 = 14;

    let floor = perlin_room(width as usize, height as usize, &mut rng);
    let floor_fixed = floor_fixer(floor, &mut rng);
    let walled = wrap_walls(floor_fixed, &mut rng);
    let decorations = decorate(&walled, &mut rng);

    let shadow_walls = shadowize(walled, &mut rng);
    let tile_grid = utility_to_cosmic(shadow_walls, &mut rng);

    let width: u32 = tile_grid.len() as u32;
    let height: u32 = tile_grid[0].len() as u32;
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let map_size = TilemapSize {
        x: width,
        y: height,
    };

    let texture_handle: Handle<Image> = asset_server.load("CosmicLegacy_PetricakeGames.png");

    // Lower layer (walls/floors)
    render_layer(
        &map_size,
        &tile_size,
        tile_grid,
        &mut commands,
        texture_handle.clone(),
        10.0,
    );
    // Upper layer (decorations)
    render_layer(
        &map_size,
        &tile_size,
        decorations,
        &mut commands,
        texture_handle,
        20.0,
    );

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("CosmicLegacy_PetricakeGames.png")),
            tile_size,
            ..Default::default()
        });
    }
}

fn render_layer<T: PartialEq + Eq + Copy + Into<u32>>(
    map_size: &TilemapSize,
    tile_size: &TilemapTileSize,
    tile_grid: Vec<Vec<Option<T>>>,
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    z: f32,
) {
    let grid_size = TilemapGridSize {
        x: tile_size.x,
        y: tile_size.y,
    };
    let map_type = TilemapType::default();
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(*map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
            let flipped_y = map_size.y - y - 1;
            let tile_pos = TilePos { x, y: flipped_y };

            match &tile_grid[x as usize][y as usize] {
                Some(tile) => {
                    let tile_entity = commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex((*tile).into()),
                            tilemap_id: TilemapId(tilemap_entity),
                            ..default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
                _ => {}
            }
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: *map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: *tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, z),
        ..Default::default()
    });
}

pub fn spot_lights(
    sprites: Query<(&TileTextureIndex, &TilePos, &TilemapId)>,
    tilemaps: Query<(&TilemapGridSize, &TilemapType, &Transform)>,
    mut commands: Commands,
) {
    for (idx, pos, tilemap_id) in sprites.iter() {
        if idx.0 == (CosmicLegacyTile::AlienTop as u32) {
            let (grid_size, map_type, tilemap_transform) = tilemaps.get(tilemap_id.0).unwrap();
            let center = pos.center_in_world(grid_size, map_type);
            println!("center {center}");
            println!("tilemap xyz {:?}", tilemap_transform.translation);
            commands.spawn((
                PointLight2d {
                    color: Color::from(GREEN_500),
                    radius: 40.0,
                    intensity: 4.0,
                    falloff: 8.0,
                    ..default()
                },
                Transform::from_xyz(
                    tilemap_transform.translation.x + center.x,
                    tilemap_transform.translation.y + (center.y - (grid_size.y / 2.)),
                    tilemap_transform.translation.z - 1.0,
                ),
            ));
        }
    }
}
