use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

pub mod overlay;
pub mod sync;
pub mod tilemaps;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((overlay::DebugGridPlugin, tilemaps::GameTilemapsPlugin))
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_zoom_with_wheel);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn camera_zoom_with_wheel(
    mut wheel: MessageReader<MouseWheel>,
    mut q_proj: Query<&mut Projection, With<Camera2d>>,
) {
    let mut scroll_y = 0.0;
    for ev in wheel.read() { scroll_y += ev.y; }
    if scroll_y == 0.0 { return }
    let factor = 1.0 - scroll_y * 0.1;
    for mut proj in &mut q_proj {
        if let Projection::Orthographic(ref mut ortho) = *proj {
            ortho.scale = (ortho.scale * factor).clamp(0.25, 8.0);
        }
    }
}
