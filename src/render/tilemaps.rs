use bevy::prelude::*;
use bevy::render::render_resource::{TextureFormat, Extent3d, TextureDimension};
use bevy::asset::RenderAssetUsages;
use bevy_ecs_tilemap::prelude::*;
use crate::core::map::MapState;
use crate::core::grid::GridConfig;

#[derive(Resource)]
pub struct TilemapLayers { pub base: Entity, pub overlay: Entity }

#[derive(Resource, Clone)]
pub struct TilemapParams {
    pub size: TilemapSize,
    pub grid_size: TilemapGridSize,
    pub tile_size: TilemapTileSize,
    pub map_type: TilemapType,
    pub anchor: TilemapAnchor,
}

pub struct GameTilemapsPlugin;
impl Plugin for GameTilemapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tilemaps);
    }
}

fn setup_tilemaps(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    map: Res<MapState>,
    grid: Res<GridConfig>,
) {
    let tile_px = grid.tile_size as u32;
    let white_tex = Image::new_fill(Extent3d { width: tile_px, height: tile_px, depth_or_array_layers: 1 }, TextureDimension::D2, &[255, 255, 255, 255], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD);
    let tex_handle = images.add(white_tex);

    let map_size = TilemapSize { x: map.size.w, y: map.size.h };
    let tile_size = TilemapTileSize { x: grid.tile_size, y: grid.tile_size };
    let grid_size = TilemapGridSize { x: grid.tile_size, y: grid.tile_size };
    let map_type = TilemapType::Square;
    let anchor = TilemapAnchor::TopLeft;

    // Base layer
    let mut base_storage = TileStorage::empty(map_size);
    let base_entity = commands.spawn_empty().id();
    let base_id = TilemapId(base_entity);
    commands.entity(base_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: base_storage.clone(),
        texture: TilemapTexture::Single(tex_handle.clone()),
        tile_size,
        map_type,
        anchor,
        transform: Transform::default(),
        ..Default::default()
    });

    // Overlay layer
    let mut overlay_storage = TileStorage::empty(map_size);
    let overlay_entity = commands.spawn_empty().id();
    let overlay_id = TilemapId(overlay_entity);
    commands.entity(overlay_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: overlay_storage.clone(),
        texture: TilemapTexture::Single(tex_handle.clone()),
        tile_size,
        map_type,
        anchor,
        transform: Transform::default(),
        ..Default::default()
    });

    commands.insert_resource(TilemapLayers { base: base_entity, overlay: overlay_entity });
    commands.insert_resource(TilemapParams { size: map_size, grid_size, tile_size, map_type, anchor });
}