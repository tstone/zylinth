use super::room::Room;
use std::cmp;

/// A "room grid" is a grid of generated rooms. There are only rooms, no connections between them.
/// This differs from a "layout" in that a layout has been resolved into tiles and is connected
pub struct RoomGrid {
    pub room_count_x: usize,
    pub room_count_y: usize,
    pub rooms: Vec<Vec<Room>>,
    pub max_room_width: Vec<u32>,
    pub max_room_height: Vec<u32>,
}

impl RoomGrid {
    pub fn generate(count_x: usize, count_y: usize) -> RoomGrid {
        let mut rooms: Vec<Vec<Room>> = vec![vec![Room::default(); count_y]; count_x];

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
                let mut room = Room::generate_walled(10, 20);

                // if rand::random_bool(0.225) {
                //     Room::remove_bottom_left_chunk_walled(&mut room);
                // }

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
