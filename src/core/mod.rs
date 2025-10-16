pub mod grid;
pub mod map;
pub mod tile;
pub mod catalog;

use bevy::prelude::*;
use map::{MapSize, MapState};
use tile::Tileset;
use catalog::CoreTilesPlugin;
use grid::GridConfig;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Default map and tileset resources
        app.add_plugins(CoreTilesPlugin)
            .insert_resource::<GridConfig>(Default::default())
            .init_resource::<Tileset>()
            .insert_resource(MapState::new(MapSize { w: 32, h: 20 }));
    }
}
