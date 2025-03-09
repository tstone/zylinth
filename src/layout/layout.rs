use rand::{rngs::ThreadRng, seq::IndexedRandom};
use std::cmp;

use crate::costmic_legacy_tiles::CosmicLegacyTiles;

use super::{maze::Maze, room::Room, room_grid::RoomGrid};

#[derive(Debug, Clone)]
pub struct Layout {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Option<CosmicLegacyTiles>>>,
}

impl Layout {
    pub fn new(count_x: usize, count_y: usize) -> Layout {
        let room_grid = RoomGrid::generate(count_x, count_y);
        let maze = Maze::generate(count_x as u16, count_y as u16);

        // space out the rooms for a bit more natural feel
        let extra_room_spacing = 4;
        let enlarged_max_room_width = room_grid
            .max_room_width
            .iter()
            .map(|v| v + extra_room_spacing)
            .collect::<Vec<_>>();
        let enlarged_max_room_height = room_grid
            .max_room_height
            .iter()
            .map(|v| v + extra_room_spacing)
            .collect::<Vec<_>>();

        // Copy maze edges into connections list per room
        let mut connections: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; count_y]; count_x];
        for edge in maze.edges {
            let (from_x, from_y) = Maze::node_to_grid_coords(edge.0, maze.width as u32);
            let (to_x, to_y) = Maze::node_to_grid_coords(edge.1, maze.width as u32);
            connections[from_x as usize][from_y as usize].push((to_x as usize, to_y as usize));
        }

        let total_width = enlarged_max_room_width.iter().sum::<u32>();
        let total_height = enlarged_max_room_height.iter().sum::<u32>();
        let mut tiles: Vec<Vec<Option<CosmicLegacyTiles>>> =
            vec![vec![None; total_height as usize]; total_width as usize];

        // After wiggles and other modifications have been applied, this keeps track
        // of the top left coords of each rom
        let mut top_left_coord: Vec<Vec<(u32, u32)>> = vec![vec![(0, 0); count_y]; count_x];

        // copy room tiles into parent grid with offset applied
        for x in 0..room_grid.room_count_x {
            let offset_x = enlarged_max_room_width.iter().take(x).sum::<u32>();

            for y in 0..room_grid.room_count_y {
                let offset_y = enlarged_max_room_height.iter().take(y).sum::<u32>();
                let room = &room_grid.rooms[x][y];

                // randomly displace the room in it's possible area to make it less uniform
                let rem_x = enlarged_max_room_width[x] - room.width as u32;
                let rem_y = enlarged_max_room_height[y] - room.height as u32;
                let wiggle_x = match rem_x {
                    0 => 0,
                    x => rand::random_range(0..x),
                };
                let wiggle_y = match rem_y {
                    0 => 0,
                    y => rand::random_range(0..y),
                };

                top_left_coord[x][y] = (offset_x + wiggle_x, offset_y + wiggle_y);

                // copy room tiles
                for room_x in 0..room.tiles.len() {
                    for room_y in 0..room.tiles[x].len() {
                        let final_x = offset_x + room_x as u32 + wiggle_x;
                        let final_y = offset_y + room_y as u32 + wiggle_y;
                        tiles[final_x as usize][final_y as usize] = room.tiles[room_x][room_y];
                    }
                }
            }
        }

        let mut rng = rand::rng();
        for x in 0..room_grid.room_count_x {
            for y in 0..room_grid.room_count_y {
                let from_room = &room_grid.rooms[x][y];
                let from_pos = top_left_coord[x][y];

                // overwrite tiles with connections
                for (to_x, to_y) in &connections[x][y] {
                    let to_pos = top_left_coord[*to_x][*to_y];
                    let to_room = &room_grid.rooms[*to_x][*to_y];

                    if to_y == &y {
                        connect_horz(from_room, from_pos, to_room, to_pos, &mut tiles, &mut rng);
                    } else {
                        connect_vert(from_room, from_pos, to_room, to_pos, &mut tiles, &mut rng);
                    }
                }
            }
        }

        return Layout {
            width: total_width,
            height: total_height,
            tiles,
        };
    }
}

/// Find the contiguous run of where the rooms "overlap" in the Y dimension
fn get_x_overlap(
    left_room: &Room,
    left_pos: (u32, u32),
    right_room: &Room,
    right_pos: (u32, u32),
    rng: &mut ThreadRng,
) -> (u32, u32) {
    let start_y = cmp::max(
        left_pos.1 + left_room.top_margin as u32 + 2, // for shadow
        right_pos.1 + right_room.top_margin as u32 + 2,
    );
    let end_y = cmp::min(
        left_pos.1 + left_room.height as u32 - left_room.bot_margin as u32 - 1,
        right_pos.1 + right_room.height as u32 - right_room.bot_margin as u32 - 1,
    );

    let y_range = (start_y..end_y).collect::<Vec<_>>();

    // randomly pick a range to use
    let a = y_range.choose(rng);
    let b = y_range.choose(rng);

    match (a, b) {
        (Some(a), Some(b)) if a > b && a - b > 1 => (*b, *a),
        (Some(a), Some(b)) if a < b && b - a > 1 => (*a, *b),
        _ => (start_y, end_y),
    }
}

/// Find the contiguous run of where the rooms "overlap" in the Y dimension
fn get_y_overlap(
    left_room: &Room,
    left_pos: (u32, u32),
    right_room: &Room,
    right_pos: (u32, u32),
    rng: &mut ThreadRng,
) -> (u32, u32) {
    let start_x = cmp::max(
        left_pos.0 + left_room.left_margin as u32 + 1,
        right_pos.0 + right_room.left_margin as u32 + 1,
    );
    let end_x = cmp::min(
        left_pos.0 + left_room.width as u32 - left_room.right_margin as u32 - 1,
        right_pos.0 + right_room.width as u32 - right_room.right_margin as u32 - 1,
    );

    let x_range = (start_x..end_x).collect::<Vec<_>>();

    // randomly pick a range to use
    let a = x_range.choose(rng);
    let b = x_range.choose(rng);

    match (a, b) {
        (Some(a), Some(b)) if a > b && a - b > 1 => (*b, *a),
        (Some(a), Some(b)) if a < b && b - a > 1 => (*a, *b),
        _ => (start_x, end_x),
    }
}

/// Create the "bridge" horizontally between rooms
fn connect_horz(
    from_room: &Room,
    from_pos: (u32, u32),
    to_room: &Room,
    to_pos: (u32, u32),
    tiles: &mut Vec<Vec<Option<CosmicLegacyTiles>>>,
    rng: &mut ThreadRng,
) {
    let overlap = get_x_overlap(from_room, from_pos, to_room, to_pos, rng);
    let from_right = from_pos.0 + from_room.width as u32 - from_room.right_margin as u32;
    let to_right = to_pos.0 + to_room.width as u32 - to_room.right_margin as u32;

    let x_range = if from_right < to_right {
        from_right..to_right
    } else {
        to_right..from_right
    };

    // floor
    for (y_index, tile_y) in ((overlap.0)..=(overlap.1)).enumerate() {
        let mut done = false;
        for (x_index, tile_x) in x_range.clone().enumerate() {
            let current_tile = tiles[tile_x as usize][tile_y as usize];
            tiles[tile_x as usize][tile_y as usize] = match current_tile {
                Some(CosmicLegacyTiles::FloorShadowLeft) | Some(CosmicLegacyTiles::Floor) => {
                    done = true;
                    if y_index == 0 {
                        Some(CosmicLegacyTiles::FloorShadowOuterCorner)
                    } else {
                        Some(CosmicLegacyTiles::Floor)
                    }
                }
                Some(CosmicLegacyTiles::TopCapLeft)
                | Some(CosmicLegacyTiles::TopCapRight)
                | Some(CosmicLegacyTiles::BottomLeftOuterCorner)
                | Some(CosmicLegacyTiles::BottomRightOuterCorner)
                | Some(CosmicLegacyTiles::TopCapBottomSimple)
                | None => {
                    if y_index == 0 && x_index == 0 {
                        Some(CosmicLegacyTiles::FloorShadowTopFadeLeft)
                    } else if y_index == 0 {
                        Some(CosmicLegacyTiles::FloorShadowTop)
                    } else {
                        Some(CosmicLegacyTiles::Floor)
                    }
                }
                t => t,
            };
            if done {
                break;
            }
        }
    }

    // top wall
    let top_y = overlap.0 as usize;
    for y in (top_y - 2..top_y) {
        let mut done = false;
        for tile_x in x_range.clone() {
            let current_tile = tiles[tile_x as usize][y as usize];
            tiles[tile_x as usize][y] = match current_tile {
                Some(CosmicLegacyTiles::TopCapLeft)
                | Some(CosmicLegacyTiles::FloorShadowLeft)
                | Some(CosmicLegacyTiles::Floor)
                | Some(CosmicLegacyTiles::TopCapBottomSimple) => {
                    done = true;
                    Some(CosmicLegacyTiles::Wall)
                }
                _ => Some(CosmicLegacyTiles::Wall),
            };
            if done {
                break;
            }
        }
    }

    // bottom wall
    let mut done = false;
    let bottom_y = overlap.1 as usize + 1;
    for tile_x in x_range {
        let current_tile = tiles[tile_x as usize][bottom_y as usize];
        tiles[tile_x as usize][bottom_y] = match current_tile {
            Some(CosmicLegacyTiles::TopCapRight) => Some(CosmicLegacyTiles::TopLeftInnerCorner),
            Some(CosmicLegacyTiles::TopCapLeft) => {
                done = true;
                tiles[(tile_x + 1) as usize][bottom_y as usize] =
                    Some(CosmicLegacyTiles::FloorShadowLeftFadeUp);
                Some(CosmicLegacyTiles::TopRightInnerCorner)
            }
            Some(CosmicLegacyTiles::FloorShadowLeft)
            | Some(CosmicLegacyTiles::Floor)
            | Some(CosmicLegacyTiles::TopCapBottomSimple) => {
                done = true;
                Some(CosmicLegacyTiles::TopCapBottomSimple)
            }
            _ => Some(CosmicLegacyTiles::TopCapBottomSimple),
        };
        if done {
            break;
        }
    }
}

/// Create the "bridge" vertically between rooms
fn connect_vert(
    from_room: &Room,
    from_pos: (u32, u32),
    to_room: &Room,
    to_pos: (u32, u32),
    tiles: &mut Vec<Vec<Option<CosmicLegacyTiles>>>,
    rng: &mut ThreadRng,
) {
    let overlap = get_y_overlap(from_room, from_pos, to_room, to_pos, rng);
    let from_bot = from_pos.1 + from_room.height as u32 - from_room.bot_margin as u32;
    let to_bot = to_pos.1 + to_room.height as u32 - to_room.bot_margin as u32;

    for tile_x in (overlap.0)..=(overlap.1) {
        let mut done = false;
        let y_range = if from_bot < to_bot {
            from_bot..to_bot
        } else {
            to_bot..from_bot
        };

        // floor
        for tile_y in y_range {
            let current_tile = tiles[tile_x as usize][tile_y as usize];
            tiles[tile_x as usize][tile_y as usize] = match current_tile {
                Some(CosmicLegacyTiles::FloorShadowLeft)
                | Some(CosmicLegacyTiles::Floor)
                | Some(CosmicLegacyTiles::BottomLeftOuterCorner)
                | Some(CosmicLegacyTiles::BottomRightOuterCorner)
                | Some(CosmicLegacyTiles::TopLeftOuterCorner)
                | Some(CosmicLegacyTiles::TopRightOuterCorner)
                | Some(CosmicLegacyTiles::TopRightInnerCorner)
                | Some(CosmicLegacyTiles::TopLeftInnerCorner) => {
                    done = true;
                    Some(CosmicLegacyTiles::Floor)
                }
                _ => Some(CosmicLegacyTiles::Floor),
            };
            if done {
                break;
            }
        }
    }
}
