use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

use crate::layout::{
    cosmic_legacy::{decorate, utility_to_cosmic},
    fixer::floor_fixer,
};

use super::{
    floor_plan::{l_room, perlin_room},
    modifications::flip_horz,
    shadowizer::shadowize,
    wall_wrap::wrap_walls,
};

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
    let mut rng = ChaCha8Rng::seed_from_u64(8);

    // TODO: randomize size a little
    let width: u32 = 12;
    let height: u32 = 12;

    // let floor = flip_horz(l_room(width as usize, height as usize, 3, 2));
    let floor = perlin_room(width as usize, height as usize, &mut rng);
    let floor_fixed = floor_fixer(floor, &mut rng);
    let walled = wrap_walls(floor_fixed, &mut rng);
    // let shadow_walls = shadowize(walled, &mut rng);
    // let decorations = decorate(&shadow_walls, &mut rng);
    let tile_grid = utility_to_cosmic(walled, &mut rng);

    let width: u32 = tile_grid.len() as u32;
    let height: u32 = tile_grid[0].len() as u32;
    let map_size = TilemapSize {
        x: width,
        y: height,
    };

    let texture_handle: Handle<Image> = asset_server.load("CosmicLegacy_PetricakeGames.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..width {
        for y in 0..height {
            // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
            let flipped_y = height - y - 1;
            let tile_pos = TilePos { x, y: flipped_y };

            match tile_grid[x as usize][y as usize] {
                Some(sprite) => {
                    let tile_entity = commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(sprite.into()),
                            tilemap_id: TilemapId(tilemap_entity),
                            ..default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
                _ => {}
            }

            // match decorations[x as usize][y as usize] {
            //     Some(sprite) => {
            //         let tile_entity = commands
            //             .spawn(TileBundle {
            //                 position: tile_pos,
            //                 texture_index: TileTextureIndex(sprite.into()),
            //                 tilemap_id: TilemapId(tilemap_entity),
            //                 ..default()
            //             })
            //             .id();
            //         tile_storage.set(&tile_pos, tile_entity);
            //     }
            //     _ => {}
            // }
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("CosmicLegacy_PetricakeGames.png")),
            tile_size,
            ..Default::default()
        });
    }
}
