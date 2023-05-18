//! Json file support for assets

use std::marker::PhantomData;

use bevy::asset::{Asset, AssetLoader, LoadContext, LoadedAsset};
use bevy::utils::BoxedFuture;
use serde::Deserialize;

pub(crate) struct JsonLoader<T>(pub(crate) PhantomData<T>);

impl<T> AssetLoader for JsonLoader<T>
where
    for<'de> T: Deserialize<'de> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = serde_json::from_slice::<T>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
