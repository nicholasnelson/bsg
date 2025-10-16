pub mod grid;
pub mod map;
pub mod tile;

use bevy::prelude::*;
use map::{MapSize, MapState};
use tile::Tileset;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Default map and tileset resources
        app.init_resource::<Tileset>()
            .insert_resource(MapState::new(MapSize { w: 32, h: 20 }));
    }
}
