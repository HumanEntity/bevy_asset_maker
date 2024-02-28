use bevy::prelude::*;
use bevy_asset_maker::create_asset;

#[derive(Debug, Clone, serde::Deserialize, Asset, Reflect)]
pub struct Simple {
    x: i32,
}

#[derive(Debug, Clone, serde::Deserialize, Asset, Reflect)]
pub struct Example {
    x: i32,
    y_path: String,
    #[serde(skip)]
    y: Handle<Simple>,
}

create_asset!(ExamplePlugin, ExampleLoader, Example, &["example.ron"]; y_path -> y?;);

/**Hello*/
fn main() {}
