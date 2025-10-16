use bevy::prelude::*;
use crate::core::map::MapState;
use crate::core::tile::{TileId, Tileset};
use bevy_ecs_tilemap::prelude::*;
use crate::render::tilemaps::TilemapLayers;
use crate::render::sync::set_tile_in_tilemap;
use crate::input::{GameplayInputState, Tool as InputTool};
use crate::core::grid::GridConfig;

pub struct PlacementPlugin;

impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (place_base_on_left_click, place_overlay_on_right_click));
    }
}


/**
 * Places a base tile on left-click unless a pipe tool is active (pipe tools own left-drag).
 * Consumes high-level gameplay input instead of raw inputs.
 */
fn place_base_on_left_click(
    gi: Res<GameplayInputState>,
    mut map: ResMut<MapState>,
    tileset: Res<Tileset>,
    mut commands: Commands,
    mut q_base: Query<&mut TileStorage>,
    layers: Res<TilemapLayers>,
    grid: Res<GridConfig>,
) {
    if matches!(gi.selected_tool, InputTool::PipePlace | InputTool::PipeErase) { return }
    if !gi.left_just_pressed { return }
    if let Some(world) = gi.world_cursor {
        let local = Vec2::new(world.x, world.y);
        let map_size = TilemapSize { x: map.size.w, y: map.size.h };
        let grid_size = TilemapGridSize { x: grid.tile_size, y: grid.tile_size };
        let tile_size = TilemapTileSize { x: grid.tile_size, y: grid.tile_size };
        let tp = TilePos::from_world_pos(&local, &map_size, &grid_size, &tile_size, &TilemapType::Square, &TilemapAnchor::TopLeft);
        let Some(tp) = tp else { return };
        map.set_base(tp.x, tp.y, TileId::Dirt);
        let mut storage = q_base.get_mut(layers.base).unwrap();
        let color = tileset.def(TileId::Dirt).color;
        set_tile_in_tilemap(&mut commands, &mut storage, layers.base, color, tp.x, tp.y);
    }
}

/**
 * Places an overlay marker on right-click unless a pipe tool is active.
 * Consumes high-level gameplay input instead of raw inputs.
 */
fn place_overlay_on_right_click(
    gi: Res<GameplayInputState>,
    mut map: ResMut<MapState>,
    tileset: Res<Tileset>,
    mut commands: Commands,
    mut q_overlay: Query<&mut TileStorage>,
    layers: Res<TilemapLayers>,
    grid: Res<GridConfig>,
) {
    if matches!(gi.selected_tool, InputTool::PipePlace | InputTool::PipeErase) { return }
    if !gi.right_just_pressed { return }
    if let Some(world) = gi.world_cursor {
        let local = Vec2::new(world.x, world.y);
        let map_size = TilemapSize { x: map.size.w, y: map.size.h };
        let grid_size = TilemapGridSize { x: grid.tile_size, y: grid.tile_size };
        let tile_size = TilemapTileSize { x: grid.tile_size, y: grid.tile_size };
        let tp = TilePos::from_world_pos(&local, &map_size, &grid_size, &tile_size, &TilemapType::Square, &TilemapAnchor::TopLeft);
        let Some(tp) = tp else { return };
        map.set_overlay(tp.x, tp.y, Some(TileId::Marker));
        let mut storage = q_overlay.get_mut(layers.overlay).unwrap();
        let color = tileset.def(TileId::Marker).color;
        set_tile_in_tilemap(&mut commands, &mut storage, layers.overlay, color, tp.x, tp.y);
    }
}
