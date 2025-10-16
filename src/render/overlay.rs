use bevy::prelude::*;
use crate::core::grid::{GridConfig, DebugGridConfig};
use crate::core::map::MapState;
// use crate::render::sync::world_from_grid;

#[derive(Component)]
struct DebugGridRoot;

pub struct DebugGridPlugin;

impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<DebugGridConfig>(Default::default())
            .add_systems(Startup, setup_debug_grid)
            .add_systems(Update, sync_debug_grid_visibility);
    }
}

fn setup_debug_grid(mut commands: Commands, map: Res<MapState>, grid: Res<GridConfig>, dbg: Res<DebugGridConfig>) {
    let root = commands.spawn((Name::new("DebugGrid"), DebugGridRoot, Visibility::Hidden, Transform::default(), GlobalTransform::default())).id();

    let w = map.size.w;
    let h = map.size.h;
    let ts = grid.tile_size;
    let color = dbg.color;
    let z = dbg.z_layer;
    let thickness = dbg.thickness;

    // Vertical lines (TopLeft-style: Y increases down, so center at -len/2)
    for x in 0..=w {
        let xw = x as f32 * ts;
        let len = h as f32 * ts;
        commands.entity(root).with_children(|p| {
            p.spawn((
                Sprite { color, custom_size: Some(Vec2::new(thickness, len)), ..Default::default() },
                Transform::from_translation(Vec3::new(xw, -len / 2.0, z)),
                GlobalTransform::default(),
                Visibility::Visible,
                Name::new("GridV"),
            ));
        });
    }

    // Horizontal lines (TopLeft-style: translate negative in Y)
    for y in 0..=h {
        let yw = y as f32 * ts;
        let len = w as f32 * ts;
        commands.entity(root).with_children(|p| {
            p.spawn((
                Sprite { color, custom_size: Some(Vec2::new(len, thickness)), ..Default::default() },
                Transform::from_translation(Vec3::new(len / 2.0, -yw, z)),
                GlobalTransform::default(),
                Visibility::Visible,
                Name::new("GridH"),
            ));
        });
    }
}

fn sync_debug_grid_visibility(mut roots: Query<&mut Visibility, With<DebugGridRoot>>, dbg: Res<DebugGridConfig>) {
    if !dbg.is_changed() { return }
    if let Ok(mut vis) = roots.single_mut() {
        *vis = if dbg.enabled { Visibility::Visible } else { Visibility::Hidden };
    }
}

