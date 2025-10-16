use bevy::prelude::*;
use crate::input::CameraInputState;

pub mod overlay;
pub mod sync;
pub mod tilemaps;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((overlay::DebugGridPlugin, tilemaps::GameTilemapsPlugin))
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (apply_input_zoom, apply_input_pan, apply_input_toggle_overlay, apply_input_toggle_engineering));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn apply_input_zoom(mut state: ResMut<CameraInputState>, mut q_proj: Query<&mut Projection, With<Camera2d>>) {
    if (state.zoom_factor - 1.0).abs() < f32::EPSILON { return }
    for mut proj in &mut q_proj {
        if let Projection::Orthographic(ref mut ortho) = *proj {
            ortho.scale = (ortho.scale * state.zoom_factor).clamp(0.25, 8.0);
        }
    }
    state.zoom_factor = 1.0;
}

fn apply_input_pan(mut state: ResMut<CameraInputState>, mut q_cam: Query<(&mut Transform, &Projection), With<Camera2d>>) {
    if state.pan_delta == Vec2::ZERO { return }
    for (mut tf, proj) in &mut q_cam {
        let scale = match proj { Projection::Orthographic(o) => o.scale, _ => 1.0 };
        let delta = state.pan_delta * scale;
        tf.translation.x += delta.x;
        tf.translation.y += delta.y;
    }
    state.pan_delta = Vec2::ZERO;
}

/**
 * Toggles visibility of the generic overlay tilemap when the input flag is set.
 *
 * @param state - input state containing the one-shot toggle flag
 * @param layers - tilemap entities to modify
 * @param q_vis - query to access and mutate visibility on entities
 */
fn apply_input_toggle_overlay(mut state: ResMut<CameraInputState>, layers: Res<tilemaps::TilemapLayers>, mut q_vis: Query<&mut Visibility>) {
    if !state.toggle_overlay { return }
    let Ok(mut vis) = q_vis.get_mut(layers.overlay) else { return };
    *vis = match *vis {
        Visibility::Visible => Visibility::Hidden,
        Visibility::Hidden => Visibility::Visible,
        Visibility::Inherited => Visibility::Visible,
    };
    state.toggle_overlay = false;
}

/**
 * Switches between normal and engineering views for pipes by toggling the two pipe tilemaps.
 *
 * @param state - input state containing the one-shot engineering toggle flag
 * @param layers - tilemap entities for normal and engineering pipe views
 * @param q_vis - query to access and mutate visibility on entities
 */
fn apply_input_toggle_engineering(mut state: ResMut<CameraInputState>, layers: Res<tilemaps::TilemapLayers>, mut q_vis: Query<&mut Visibility>) {
    if !state.toggle_engineering { return }
    // First fetch and compute desired vis
    let pipes_visible = {
        if let Ok(vis_pipes) = q_vis.get_mut(layers.pipes) {
            matches!(*vis_pipes, Visibility::Visible | Visibility::Inherited)
        } else { return }
    };
    // Now apply to both in separate borrows
    if let Ok(mut vis_pipes) = q_vis.get_mut(layers.pipes) {
        *vis_pipes = if pipes_visible { Visibility::Hidden } else { Visibility::Visible };
    }
    if let Ok(mut vis_eng) = q_vis.get_mut(layers.pipes_eng) {
        *vis_eng = if pipes_visible { Visibility::Visible } else { Visibility::Hidden };
    }
    state.toggle_engineering = false;
}
