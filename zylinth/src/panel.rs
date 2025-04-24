use avian2d::prelude::{Collider, CollisionLayers, RigidBody, Sensor};
use bevy::prelude::*;

use crate::connections::SourceStateChanged;
use crate::defs::{ControlSource, ControlTarget, GameLayer};
use crate::map::{Tile, TileRole, TuesdayTile};
use crate::selection::Selectable;

#[derive(Component)]
pub struct DoorPanel;

pub struct DoorPanelPlugin;

impl Plugin for DoorPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(panel_added);
        app.add_systems(Update, activation_changed);
    }
}

fn panel_added(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(Entity, &Tile)>,
    mut commands: Commands,
) {
    let (entity, tile) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::DoorPanel(id)) = tile.role {
        // panels are both sources and targets
        commands.entity(entity).insert((
            DoorPanel,
            ControlSource::off(id),
            ControlTarget::off(id),
            Selectable::default(),
            RigidBody::Static,
            Sensor,
            Collider::rectangle(14.0, 20.0),
            CollisionLayers::new(GameLayer::Interactables, [GameLayer::Player]),
        ));
    }
}

fn activation_changed(
    mut panels: Query<
        (&ControlTarget, &mut ControlSource, &mut Sprite, &Selectable),
        (Changed<ControlTarget>, With<DoorPanel>),
    >,
    mut ev_sourcestate: EventWriter<SourceStateChanged>,
) {
    for (target, mut source, mut sprite, selectable) in panels.iter_mut() {
        debug!("panel {} changed to {}", target.id, target.activated);
        // propagation activation to source
        source.on = target.activated;
        // update sprite
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = panel_sprite_index(target.activated, selectable);
        }
        ev_sourcestate.send(SourceStateChanged {
            source_id: source.id,
            on: target.activated,
        });
    }
}

fn panel_sprite_index(activated: bool, selectable: &Selectable) -> usize {
    if activated && selectable.selected {
        TuesdayTile::PanelEnabledSelected.into()
    } else if activated && !selectable.selected {
        TuesdayTile::PanelEnabled(1).into()
    } else if !activated && selectable.selected {
        TuesdayTile::PanelDisabledSelected.into()
    } else {
        TuesdayTile::PanelDisabled(1).into()
    }
}
