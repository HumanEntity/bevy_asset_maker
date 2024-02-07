#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! create_asset {
    (
        $mod_name:ident;
        $($derive:ident,)*;
        $asset_name:ident
        $(
            $field_name:ident : $field_type: ty $( = $handle_path:ident)?,
        )*;
        $asset_plugin:ident,
        $asset_loader:ident,
        $extensions:expr,
    ) => {
        #[allow(unused_imports)]
        use $mod_name::{$asset_name, $asset_plugin};
        mod $mod_name {
            use bevy::prelude::*;
            use bevy::asset::ReflectAsset;
            #[allow(unused_imports)]
            use super::*;
            #[derive(serde::Deserialize, bevy::asset::Asset, bevy::prelude::Reflect)]
            #[derive($($derive ,)*)]
            #[reflect(Asset)]
            pub struct $asset_name {
                $(
                    $(
                        pub $handle_path: String,
                        #[serde(skip)]
                    )?
                    pub $field_name : $field_type,
                )*
            }

            pub struct $asset_plugin;
            impl bevy::prelude::Plugin for $asset_plugin {
                fn build(&self, app: &mut bevy::prelude::App) {
                    app.register_type::<$asset_name>()
                        .register_asset_reflect::<$asset_name>()
                        .init_asset::<$asset_name>()
                        .init_asset_loader::<$asset_loader>()
                        .add_systems(bevy::prelude::Update, finalize_asset);
                }
            }

            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn finalize_asset(
                mut asset_events: EventReader<AssetEvent<$asset_name>>,
                mut assets: ResMut<Assets<$asset_name>>,
                asset_server: Res<bevy::prelude::AssetServer>,
                ) {
                for event in asset_events.read() {
                    match event {
                        AssetEvent::Added { id } => {
                            if let Some(asset) = assets.get_mut(*id) {
                                $(
                                    $(
                                        asset.$field_name = asset_server.load(&asset.$handle_path);
                                    )?
                                )*
                            }
                        }
                        _ => ()
                    }
                }
            }

            #[derive(Default)]
            pub struct $asset_loader;
            impl bevy::asset::AssetLoader for $asset_loader {
                type Asset = $asset_name;
                type Settings = ();
                type Error = ron::de::Error;
                fn load<'a>(
                        &'a self,
                        reader: &'a mut bevy::asset::io::Reader,
                        _settings: &'a Self::Settings,
                        _load_context: &'a mut bevy::asset::LoadContext,
                    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
                    use bevy::asset::AsyncReadExt;
                    Box::pin(async move {
                        let mut bytes = Vec::new();
                        reader.read_to_end(&mut bytes).await?;
                        let custom_asset = ron::de::from_bytes::<$asset_name>(&bytes)?;
                        Ok(custom_asset)
                    })
                }

                fn extensions(&self) -> &[&str] {
                    $extensions
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    create_asset!(
        damage;
        Clone,;
        Damage
        damage : i32,;
        DamageAssetPlugin,
        DamageAssetLoader,
        &["damage.ron"],
    );

    create_asset!(
        asset;
        Debug, Clone,;
        SimpleAsset
        damage : Handle<Damage> = damage_path,;
        SimpleAssetPlugin,
        SimpleAssetLoader,
        &["simple.ron"],
    );
}
