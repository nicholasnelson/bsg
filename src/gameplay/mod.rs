pub mod placement;
pub mod rules;
pub mod piping;

use bevy::prelude::*;
use crate::render::sync::TileSyncPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use placement::PlacementPlugin;
use piping::PipePlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilemapPlugin, TileSyncPlugin, PlacementPlugin, PipePlugin));
    }
}
