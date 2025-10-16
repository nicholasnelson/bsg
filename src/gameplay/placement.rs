use bevy::prelude::*;
use crate::core::map::MapState;
use crate::core::tile::{TileId, Tileset};
use bevy_ecs_tilemap::prelude::*;
use crate::render::tilemaps::{TilemapLayers, TilemapParams};
use crate::render::sync::set_tile_in_tilemap;

pub struct PlacementPlugin;

impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (place_base_on_left_click, place_overlay_on_right_click));
    }
}

fn cursor_to_tilepos(window: &Window, camera: (&GlobalTransform, &Camera), map_tf: &Transform, params: &TilemapParams) -> Option<TilePos> {
    let (camera_transform, camera) = camera;
    let Some(cursor_pos) = window.cursor_position() else { return None };
    let Ok(world_2d) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return None };
    let inv = map_tf.to_matrix().inverse();
    let local3 = inv.mul_vec4(Vec4::new(world_2d.x, world_2d.y, 0.0, 1.0)).truncate();
    let local2 = Vec2::new(local3.x, local3.y);
    TilePos::from_world_pos(&local2, &params.size, &params.grid_size, &params.tile_size, &params.map_type, &params.anchor)
}

fn place_base_on_left_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut map: ResMut<MapState>,
    tileset: Res<Tileset>,
    mut commands: Commands,
    mut q_base: Query<&mut TileStorage>,
    layers: Res<TilemapLayers>,
    params: Res<TilemapParams>,
    q_tf: Query<&Transform>,
) {
    if !buttons.just_pressed(MouseButton::Left) { return }
    let Ok(window) = windows.single() else { return };
    let Ok(camera) = camera_q.single() else { return };
    let Ok(map_tf) = q_tf.get(layers.base) else { return };
    if let Some(tp) = cursor_to_tilepos(window, camera, map_tf, &params) {
        map.set_base(tp.x, tp.y, TileId::Dirt);
        let mut storage = q_base.get_mut(layers.base).unwrap();
        let color = tileset.def(TileId::Dirt).color;
        set_tile_in_tilemap(&mut commands, &mut storage, layers.base, color, tp.x, tp.y);
    }
}

fn place_overlay_on_right_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut map: ResMut<MapState>,
    tileset: Res<Tileset>,
    mut commands: Commands,
    mut q_overlay: Query<&mut TileStorage>,
    layers: Res<TilemapLayers>,
    params: Res<TilemapParams>,
    q_tf: Query<&Transform>,
) {
    if !buttons.just_pressed(MouseButton::Right) { return }
    let Ok(window) = windows.single() else { return };
    let Ok(camera) = camera_q.single() else { return };
    let Ok(map_tf) = q_tf.get(layers.base) else { return };
    if let Some(tp) = cursor_to_tilepos(window, camera, map_tf, &params) {
        map.set_overlay(tp.x, tp.y, Some(TileId::Marker));
        let mut storage = q_overlay.get_mut(layers.overlay).unwrap();
        let color = tileset.def(TileId::Marker).color;
        set_tile_in_tilemap(&mut commands, &mut storage, layers.overlay, color, tp.x, tp.y);
    }
}
