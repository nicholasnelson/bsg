use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
// Input should not depend on render/tilemaps; emit world cursor instead

#[derive(Resource)]
pub struct CameraInputState {
    pub zoom_factor: f32,
    pub pan_delta: Vec2,
    pub toggle_overlay: bool,
    pub toggle_engineering: bool,
}

impl Default for CameraInputState {
    fn default() -> Self { Self { zoom_factor: 1.0, pan_delta: Vec2::ZERO, toggle_overlay: false, toggle_engineering: false } }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraInputState>()
            .init_resource::<GameplayInputState>()
            .add_systems(Update, (
                collect_wheel_zoom,
                collect_wasd_pan,
                toggle_overlay_on_backspace,
                toggle_engineering_on_key,
                collect_tool_keys,
                collect_pointer_actions,
            ));
    }
}

fn collect_wheel_zoom(mut wheel: MessageReader<MouseWheel>, mut state: ResMut<CameraInputState>) {
    let mut scroll_y = 0.0;
    for ev in wheel.read() { scroll_y += ev.y; }
    if scroll_y == 0.0 { return }
    let factor = 1.0 - scroll_y * 0.1;
    state.zoom_factor *= factor;
}

fn collect_wasd_pan(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut state: ResMut<CameraInputState>) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    if dir == Vec2::ZERO { return }
    let delta = dir.normalize() * 400.0 * time.delta_secs();
    state.pan_delta += delta;
}

fn toggle_overlay_on_backspace(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<CameraInputState>) {
    if keys.just_pressed(KeyCode::Backspace) { state.toggle_overlay = true; }
}

/**
 * Collects the engineering-view toggle (KeyE) into input state; render systems consume this flag.
 *
 * @param keys - keyboard input
 * @param state - input state to set one-shot toggle flag
 */
fn toggle_engineering_on_key(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<CameraInputState>) {
    if keys.just_pressed(KeyCode::KeyE) { state.toggle_engineering = true; }
}

/** Player tool modes for gameplay interactions. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tool { None, PipePlace, PipeErase }

/**
 * Transient gameplay input derived from raw inputs each frame.
 * Gameplay systems consume these fields and may reset the one-shot flags to false.
 */
#[derive(Resource)]
pub struct GameplayInputState {
    pub selected_tool: Tool,
    pub left_just_pressed: bool,
    pub left_pressed: bool,
    pub left_just_released: bool,
    pub right_just_pressed: bool,
    pub right_pressed: bool,
    pub right_just_released: bool,
    pub world_cursor: Option<Vec2>,
}

impl Default for GameplayInputState {
    fn default() -> Self {
        Self {
            selected_tool: Tool::None,
            left_just_pressed: false,
            left_pressed: false,
            left_just_released: false,
            right_just_pressed: false,
            right_pressed: false,
            right_just_released: false,
            world_cursor: None,
        }
    }
}

/** Handles keybinds for selecting gameplay tools. */
fn collect_tool_keys(keys: Res<ButtonInput<KeyCode>>, mut gi: ResMut<GameplayInputState>) {
    if keys.just_pressed(KeyCode::KeyP) { gi.selected_tool = Tool::PipePlace; }
    if keys.just_pressed(KeyCode::KeyO) { gi.selected_tool = Tool::PipeErase; }
    if keys.just_pressed(KeyCode::Escape) { gi.selected_tool = Tool::None; }
}

/**
 * Produces per-frame pointer actions (left/right pressed/released) and current world cursor position.
 */
fn collect_pointer_actions(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut gi: ResMut<GameplayInputState>,
) {
    gi.left_just_pressed = buttons.just_pressed(MouseButton::Left);
    gi.left_pressed = buttons.pressed(MouseButton::Left);
    gi.left_just_released = buttons.just_released(MouseButton::Left);

    gi.right_just_pressed = buttons.just_pressed(MouseButton::Right);
    gi.right_pressed = buttons.pressed(MouseButton::Right);
    gi.right_just_released = buttons.just_released(MouseButton::Right);

    let Ok(window) = windows.single() else { gi.world_cursor = None; return };
    let Ok((cam_tf, cam)) = camera_q.single() else { gi.world_cursor = None; return };
    let Some(cursor_pos) = window.cursor_position() else { gi.world_cursor = None; return };
    let Ok(world_2d) = cam.viewport_to_world_2d(cam_tf, cursor_pos) else { gi.world_cursor = None; return };
    gi.world_cursor = Some(world_2d);
}


