use avian2d::prelude::*;
use bevy::dev_tools::fps_overlay::*;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_lit::prelude::Lighting2dPlugin;
use camera::CameraSetup;
use door::DoorPlugin;
use map::{SpawnBuildingMap, TileLayoutPlugin};
use panel::DoorPanelPlugin;
use player::PlayerPlugin;
use seed::SeedPlugin;
use sprite_animation::SpriteAnimationPlugin;
use switch::SwitchPlugin;

mod camera;
mod door;
mod map;
mod panel;
mod player;
mod seed;
mod sprite_animation;
mod switch;

const BASE_COLOR: Color = Color::hsl(231., 0.39, 0.13);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        name: Some("Game".to_string()),
                        resolution: (1920., 1080.).into(),
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
        .add_plugins(FpsOverlayPlugin { ..default() })
        .add_plugins((Lighting2dPlugin, SpriteAnimationPlugin, CameraSetup))
        .add_plugins(PhysicsPlugins::default())
        // .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins((
            SeedPlugin,
            PlayerPlugin,
            TileLayoutPlugin,
            DoorPlugin,
            SwitchPlugin,
            DoorPanelPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .insert_resource(ClearColor(BASE_COLOR))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.queue(SpawnBuildingMap {
        width: 50,
        height: 22,
        density: 0.125,
        branch_factor: 0.25,
        wander_factor: 0.5,
    });
}
