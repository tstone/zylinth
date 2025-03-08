use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::seq::IteratorRandom;

use crate::{costmic_legacy_tiles::CosmicLegacyTiles, maze::Maze};

struct Rect {
    w: u16,
    h: u16,
}

pub fn render_maze(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    let base_size = Rect { w: 3, h: 3 };
    let map_size = TilemapSize {
        x: (base_size.w * 3) as u32,
        y: (base_size.h * 3) as u32,
    };

    let texture_handle: Handle<Image> = asset_server.load("CosmicLegacy_PetricakeGames.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let base_maze = Maze::generate(base_size.w, base_size.h);
    println!("maze: {:?}", &base_maze.edges);
    let tiles = expand_maze_to_cosmic_legacy_tiles(&base_maze, 1);
    println!("tiles: {:?}", tiles);

    for x in 0..tiles.len() {
        let y_len = tiles[x].len() as u32;
        for y in 0..y_len {
            match tiles[x][y as usize] {
                Some(sprite) => {
                    let tile_pos = TilePos {
                        x: x as u32,
                        y: y_len - y, // flip Y since sprite sheet 0,0 is bottom left
                    };
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
                _ => {
                    let tile_pos = TilePos {
                        x: x as u32,
                        y: y_len - y, // flip Y since sprite sheet 0,0 is bottom left
                    };
                    let tile_entity = commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(CosmicLegacyTiles::DarkestBlue.into()),
                            tilemap_id: TilemapId(tilemap_entity),
                            ..default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
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

fn expand_maze_to_cosmic_legacy_tiles(
    maze: &Maze,
    path_thickness: u8,
) -> Vec<Vec<Option<CosmicLegacyTiles>>> {
    let mut rng = rand::rng();
    let cap: u32 = 1;
    let wall_thickness: u8 = 1;
    let outer_thickness = wall_thickness + path_thickness;
    let scaled_width = (maze.width * outer_thickness as u16) + 2;
    let scaled_height = (maze.height * outer_thickness as u16) + 2;
    let mut grid: Vec<Vec<Option<CosmicLegacyTiles>>> =
        vec![vec![None; scaled_height as usize]; scaled_width as usize];

    // generate walls first then look for shadows? -- or always assume a wall is along the top?
    // TODO: exclude top cap and add that second?

    // left cap
    for y in 0..scaled_height {
        grid[0][y as usize] = Some(CosmicLegacyTiles::TopCapLeft);
    }

    // top cap
    for x in 0..scaled_width {
        if x == 0 {
            grid[x as usize][0] = Some(CosmicLegacyTiles::TopLeftOuterCorner);
        } else if x == scaled_width - 1 {
            grid[x as usize][0] = Some(CosmicLegacyTiles::TopRightOuterCorner);
        } else {
            grid[x as usize][0] = Some(CosmicLegacyTiles::TopCapTopSimple);
        }
    }

    for (from, _to) in &maze.edges {
        let (from_x, from_y) = Maze::node_to_scaled_coords(
            *from,
            wall_thickness + path_thickness as u8,
            maze.width.into(),
        );
        let from_y = from_y + cap as u32; // always offset Y by top cap

        // walls along the top
        if from_y == cap {
            for x in from_x..=(from_x + outer_thickness as u32) {
                if rand::random_bool(0.65) {
                    grid[x as usize][from_y as usize] = Some(CosmicLegacyTiles::Wall);
                } else {
                    grid[x as usize][from_y as usize] = CosmicLegacyTiles::wall_tiles()
                        .iter()
                        .choose(&mut rng)
                        .copied();
                }
            }
        }

        // center floor squares
        for x in (from_x + cap)..(from_x + outer_thickness as u32) {
            for y in (from_y + cap)..(from_y + outer_thickness as u32) {
                grid[x as usize][y as usize] = Some(CosmicLegacyTiles::Floor);
            }
        }

        // let (to_x, to_y) = Maze::node_to_scaled_coords(*to, scale, maze.width.into());

        // println!("from ({},{}) to ({},{})", from_x, from_y, to_x, to_y);

        // let mid_x = from_x + (scale as u16 / 2);
        // let mid_y = from_y + (scale as u16 / 2);

        // // TODO: handle reverse range

        // if from_y == to_y {
        //     println!(
        //         "range x: {:?}",
        //         (mid_x..=to_x).into_iter().collect::<Vec<_>>()
        //     );
        //     for x in mid_x..=to_x {
        //         grid[x as usize][mid_y as usize] = Some(CosmicLegacyTiles::Floor);
        //     }
        // } else if from_x == to_x {
        //     println!(
        //         "range y: {:?}",
        //         (mid_y..=to_y).into_iter().collect::<Vec<_>>()
        //     );
        //     for y in mid_y..=to_y {
        //         grid[mid_x as usize][y as usize] = Some(CosmicLegacyTiles::Floor);
        //     }
        // } else {
        //     println!("not rendered?");
        // }

        // println!("center: ({},{})", from_x, mid_y);
        // grid[mid_x as usize][mid_y as usize] = Some(CosmicLegacyTiles::PanelFan);
    }

    return grid;
}
