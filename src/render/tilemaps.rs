use bevy::prelude::*;
use bevy::render::render_resource::{TextureFormat, Extent3d, TextureDimension};
use bevy::asset::RenderAssetUsages;
use bevy_ecs_tilemap::prelude::*;
use crate::core::map::MapState;
use crate::core::grid::GridConfig;

/**
 * Groups the tilemap entity IDs for each render layer so systems can find and update them.
 * Layers:
 * - base: terrain/background
 * - overlay: general markers/UI tiles
 * - pipes: normal view for pipes
 * - pipes_eng: engineering view for pipes (toggled visible in engineering mode)
 */
#[derive(Resource)]
pub struct TilemapLayers { pub base: Entity, pub overlay: Entity, pub pipes: Entity, pub pipes_eng: Entity }

// Removed TilemapParams; gameplay now converts world->grid via core GridConfig

pub struct GameTilemapsPlugin;
impl Plugin for GameTilemapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tilemaps);
    }
}

/**
 * Creates all tilemap layers (base, overlay, pipes, pipes_eng) with consistent sizing and grid params.
 * The engineering pipes layer starts Hidden; we toggle between pipes and pipes_eng at runtime.
 *
 * @param commands - ECS command buffer for spawning entities/resources
 * @param images - asset store used to create a placeholder tile texture
 * @param map - current map state to size the tilemaps
 * @param grid - grid configuration (tile size, etc.)
 */
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
    let base_storage = TileStorage::empty(map_size);
    let base_entity = commands.spawn_empty().id();
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
    let overlay_storage = TileStorage::empty(map_size);
    let overlay_entity = commands.spawn_empty().id();
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

    // Pipes (normal view) layer
    let pipes_storage = TileStorage::empty(map_size);
    let pipes_entity = commands.spawn_empty().id();
    commands.entity(pipes_entity).insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: pipes_storage.clone(),
            texture: TilemapTexture::Single(tex_handle.clone()),
            tile_size,
            map_type,
            anchor,
            transform: Transform::default(),
            ..Default::default()
        },
        Name::new("Pipes"),
    ));

    // Pipes (engineering view) layer
    let pipes_eng_storage = TileStorage::empty(map_size);
    let pipes_eng_entity = commands.spawn_empty().id();
    commands.entity(pipes_eng_entity).insert((
        TilemapBundle {
            grid_size,
            size: map_size,
            storage: pipes_eng_storage.clone(),
            texture: TilemapTexture::Single(tex_handle.clone()),
            tile_size,
            map_type,
            anchor,
            transform: Transform::default(),
            ..Default::default()
        },
        Name::new("PipesEngineering"),
    ));
    // Set initial visibility after inserting the bundle to avoid duplicate Visibility in the same bundle
    commands.entity(pipes_eng_entity).insert(Visibility::Hidden);

    commands.insert_resource(TilemapLayers { base: base_entity, overlay: overlay_entity, pipes: pipes_entity, pipes_eng: pipes_eng_entity });
}