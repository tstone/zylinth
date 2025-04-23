use std::collections::HashMap;

use avian2d::prelude::{Collider, CollisionLayers, RigidBody, Sensor};
use bevy::prelude::*;

use crate::defs::GameLayer;
use crate::door::Door;
use crate::map::{Tile, TileRole, TuesdayTile};
use crate::selection::Selectable;

#[derive(Component)]
pub struct DoorPanel {
    pub id: u8,
}

pub struct DoorPanelPlugin;

impl Plugin for DoorPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(door_added);
        app.add_systems(Update, door_changed);
        app.add_systems(Update, update_panel_selection);
    }
}

fn door_added(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(Entity, &Tile)>,
    mut commands: Commands,
) {
    let (entity, tile) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::DoorPanel(id)) = tile.role {
        commands.entity(entity).insert((
            DoorPanel { id },
            Selectable::default(),
            RigidBody::Static,
            Sensor,
            Collider::rectangle(14.0, 20.0),
            CollisionLayers::new(GameLayer::Interactables, [GameLayer::Player]),
        ));
    }
}

fn door_changed(
    doors: Query<&Door, Changed<Door>>,
    mut panels: Query<(&DoorPanel, &Selectable, &mut Sprite)>,
) {
    for door in doors.iter() {
        for (panel, selectable, mut sprite) in panels.iter_mut() {
            if panel.id == door.id {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = panel_sprite_index(door, selectable);
                }
            }
        }
    }
}

fn update_panel_selection(
    doors: Query<&Door>,
    mut panels: Query<(&DoorPanel, &Selectable, &mut Sprite), Changed<Selectable>>,
) {
    for (panel, selectable, mut sprite) in panels.iter_mut() {
        for door in doors.iter() {
            if door.id == panel.id {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = panel_sprite_index(door, selectable);
                }
            }
        }
    }
}

fn panel_sprite_index(door: &Door, selectable: &Selectable) -> usize {
    if door.open && selectable.selected {
        TuesdayTile::PanelEnabledSelected.into()
    } else if door.open && !selectable.selected {
        TuesdayTile::PanelEnabled(1).into()
    } else if !door.open && selectable.selected {
        TuesdayTile::PanelDisabledSelected.into()
    } else {
        TuesdayTile::PanelDisabled(1).into()
    }
}
