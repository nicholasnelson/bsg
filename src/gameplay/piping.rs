/**
 * Piping gameplay systems: tool selection, drag-to-build/erase, and connectivity-based rendering.
 * Option A: two tilemaps for pipes (normal and engineering), with visibility toggled via input.
 */
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::core::map::MapState;
use crate::render::tilemaps::TilemapLayers;
use crate::render::sync::{set_tile_with_index, remove_tile_in_tilemap};
use crate::input::{GameplayInputState, Tool as InputTool};
use crate::core::grid::GridConfig;

/** Tracks whether the player is dragging a pipe path and the last visited tile. */
#[derive(Resource, Default)]
pub struct PipeDragState { pub dragging: bool, pub last: Option<TilePos> }

/** Pipe occupancy and connectivity mask per cell (same dimensions as the map). */
#[derive(Resource)]
pub struct PipeMap { pub present: Vec<bool>, pub mask: Vec<u8> }

impl PipeMap {
    pub fn new(size: (u32, u32)) -> Self {
        let (w, h) = size; let n = (w * h) as usize; Self { present: vec![false; n], mask: vec![0; n] }
    }
}

impl PipeMap {
    fn idx(&self, map: &MapState, x: u32, y: u32) -> usize { map.idx(x, y) }
    fn has(&self, map: &MapState, x: u32, y: u32) -> bool { self.present[self.idx(map, x, y)] }
    fn set(&mut self, map: &MapState, x: u32, y: u32, val: bool) { let i = self.idx(map, x, y); self.present[i] = val; }
    fn set_mask(&mut self, map: &MapState, x: u32, y: u32, m: u8) { let i = self.idx(map, x, y); self.mask[i] = m; }
    fn get_mask(&self, map: &MapState, x: u32, y: u32) -> u8 { self.mask[self.idx(map, x, y)] }
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipeDragState>()
            .add_systems(Startup, init_pipemap)
            .add_systems(Update, (drag_from_input, apply_connectivity_and_tiles));
    }
}

/** Initializes the PipeMap resource to match the current map size. */
fn init_pipemap(mut commands: Commands, map: Res<MapState>) {
    commands.insert_resource(PipeMap::new((map.size.w, map.size.h)));
}

/**
 * Uses collected input state to start/stop drags and write pipe presence along the drag path.
 * Supports straight lines and simple L-turns.
 */
fn drag_from_input(
    gi: Res<GameplayInputState>,
    mut drag: ResMut<PipeDragState>,
    mut pipemap: ResMut<PipeMap>,
    map: Res<MapState>,
    grid: Res<GridConfig>,
) {
    let placing_mode = matches!(gi.selected_tool, InputTool::PipePlace | InputTool::PipeErase);
    if !placing_mode { drag.dragging = false; drag.last = None; return }

    if gi.left_just_pressed { drag.dragging = true; drag.last = None; }
    if gi.left_just_released { drag.dragging = false; drag.last = None; }
    if !drag.dragging || !gi.left_pressed { return }

    let Some(world) = gi.world_cursor else { return };
    let local = Vec2::new(world.x, world.y);
    let map_size = TilemapSize { x: map.size.w, y: map.size.h };
    let grid_size = TilemapGridSize { x: grid.tile_size, y: grid.tile_size };
    let tile_size = TilemapTileSize { x: grid.tile_size, y: grid.tile_size };
    let Some(tp) = TilePos::from_world_pos(&local, &map_size, &grid_size, &tile_size, &TilemapType::Square, &TilemapAnchor::TopLeft) else { return };
    let Some(last) = drag.last else { drag.last = Some(tp); return };
    if last == tp { return }

    let placing = matches!(gi.selected_tool, InputTool::PipePlace);
    let mut fill_straight = |from: TilePos, to: TilePos, val: bool| {
        if from.x == to.x {
            let x = from.x; let (a, b) = if from.y <= to.y { (from.y, to.y) } else { (to.y, from.y) };
            for y in a..=b { pipemap.set(&map, x, y, val); }
        } else if from.y == to.y {
            let y = from.y; let (a, b) = if from.x <= to.x { (from.x, to.x) } else { (to.x, from.x) };
            for x in a..=b { pipemap.set(&map, x, y, val); }
        }
    };

    if last.x == tp.x || last.y == tp.y {
        fill_straight(last, tp, placing);
    } else {
        let mid = TilePos { x: tp.x, y: last.y };
        fill_straight(last, mid, placing);
        fill_straight(mid, tp, placing);
    }
    drag.last = Some(tp);
}

/**
 * Recomputes NESW connectivity for all cells and updates both pipe tilemaps.
 * For now, the connectivity mask is used directly as the tile texture index.
 */
fn apply_connectivity_and_tiles(
    mut commands: Commands,
    layers: Res<TilemapLayers>,
    mut q_sets: ParamSet<(Query<&mut TileStorage>, Query<&mut TileStorage>)>,
    pipemap: ResMut<PipeMap>,
    map: Res<MapState>,
) {
    if !pipemap.is_changed() { return }
    let w = map.size.w; let h = map.size.h;
    let inside = |x: i32, y: i32| x >= 0 && y >= 0 && (x as u32) < w && (y as u32) < h;
    let has = |x: i32, y: i32| inside(x, y) && pipemap.present[map.idx(x as u32, y as u32)];

    let mut ops: Vec<(u32, u32, Option<u32>)> = Vec::with_capacity((w * h) as usize);
    for y in 0..h { for x in 0..w {
        let present = pipemap.present[map.idx(x, y)];
        if !present {
            ops.push((x, y, None));
        } else {
            let n = if has(x as i32, y as i32 - 1) { 1 } else { 0 };
            let e = if has(x as i32 + 1, y as i32) { 2 } else { 0 };
            let s = if has(x as i32, y as i32 + 1) { 4 } else { 0 };
            let wv = if has(x as i32 - 1, y as i32) { 8 } else { 0 };
            let mask = (n | e | s | wv) as u32;
            ops.push((x, y, Some(mask)));
        }
    }}

    {
        let mut q = q_sets.p0();
        let mut storage = q.get_mut(layers.pipes).unwrap();
        for (x, y, mask_opt) in ops.iter().copied() {
            match mask_opt {
                Some(mask) => set_tile_with_index(&mut commands, &mut storage, layers.pipes, mask, Color::WHITE, x, y),
                None => remove_tile_in_tilemap(&mut commands, &mut storage, x, y),
            }
        }
    }

    {
        let mut q = q_sets.p1();
        let mut storage_eng = q.get_mut(layers.pipes_eng).unwrap();
        for (x, y, mask_opt) in ops.into_iter() {
            match mask_opt {
                Some(mask) => set_tile_with_index(&mut commands, &mut storage_eng, layers.pipes_eng, mask, Color::srgb(0.6, 0.9, 1.0), x, y),
                None => remove_tile_in_tilemap(&mut commands, &mut storage_eng, x, y),
            }
        }
    }
}


