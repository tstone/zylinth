use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

use crate::connections::SourceStateChanged;
use crate::defs::{ControlLink, ControlSource, GameLayer};
use crate::map::{Tile, TileRole, TuesdayTile};
use crate::selection::Selectable;

#[derive(Component)]
pub struct Switch;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(switch_added);
        app.add_systems(
            PostUpdate,
            press_switch.after(TransformSystem::TransformPropagate),
        );
        app.add_systems(Update, source_changed);
        app.add_systems(Update, selection_changed);
    }
}

fn switch_added(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(&Tile, Entity)>,
    links: Query<&ControlLink>,
    mut commands: Commands,
    mut ev_switchstate: EventWriter<SourceStateChanged>,
) {
    let (tile, entity) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::Switch(id, on)) = tile.role {
        debug!("Setting up switch {id}");
        let existing_links = links.iter().filter(|l| l.source == id).collect::<Vec<_>>();
        commands.entity(entity).insert((
            Switch,
            Selectable::default(),
            ControlSource::new(id, on, existing_links.len() > 0),
            RigidBody::Static,
            Collider::ellipse(10.0, 8.0),
            CollisionLayers::new(GameLayer::Interactables, [GameLayer::Player]),
        ));
        ev_switchstate.send(SourceStateChanged { source_id: id, on });
    }
}

fn press_switch(
    input: Res<ButtonInput<KeyCode>>,
    mut switches: Query<(&mut ControlSource, &Selectable), With<Switch>>,
    mut ev_sourcestate: EventWriter<SourceStateChanged>,
) {
    if input.any_just_released([KeyCode::Enter, KeyCode::KeyF]) {
        for (mut source, selectable) in switches.iter_mut() {
            if selectable.selected {
                source.on = !source.on;
                debug!(
                    "switch {} changed to: {}",
                    source.id,
                    if source.on { "on" } else { "off" }
                );
                ev_sourcestate.send(SourceStateChanged {
                    source_id: source.id,
                    on: source.on,
                });
            }
        }
    }
}

fn source_changed(
    mut switch: Query<
        (&ControlSource, &Selectable, &mut Sprite),
        (Changed<ControlSource>, With<Switch>),
    >,
) {
    for (source, selectable, mut sprite) in switch.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = sprite_index(source, selectable);
        }
    }
}

fn selection_changed(
    mut switch: Query<
        (&ControlSource, &Selectable, &mut Sprite),
        (Changed<Selectable>, With<Switch>),
    >,
) {
    for (source, selectable, mut sprite) in switch.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = sprite_index(source, selectable);
        }
    }
}

fn sprite_index(source: &ControlSource, selectable: &Selectable) -> usize {
    if source.on && selectable.selected {
        TuesdayTile::SwitchSelectedRight.into()
    } else if source.on && !selectable.selected {
        TuesdayTile::SwitchRight(1).into()
    } else if !source.on && !selectable.selected {
        TuesdayTile::SwitchLeft(1).into()
    } else {
        TuesdayTile::SwitchSelectedLeft.into()
    }
}
