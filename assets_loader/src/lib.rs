mod credit;
mod json;

use bevy::prelude::{AddAsset, App, Plugin};

pub use credit::{AssetCredit, AssetType, Credits};

#[non_exhaustive]
enum Format {
    Json,
}

pub struct LoaderPlugin(Format);

impl Plugin for LoaderPlugin
{
    fn build(&self, app: &mut App) {
        match self.0 {
            Format::Json => {
                app.add_asset_loader(json::JsonLoader);
            }
        }
    }
}

impl LoaderPlugin {
    pub fn json() -> Self {
        Self(Format::Json)
    }
}
