use bevy::ecs::component::Component;
use tilegen::TileGrid;

pub struct Puzzle<T: Clone + PartialEq + Eq> {
    pub grid: TileGrid<T>,
    pub door_controls: Vec<DoorControl>,
}

#[derive(Component)]
pub struct DoorControl {
    /// the u8 here is the ID of the tile enum
    pub doors: Vec<u8>,
    pub switches: Vec<u8>,
}
