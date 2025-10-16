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

pub fn world_from_grid_with_tile(px: f32, x: u32, y: u32) -> Vec3 {
    // Center sprites within px grid cells whose top-left origin is at (0,0)
    Vec3::new(x as f32 * px + px / 2.0, y as f32 * px + px / 2.0, 0.0)
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

/**
 * Spawns or replaces a tile at (x,y) with a specific texture index and color in a tilemap.
 * Useful when selecting a sprite variant based on connectivity masks or animation frames.
 *
 * @param commands - ECS command buffer
 * @param storage - tile storage for the tilemap
 * @param tilemap_entity - target tilemap entity
 * @param texture_index - tile texture index to assign
 * @param color - tile color tint
 * @param x - tile x coordinate
 * @param y - tile y coordinate
 */
pub fn set_tile_with_index(
    commands: &mut Commands,
    storage: &mut TileStorage,
    tilemap_entity: Entity,
    texture_index: u32,
    color: Color,
    x: u32,
    y: u32,
) {
    let pos = TilePos { x, y };
    if let Some(existing) = storage.get(&pos) {
        commands.entity(existing).despawn();
    }
    let tile_entity = commands.spawn((TileBundle {
        position: pos,
        tilemap_id: TilemapId(tilemap_entity),
        texture_index: TileTextureIndex(texture_index),
        color: TileColor(color),
        ..Default::default()
    }, Name::new("Tile"))).id();
    storage.set(&pos, tile_entity);
}

/**
 * Removes a tile at (x,y) from a tilemap if it exists.
 *
 * @param commands - ECS command buffer
 * @param storage - tile storage for the tilemap
 * @param x - tile x coordinate
 * @param y - tile y coordinate
 */
pub fn remove_tile_in_tilemap(
    commands: &mut Commands,
    storage: &mut TileStorage,
    x: u32,
    y: u32,
) {
    let pos = TilePos { x, y };
    if let Some(existing) = storage.get(&pos) {
        commands.entity(existing).despawn();
        storage.remove(&pos);
    }
}