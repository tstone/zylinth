use bevy::dev_tools::fps_overlay::*;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_lit::prelude::Lighting2dPlugin;
use camera::CameraSetup;
use movement::MovementPlugin;
use player::PlayerPlugin;
use sprite_animation::SpriteAnimationPlugin;

mod camera;
pub mod collision;
mod layout;
mod movement;
mod player;
mod sprite_animation;

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
        .add_plugins(FpsOverlayPlugin { ..default() })
        .add_plugins((Lighting2dPlugin, MovementPlugin, SpriteAnimationPlugin))
        .add_plugins(CameraSetup)
        .insert_resource(ClearColor(BASE_MAROON))
        .add_systems(Startup, layout::generate_layout)
        .add_systems(PostStartup, layout::spot_lights)
        .add_plugins(PlayerPlugin)
        .run();
}
