use avian2d::prelude::{Collider, CollisionLayers, RigidBody, Sensor};
use bevy::prelude::*;

use crate::connections::SourceStateChanged;
use crate::defs::{ControlLink, ControlSource, ControlTarget, GameLayer};
use crate::map::{Tile, TileRole, TuesdayTile};
use crate::selection::Selectable;

#[derive(Component)]
pub struct DoorPanel;

pub struct DoorPanelPlugin;

impl Plugin for DoorPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(panel_added);
        app.add_systems(Update, (target_changed, selection_changed));
    }
}

fn panel_added(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(Entity, &Tile)>,
    links: Query<&ControlLink>,
    sources: Query<&ControlSource>,
    mut commands: Commands,
) {
    let (entity, tile) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::DoorPanel(id)) = tile.role {
        let source_links = links.iter().filter(|l| l.source == id).collect::<Vec<_>>();
        let target_links = links.iter().filter(|l| l.target == id).collect::<Vec<_>>();
        let source_connected = source_links.len() > 0;
        let mut on = false;
        if source_connected {
            let source = sources
                .iter()
                .find(|s| s.id == source_links[0].source)
                .unwrap();
            on = source.on;
        }

        // panels are both sources and targets
        commands.entity(entity).insert((
            DoorPanel,
            ControlSource::new(id, on, source_connected),
            ControlTarget::new(id, on, target_links.len() > 0),
            Selectable::default(),
            RigidBody::Static,
            Sensor,
            Collider::rectangle(14.0, 20.0),
            CollisionLayers::new(GameLayer::Interactables, [GameLayer::Player]),
        ));
    }
}

fn target_changed(
    mut panels: Query<
        (&ControlTarget, &mut ControlSource, &mut Sprite, &Selectable),
        (With<DoorPanel>, Changed<ControlTarget>),
    >,
    mut ev_sourcestate: EventWriter<SourceStateChanged>,
) {
    for (target, mut source, mut sprite, selectable) in panels.iter_mut() {
        if target.activated != source.on {
            debug!("panel {} changed to {}", target.id, target.activated);
            source.on = target.activated;
            ev_sourcestate.send(SourceStateChanged {
                source_id: source.id,
                on: target.activated,
            });
        }

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = panel_sprite_index(target.connected, selectable);
        }
    }
}

fn selection_changed(
    mut panels: Query<
        (&mut Sprite, &Selectable, &ControlTarget),
        (With<DoorPanel>, Changed<Selectable>),
    >,
) {
    for (mut sprite, selectable, target) in panels.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = panel_sprite_index(target.connected, selectable);
        }
    }
}

fn panel_sprite_index(connected: bool, selectable: &Selectable) -> usize {
    if connected && selectable.selected {
        TuesdayTile::PanelEnabledSelected.into()
    } else if connected && !selectable.selected {
        TuesdayTile::PanelEnabled(1).into()
    } else if !connected && selectable.selected {
        TuesdayTile::PanelDisabledSelected.into()
    } else {
        TuesdayTile::PanelDisabled(1).into()
    }
}
