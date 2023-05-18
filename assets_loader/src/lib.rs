mod credit;
mod json;

use std::marker::PhantomData;

use bevy::{
    asset::Asset,
    prelude::{AddAsset, App, Plugin},
};

pub use credit::{AssetCredit, AssetType, Credits};
use serde::Deserialize;

#[non_exhaustive]
enum Format {
    Json,
}

pub struct LoaderPlugin<T>(Format, PhantomData<T>);

impl<T> Plugin for LoaderPlugin<T>
where
    for<'de> T: Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        match self.0 {
            Format::Json => {
                app.add_asset::<T>()
                    .add_asset_loader(json::JsonLoader::<T>(PhantomData));
            }
        }
    }
}

impl<T> LoaderPlugin<T> {
    pub fn json() -> Self {
        Self(Format::Json, PhantomData)
    }
}
