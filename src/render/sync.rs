use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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

pub fn set_tile_in_tilemap(
    commands: &mut Commands,
    storage: &mut TileStorage,
    tilemap_entity: Entity,
    color: Color,
    x: u32,
    y: u32,
) {
    let pos = TilePos { x, y };
    // If there's an existing tile at this position, we can choose to overwrite it:
    if let Some(existing) = storage.get(&pos) {
        commands.entity(existing).despawn();
    }
    let tile_entity = commands.spawn((TileBundle {
        position: pos,
        tilemap_id: TilemapId(tilemap_entity),
        texture_index: TileTextureIndex(0),
        color: TileColor(color),
        ..Default::default()
    }, Name::new("Tile"))).id();
    storage.set(&pos, tile_entity);
}