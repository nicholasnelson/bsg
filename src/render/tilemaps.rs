use bevy::prelude::*;
use crate::core::map::{MapSize, MapState};

#[derive(Resource)]
pub struct Layers {
    pub base_root: Entity,
    pub overlay_root: Entity,
}

#[derive(Component)]
pub struct BaseTile;
#[derive(Component)]
pub struct OverlayTile;

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tile_layers);
    }
}

fn setup_tile_layers(mut commands: Commands, map: Res<MapState>) {
    let base_root = commands.spawn((Transform::default(), GlobalTransform::default(), Visibility::Visible, Name::new("BaseLayer"))).id();
    let overlay_root = commands.spawn((Transform::default(), GlobalTransform::default(), Visibility::Visible, Name::new("OverlayLayer"))).id();

    commands.insert_resource(Layers { base_root, overlay_root });

    // Pre-spawn empty base tiles (optional). We can spawn lazily during placement too.
    let MapSize { w, h } = map.size;
    for y in 0..h {
        for x in 0..w {
            let _ = (x, y); // keep for potential prefill
        }
    }
}