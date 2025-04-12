use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

use crate::sprite_animation::SpriteAnimConfig;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

#[derive(Component)]
struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 16, 1, None, None);
    let mut anim_config = SpriteAnimConfig::new(0, 15, 11);

    commands.spawn((
        Player,
        Sprite {
            image: asset_server.load("D2.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layouts.add(layout),
                index: anim_config.first_sprite_index,
            }),
            ..default()
        },
        anim_config,
        PointLight2d {
            color: Color::hsl(57.0, 0.6, 0.79),
            intensity: 3.0,
            radius: 150.0,
            falloff: 3.0,
            ..default()
        },
        Transform::from_xyz(0., 0., 20.),
    ));
}

// TODO: light to follow player
