use bevy::asset::{io::Reader, Asset, AssetLoader, LoadContext};
use bevy::prelude::*;
use serde::Deserialize;

/**
 * Runtime tile catalog asset loaded from JSON files.
 * Example JSON:
 * [
 *   { "id": "Dirt", "layer": "Base", "color": [0.55, 0.42, 0.35, 1.0] },
 *   { "id": "Marker", "layer": "Overlay", "color": [1.0, 1.0, 0.0, 1.0] }
 * ]
 */
#[derive(Asset, TypePath, Clone, Default)]
pub struct TileCatalog {
    pub defs: Vec<TileCatalogEntry>,
}

#[derive(Clone, Deserialize)]
pub struct TileCatalogEntry {
    pub id: String,
    pub layer: String,
    pub color: [f32; 4],
}

#[derive(Default)]
pub struct TileCatalogLoader;

impl AssetLoader for TileCatalogLoader {
    type Asset = TileCatalog;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let defs: Vec<TileCatalogEntry> = serde_json::from_slice(&bytes)?;
        Ok(TileCatalog { defs })
    }

    fn extensions(&self) -> &[&str] { &["json"] }
}

pub struct CoreTilesPlugin;

impl Plugin for CoreTilesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TileCatalog>()
            .init_asset_loader::<TileCatalogLoader>();
    }
}


