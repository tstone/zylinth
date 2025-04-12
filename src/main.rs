use bevy::color::palettes::tailwind::GRAY_300;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_lit::prelude::{AmbientLight2d, Lighting2dPlugin, Lighting2dSettings};
use bevy_pancam::{PanCam, PanCamPlugin};

mod layout;

const BASE_MAROON: Color = Color::hsl(281., 0.51, 0.17);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        name: Some("Game".to_string()),
                        resolution: (1200., 1200.).into(),
                        // mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,zylinth=trace".to_string(),
                    level: Level::TRACE,
                    ..Default::default()
                }),
        )
        .add_plugins((
            TilemapPlugin,
            Lighting2dPlugin, /* PanCamPlugin::default()) */
        ))
        .insert_resource(ClearColor(BASE_MAROON))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, zoom)
        .add_systems(Startup, layout::generate_layout)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        Lighting2dSettings { ..default() },
        AmbientLight2d {
            brightness: 0.1,
            color: Color::from(GRAY_300),
        },
    ));
}

fn zoom(mut query: Query<&mut OrthographicProjection, With<Camera2d>>) {
    let mut projection = query.single_mut();
    projection.scale = 0.3;
}
