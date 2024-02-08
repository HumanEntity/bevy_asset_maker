#![doc = include_str!("../README.md")]

pub use serde;

#[macro_export]
macro_rules! create_asset {
    (
        $asset_plugin:ident,
        $asset_loader:ident,
        $asset_name:ident,
        $extensions:expr;
        $($handle_path:ident -> $field_name:ident)*?
        $($opt_handle_path:ident -> $opt_field_name:ident)*;
    ) => {
        use bevy::prelude::*;
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
                            $(
                                if let Some(path) = &asset.$opt_handle_path {
                                    asset.$opt_field_name = Some(asset_server.load(path));
                                }
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
        #[cfg(feature = "saver")]
        impl bevy::asset::saver::AssetSaver for $asset_loader {
            type Asset = $asset_name;
            type Settings = ();
            type Error = ron::Error;
            type OutputLoader = Self;
            fn save<'a>(
                &'a self,
                writer: &'a mut bevy::asset::io::Writer,
                asset: bevy::asset::saver::SavedAsset<'a, Self::Asset>,
                _settings: &'a Self::Settings
            ) -> core::pin::Pin<Box<dyn core::future::Future<Output = Result<<Self::OutputLoader as bevy::asset::AssetLoader>::Settings, Self::Error>> + Send + 'a>> {
                use bevy::asset::AsyncWriteExt;
                Box::pin(async move {
                    let asset = asset.get();
                    let string = ron::ser::to_string_pretty(asset, ron::ser::PrettyConfig::default())?;
                    let bytes = string.as_bytes();

                    writer.write(&bytes).await?;
                    Ok(())
                })
            }
        }
    };
    (
        $mod_name:ident;
        $($derive:ident,)*;
        $asset_name:ident
        $(
            $field_name:ident : $field_type: ty $( = $handle_path:ident)?,
        )*?
        $(
            $opt_field_name:ident : $opt_field_type: ty = $opt_handle_path:ident,
        )*;
        $asset_plugin:ident,
        $asset_loader:ident,
        $extensions:expr,
    ) =>{
        #[allow(unused_imports)]
        use $mod_name::{
            $asset_name,
            $asset_plugin,
        };
        mod $mod_name {
            use super::*;
            create_asset!(
                $($derive,)*;
                $asset_name
                $(
                    $field_name : $field_type $( = $handle_path )?,
                )*?
                $(
                    $opt_field_name : $opt_field_type = $opt_handle_path,
                )*;
                $asset_plugin,
                $asset_loader,
                $extensions,
            );
        }
    };
    (
        $($derive:ident,)*;
        $asset_name:ident
        $(
            $field_name:ident : $field_type: ty $( = $handle_path:ident)?,
        )*?
        $(
            $opt_field_name:ident : $opt_field_type: ty = $opt_handle_path:ident,
        )*;
        $asset_plugin:ident,
        $asset_loader:ident,
        $extensions:expr,
    ) => {
        use bevy::asset::ReflectAsset;
        #[allow(unused_imports)]
        #[derive(serde::Deserialize, bevy::asset::Asset, bevy::prelude::Reflect)]
        #[cfg_attr(feature = "saver", derive(serde::Serialize))]
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
            $(
                pub $opt_handle_path: Option<String>,
                #[serde(skip)]
                pub $opt_field_name : Option<$opt_field_type>,
            )*
        }
        create_asset!(
            $asset_plugin,
            $asset_loader,
            $asset_name,
            $extensions;
            $(
                $(
                    $handle_path -> $field_name
                )?
            )*?
            $(
                $opt_handle_path -> $opt_field_name
            )*;
        );
    };
}

#[cfg(test)]
mod test {
    use super::*;
    create_asset!(
        damage;
        Clone,;
        Damage
        damage : i32,?;
        DamageAssetPlugin,
        DamageAssetLoader,
        &["damage.ron"],
    );

    create_asset!(
        simple;
        Debug, Clone,;
        SimpleAsset
        ?
        damage : Handle<Damage> = damage_path,;
        SimpleAssetPlugin,
        SimpleAssetLoader,
        &["simple.ron"],
    );
}
