use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::layout::Layout;

pub fn generate_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    let layout = Layout::new(12, 16);
    let map_size = TilemapSize {
        x: layout.width,
        y: layout.height,
    };

    let texture_handle: Handle<Image> = asset_server.load("CosmicLegacy_PetricakeGames.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..layout.width {
        for y in 0..layout.height {
            match layout.tiles[x as usize][y as usize] {
                Some(sprite) => {
                    // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
                    let flipped_y = layout.height - y - 1;
                    let tile_pos = TilePos { x, y: flipped_y };
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
