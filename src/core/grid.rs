use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct GridConfig {
    pub tile_size: f32,
}

impl Default for GridConfig {
    fn default() -> Self { Self { tile_size: 16.0 } }
}

#[derive(Resource, Clone, Copy)]
pub struct DebugGridConfig {
    pub enabled: bool,
    pub color: Color,
    pub thickness: f32,
    pub z_layer: f32,
}

impl Default for DebugGridConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            color: Color::srgb(0.2, 0.8, 0.2),
            thickness: 1.0,
            z_layer: 50.0,
        }
    }
}

