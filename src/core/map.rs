use bevy::prelude::*;
use crate::core::tile::TileId;

#[derive(Clone, Copy)]
pub struct MapSize { pub w: u32, pub h: u32 }

#[derive(Resource)]
pub struct MapState {
    pub size: MapSize,
    pub base: Vec<TileId>,
    pub overlay: Vec<Option<TileId>>, // optional overlay marker
}
impl MapState {
    pub fn new(size: MapSize) -> Self {
        let num = (size.w * size.h) as usize;
        Self {
            size,
            base: vec![TileId::Empty; num],
            overlay: vec![None; num],
        }
    }

    pub fn idx(&self, x: u32, y: u32) -> usize { (y * self.size.w + x) as usize }

    pub fn get_base(&self, x: u32, y: u32) -> TileId { self.base[self.idx(x, y)] }
    pub fn set_base(&mut self, x: u32, y: u32, tile: TileId) {
        let idx = self.idx(x, y);
        self.base[idx] = tile;
    }

    pub fn get_overlay(&self, x: u32, y: u32) -> Option<TileId> { self.overlay[self.idx(x, y)] }
    pub fn set_overlay(&mut self, x: u32, y: u32, tile: Option<TileId>) {
        let idx = self.idx(x, y);
        self.overlay[idx] = tile;
    }
}