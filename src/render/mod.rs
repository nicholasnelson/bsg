use bevy::prelude::*;

pub mod overlay;
pub mod sync;
pub mod tilemaps;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((overlay::DebugGridPlugin, tilemaps::GameTilemapsPlugin))
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
