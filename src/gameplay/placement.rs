use bevy::prelude::*;
use crate::core::map::MapState;
use crate::core::tile::{TileId, Tileset};
use crate::render::sync::spawn_tile_sprite;

pub struct PlacementPlugin;

impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (place_base_on_left_click, place_overlay_on_right_click));
    }
}

fn cursor_to_grid_pos(window: &Window, camera: (&GlobalTransform, &Camera), map: &MapState) -> Option<(u32, u32)> {
    let (camera_transform, camera) = camera;
    let Some(cursor_pos) = window.cursor_position() else { return None };
    let Ok(world_2d) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return None };
    // Subtract half-tile to counter centering offset so clicks map to correct cell
    let gx = ((world_2d.x) / 16.0).floor() as i32;
    let gy = ((world_2d.y) / 16.0).floor() as i32;
    if gx < 0 || gy < 0 { return None }
    if gx >= map.size.w as i32 || gy >= map.size.h as i32 { return None }
    Some((gx as u32, gy as u32))
}

fn place_base_on_left_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut map: ResMut<MapState>,
    tileset: Res<Tileset>,
    mut commands: Commands,
) {
    if !buttons.just_pressed(MouseButton::Left) { return }
    let Ok(window) = windows.single() else { return };
    let Ok(camera) = camera_q.single() else { return };
    if let Some((x, y)) = cursor_to_grid_pos(window, camera, &map) {
        map.set_base(x, y, TileId::Dirt);
        spawn_tile_sprite(&mut commands, &tileset, TileId::Dirt, x, y);
    }
}

fn place_overlay_on_right_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut map: ResMut<MapState>,
    tileset: Res<Tileset>,
    mut commands: Commands,
) {
    if !buttons.just_pressed(MouseButton::Right) { return }
    let Ok(window) = windows.single() else { return };
    let Ok(camera) = camera_q.single() else { return };
    if let Some((x, y)) = cursor_to_grid_pos(window, camera, &map) {
        map.set_overlay(x, y, Some(TileId::Marker));
        spawn_tile_sprite(&mut commands, &tileset, TileId::Marker, x, y);
    }
}
