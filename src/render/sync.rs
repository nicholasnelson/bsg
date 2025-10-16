use bevy::prelude::*;
use crate::core::tile::{TileLayer, Tileset, TileId};
use super::tilemaps::{Layers, BaseTile, OverlayTile};

#[derive(Component)]
pub struct GridPos { pub x: u32, pub y: u32 }

pub struct TileSyncPlugin;
impl Plugin for TileSyncPlugin {
    fn build(&self, _app: &mut App) {
        // no-op: helpers used by placement
    }
}

pub fn world_from_grid(x: u32, y: u32) -> Vec3 {
    // Center sprites within 16x16 grid cells whose top-left origin is at (0,0)
    Vec3::new(x as f32 * 16.0 + 8.0, y as f32 * 16.0 + 8.0, 0.0)
}

pub fn spawn_tile_sprite(commands: &mut Commands, tileset: &Tileset, tile: TileId, x: u32, y: u32) {
    let def = tileset.def(tile);
    let (size, z, tag) = match def.layer { TileLayer::Base => (14.0, 0.0, "BaseTile"), TileLayer::Overlay => (8.0, 1.0, "OverlayTile") };
    commands.spawn((
        Sprite { color: def.color, custom_size: Some(Vec2::splat(size)), ..Default::default() },
        Transform::from_translation(world_from_grid(x, y) + Vec3::new(0.0, 0.0, z)),
        GlobalTransform::default(),
        Visibility::Visible,
        GridPos { x, y },
        Name::new(tag),
    ));
}