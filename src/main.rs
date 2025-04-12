use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_lit::prelude::Lighting2dPlugin;
use bevy_pancam::PanCamPlugin;
use player::PlayerPlugin;
use sprite_animation::SpriteAnimationPlugin;

mod layout;
mod player;
mod sprite_animation;
mod startup;

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
            Lighting2dPlugin,
            PanCamPlugin,
            SpriteAnimationPlugin,
            PlayerPlugin,
        ))
        .insert_resource(ClearColor(BASE_MAROON))
        .add_systems(Startup, startup::camera)
        .add_systems(PostStartup, startup::zoom)
        .add_systems(Startup, layout::generate_layout)
        .add_systems(PostStartup, layout::spot_lights)
        .run();
}
