use bevy::prelude::*;

use crate::door::Door;
use crate::map::{Tile, TileRole, TuesdayTile};

#[derive(Component)]
pub struct DoorPanel {
    pub id: u8,
}

pub struct DoorPanelPlugin;

impl Plugin for DoorPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(door_added);
        app.add_systems(Update, door_changed);
    }
}

fn door_added(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(Entity, &Tile)>,
    mut commands: Commands,
) {
    let (entity, tile) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::DoorPanel(id)) = tile.role {
        commands.entity(entity).insert(DoorPanel { id });
    }
}

fn door_changed(doors: Query<&Door, Changed<Door>>, mut panels: Query<(&DoorPanel, &mut Sprite)>) {
    for door in doors.iter() {
        for (panel, mut sprite) in panels.iter_mut() {
            if panel.id == door.id {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if door.open {
                        atlas.index = TuesdayTile::PanelEnabled(1).into();
                    } else {
                        atlas.index = TuesdayTile::PanelDisabled(1).into();
                    }
                }
            }
        }
    }
}
