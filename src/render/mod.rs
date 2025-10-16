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
            .add_systems(Update, (camera_zoom_with_wheel, camera_pan_with_wasd));
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

fn camera_pan_with_wasd(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q_cam: Query<(&mut Transform, &Projection), With<Camera2d>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    if dir == Vec2::ZERO { return }
    let dir = dir.normalize();
    for (mut tf, proj) in &mut q_cam {
        let scale = match proj { Projection::Orthographic(o) => o.scale, _ => 1.0 };
        let speed = 400.0 * scale;
        let delta = dir * speed * time.delta_secs();
        tf.translation.x += delta.x;
        tf.translation.y += delta.y;
    }
}
