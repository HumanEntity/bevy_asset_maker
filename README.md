# Bevy asset maker

Utility for creating content assets

## Usage

```rust
// Import `bevy_asset_maker`
use bevy_asset_maker::*;

create_asset!(
    // Module, core functionality will be placed in
    damage;
    // Additional derives
    Debug, Clone, Copy,;
    // Asset name
    DamageAsset
    // Fields
    damage : f32,
    ;
    // Asset Plugin name
    DamageAssetPlugin,
    // Asset Loader name
    DamageAssetLoader,
    // extensions
    &["damage.ron"],
);

// Generated `DamageAsset` struct would look like this:
// pub struct DamageAsset {
//     damage: f32
// }

```


# Compatibility

| bevy   | bevy_asset_maker |
|--------|------------------|
| 0.12.1 | 0.1.*            |
