use crate::costmic_legacy_tiles::CosmicLegacyTiles;

use super::{maze::Maze, room_grid::RoomGrid};

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

        // Copy maze edges into connections list per room
        let mut connections: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; count_y]; count_x];
        for edge in maze.edges {
            let (from_x, from_y) = Maze::node_to_grid_coords(edge.0, maze.width as u32);
            let to = Maze::node_to_grid_coords(edge.1, maze.width as u32);
            connections[from_x as usize][from_y as usize].push((to.0 as usize, to.1 as usize));
        }

        let total_width = room_grid.max_room_width.iter().sum::<u32>();
        let total_height = room_grid.max_room_height.iter().sum::<u32>();
        let mut tiles: Vec<Vec<Option<CosmicLegacyTiles>>> =
            vec![vec![None; total_height as usize]; total_width as usize];

        // copy room tiles into parent grid with offset applied
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

                for room_x in 0..room.tiles.len() {
                    for room_y in 0..room.tiles[x].len() {
                        let final_x = offset_x + room_x as u32 + wiggle_x;
                        let final_y = offset_y + room_y as u32 + wiggle_y;
                        tiles[final_x as usize][final_y as usize] = room.tiles[room_x][room_y];
                    }
                }

                // write connections
            }
        }

        return Layout {
            width: total_width,
            height: total_height,
            tiles,
        };
    }
}
