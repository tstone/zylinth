use bevy::color::palettes::tailwind::{
    GRAY_50, GRAY_300, GRAY_500, GRAY_700, SLATE_700, SLATE_900, YELLOW_300,
};
use bevy::prelude::*;

use crate::defs::{ControlLink, ControlSource, ControlTarget};
use crate::player::Player;
use crate::selection::Selectable;

#[derive(Resource, Default, Debug)]
pub struct ConnectionState {
    source_entity: Option<Entity>,
    source_id: Option<u8>,
    target_entity: Option<Entity>,
    target_id: Option<u8>,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionMode {
    #[default]
    Default,
    MakingConnection,
}

#[derive(Event)]
#[allow(unused)]
pub struct SourceStateChanged {
    pub source_id: u8,
    pub on: bool,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct TempConnectionLine;

pub struct ConnectionsPlugin;

impl Plugin for ConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SourceStateChanged>();
        app.insert_state(ConnectionMode::Default);
        app.insert_resource(ConnectionState::default());
        app.init_gizmo_group::<TempConnectionLine>();

        app.add_systems(Startup, temp_line_setup);
        app.add_systems(Update, propagate_source_to_target);
        app.add_systems(
            Update,
            start_connection.run_if(in_state(ConnectionMode::Default)),
        );
        app.add_systems(
            Update,
            (render_temp_connection, end_connection)
                .run_if(in_state(ConnectionMode::MakingConnection)),
        );
    }
}

fn start_connection(
    selectables: Query<(
        &Selectable,
        Entity,
        Option<&ControlSource>,
        Option<&ControlTarget>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
    mut connection_state: ResMut<ConnectionState>,
    mut next_mode: ResMut<NextState<ConnectionMode>>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    if input.any_just_pressed([KeyCode::KeyV, KeyCode::ControlRight]) {
        for (selectable, entity, source, target) in selectables.iter() {
            if selectable.selected {
                if let Some(source) = source {
                    connection_state.source_entity = Some(entity);
                    connection_state.source_id = Some(source.id);
                }
                if let Some(target) = target {
                    connection_state.target_entity = Some(entity);
                    connection_state.target_id = Some(target.id);
                }

                if connection_state.source_entity.is_some()
                    || connection_state.target_entity.is_some()
                {
                    // change state to connection active
                    next_mode.set(ConnectionMode::MakingConnection);
                } else {
                    debug!("Connection not started as no source or target found");
                }
            }
        }
    }
}

fn temp_line_setup(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<TempConnectionLine>();
    config.line_style = GizmoLineStyle::Dotted;
    config.line_width = 3.0;
}

fn render_temp_connection(
    player: Query<&GlobalTransform, With<Player>>,
    mut gizmos: Gizmos<TempConnectionLine>,
    connection_state: Res<ConnectionState>,
    entities: Query<&GlobalTransform, Without<Player>>,
) {
    let from_entity = connection_state
        .source_entity
        .or(connection_state.target_entity);
    if let Some(from_entity) = from_entity {
        let from_transform = entities.get(from_entity).unwrap();
        let player_transform = player.get_single().unwrap();
        let from = Vec2::new(
            from_transform.translation().x,
            from_transform.translation().y,
        );
        let to = Vec2::new(
            player_transform.translation().x,
            player_transform.translation().y,
        );

        gizmos.line_gradient_2d(from, to, GRAY_500, SLATE_900);
    }
}

fn end_connection(
    selectables: Query<(
        &Selectable,
        Entity,
        Option<&ControlSource>,
        Option<&ControlTarget>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
    mut connection_state: ResMut<ConnectionState>,
    mut next_mode: ResMut<NextState<ConnectionMode>>,
    mut commands: Commands,
) {
    if input.any_just_pressed([KeyCode::KeyV, KeyCode::ControlRight]) {
        for (selectable, entity, source, target) in selectables.iter() {
            if selectable.selected {
                match (
                    connection_state.source_id,
                    connection_state.target_id,
                    source,
                    target,
                ) {
                    // there is at least a from source and only a to target
                    (Some(source_id), _, None, Some(target))
                        if connection_state.source_entity != Some(entity) =>
                    {
                        commands.spawn(ControlLink::new(source_id, target.id));
                    }
                    // there is at least a from target, and only a to source
                    (_, Some(target_id), Some(source), None)
                        if connection_state.target_entity != Some(entity) =>
                    {
                        commands.spawn(ControlLink::new(source.id, target_id));
                    }
                    // there is only a from source, and at least a to target
                    (Some(source_id), None, _, Some(target))
                        if connection_state.source_entity != Some(entity) =>
                    {
                        commands.spawn(ControlLink::new(source_id, target.id));
                    }
                    // there is only a from target, and at least a to source
                    (None, Some(target_id), Some(source), _)
                        if connection_state.target_entity != Some(entity) =>
                    {
                        commands.spawn(ControlLink::new(source.id, target_id));
                    }
                    _ => {
                        warn!(
                            "Found ambiguous or disallowed case for connection: {:?} {:?} {:?} {:?}",
                            connection_state.source_id, connection_state.target_id, source, target,
                        );
                    }
                }
            }
        }

        // TODO: turn off temp

        connection_state.source_entity = None;
        connection_state.source_id = None;
        connection_state.target_entity = None;
        connection_state.target_id = None;
        next_mode.set(ConnectionMode::Default);
    }
}

pub fn propagate_source_to_target(
    mut ev_sourcestate: EventReader<SourceStateChanged>,
    links: Query<&ControlLink>,
    mut targets: Query<&mut ControlTarget>,
) {
    for event in ev_sourcestate.read() {
        for link in links.iter() {
            if link.source == event.source_id {
                let mut target = targets
                    .iter_mut()
                    .find(|target| target.id == link.target)
                    .unwrap();

                if event.on && !target.activated {
                    target.activated = true;
                } else if !event.on && target.activated {
                    target.activated = false;
                }
            }
        }
    }
}
