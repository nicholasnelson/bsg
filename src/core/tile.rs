use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileId { Empty, Dirt, Marker }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileLayer { Base, Overlay }

pub struct TileDef {
    pub id: TileId,
    pub layer: TileLayer,
    pub color: Color,
}

#[derive(Resource)]
pub struct Tileset {
    pub defs: Vec<TileDef>,
}

impl Default for Tileset {
    fn default() -> Self {
        Self {
            defs: vec![
                TileDef { id: TileId::Empty, layer: TileLayer::Base, color: Color::NONE },
                TileDef { id: TileId::Dirt, layer: TileLayer::Base, color: Color::srgb(0.55, 0.42, 0.35) },
                TileDef { id: TileId::Marker, layer: TileLayer::Overlay, color: Color::srgb(1.0, 1.0, 0.0) },
            ],
        }
    }
}

impl Tileset {
    pub fn def(&self, id: TileId) -> &TileDef {
        self.defs.iter().find(|def| def.id == id).unwrap()
    }
}