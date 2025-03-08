use std::cmp;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::seq::IteratorRandom;

use crate::room::Room;

pub fn render_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    let room_grid = RoomGrid::generate(3, 3);
    let map_size = TilemapSize {
        x: room_grid.max_room_width.iter().sum(),
        y: room_grid.max_room_height.iter().sum(),
    };

    let texture_handle: Handle<Image> = asset_server.load("CosmicLegacy_PetricakeGames.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // render layout
    for x in 0..room_grid.room_count_x {
        let offset_x = room_grid.max_room_width.iter().take(x).sum::<u32>();

        for y in 0..room_grid.room_count_y {
            let offset_y = room_grid.max_room_height.iter().take(y).sum::<u32>();
            let room = &room_grid.rooms[x][y];

            // randomly displace the room in it's possible area to make it less uniform
            let rem_x = room_grid.max_room_width[x] - room.width as u32;
            let rem_y = room_grid.max_room_height[y] - room.height as u32;
            let wiggle_x = match rem_x {
                0 => 0,
                x => rand::random_range(0..x),
            };
            let wiggle_y = match rem_y {
                0 => 0,
                y => rand::random_range(0..y),
            };

            add_room(
                room,
                offset_x + wiggle_x,
                offset_y + wiggle_y,
                tilemap_entity,
                &mut tile_storage,
                &mut commands,
            );
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

struct RoomGrid {
    room_count_x: usize,
    room_count_y: usize,
    rooms: Vec<Vec<Room>>,
    max_room_width: Vec<u32>,
    max_room_height: Vec<u32>,
}

impl RoomGrid {
    fn generate(count_x: usize, count_y: usize) -> RoomGrid {
        let mut rooms: Vec<Vec<Room>> = vec![vec![Room::default(); count_y]; count_y];

        // max room width/height keeps track of the widest/tallest size for a given x/y
        // e.g. if the room sizes are:
        //   (5,5) (6,6)
        //   (7,7), (5,5)
        // Then max_room_width would be [7, 6] and max_room_height [6, 7]
        let mut max_room_width: Vec<u32> = vec![0; count_x];
        let mut max_room_height: Vec<u32> = vec![0; count_y];

        // generate a layout of rooms
        for x in 0..count_x {
            for y in 0..count_y {
                let mut room = Room::generate_walled(8, 20);

                if rand::random_bool(0.25) {
                    Room::remove_bottom_left_chunk_walled(&mut room);
                }

                max_room_width[x] = cmp::max(room.width.into(), max_room_width[x]);
                max_room_height[y] = cmp::max(room.height.into(), max_room_height[y]);
                rooms[x][y] = room;
            }
        }

        return RoomGrid {
            room_count_x: count_x,
            room_count_y: count_y,
            rooms,
            max_room_width,
            max_room_height,
        };
    }
}

fn add_room(
    room: &Room,
    offset_x: u32,
    offset_y: u32,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    commands: &mut Commands,
) {
    for x in 0..room.tiles.len() {
        let y_len = room.tiles[x].len() as u32;
        for y in 0..y_len {
            match room.tiles[x][y as usize] {
                Some(sprite) => add_tile(
                    x as u32 + offset_x,
                    (y_len - y - 1) + offset_y,
                    sprite.into(),
                    tilemap_entity,
                    tile_storage,
                    commands,
                ),
                _ => {}
            }
        }
    }
}

fn add_tile(
    x: u32,
    y: u32,
    sprite_id: u32,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    commands: &mut Commands,
) {
    let tile_pos = TilePos { x, y };
    let tile_entity = commands
        .spawn(TileBundle {
            position: tile_pos,
            texture_index: TileTextureIndex(sprite_id),
            tilemap_id: TilemapId(tilemap_entity),
            ..default()
        })
        .id();
    tile_storage.set(&tile_pos, tile_entity);
}
