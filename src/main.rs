use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
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
                        resolution: (2000., 1200.).into(),
                        // mode: bevy::window::WindowMode::Fullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((TilemapPlugin, PanCamPlugin::default()))
        .insert_resource(ClearColor(BASE_MAROON))
        .add_systems(Startup, startup)
        .add_systems(Startup, layout::generate_layout)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off, PanCam::default()));
}
