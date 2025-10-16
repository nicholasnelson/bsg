pub mod placement;
pub mod rules;

use bevy::prelude::*;
use crate::core::CorePlugin;
use crate::render::sync::TileSyncPlugin;
use crate::render::tilemaps::TilemapPlugin;
use placement::PlacementPlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CorePlugin, TilemapPlugin, TileSyncPlugin, PlacementPlugin));
    }
}
