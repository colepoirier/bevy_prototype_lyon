# Bevy + Lyon = ❤

[![Crates.io](https://img.shields.io/crates/v/bevy_prototype_lyon)](https://crates.io/crates/bevy_prototype_lyon)
[![Crates.io](https://img.shields.io/crates/l/bevy_prototype_lyon)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/bevy_prototype_lyon)](https://crates.io/crates/bevy_prototype_lyon)
[![GitHub Repo stars](https://img.shields.io/github/stars/Nilirad/bevy_prototype_lyon)](https://github.com/Nilirad/bevy_prototype_lyon)
[![CI](https://github.com/Nilirad/bevy_prototype_lyon/actions/workflows/ci.yml/badge.svg)](https://github.com/Nilirad/bevy_prototype_lyon/actions/workflows/ci.yml)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

`bevy_prototype_lyon` enables [**Bevy**](https://bevyengine.org) users to draw 2D shapes and paths, like triangles, circles, rectangles, lines, arcs and beziers.

[**Try out the live demo!**](https://nilirad.github.io/bevy_prototype_lyon_showcase/)
![Regular polygon demo](docs/polygon_demo.webp)

## How does it work?

Currently Bevy does not support drawing custom shapes in an easy way. This crate uses a variation of Bevy's `SpriteBundle` with custom meshes to draw shapes. The [**lyon**](https://docs.rs/lyon_tessellation) crate is used to generate those custom mesh.

## Changelog

### 0.6.0

- Support for Bevy 0.8

### 0.5.0
- Support for Bevy 0.7

### 0.4.0
- Support for Bevy 0.6
- Shape properties can be dynamically changed

### 0.3.1
- Restored support for bevy_webgl2 (lost on v0.3.0).

### 0.3.0
- Support for Bevy 0.5
- Shapes with outline

### 0.2.0
- Complete API reworking
- Regular polygon support
- Extensible shape system through `Geometry` trait

### 0.1.5
- updated dependency to `lyon_tessellation v0.17`
- with `lyon_tessellation v0.17`, unfortunately rectangles with rounded borders are no longer supported.
- `Quad`, `Triangle` and `Polyline` have been substituted by a general-purpose `Polygon` shape.

## Usage

Add the following line in your `cargo.toml` manifest file, under the `[dependencies]` section:

```TOML
bevy_prototype_lyon = "0.6.0"
```

Then, you can start by drawing simple shapes:

```rust
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform::default(),
    ));
}
```

Don't forget to check out the [examples](examples/) to learn more!

## Bevy versions supported

I strive to support the latest version of Bevy. Support for a version of Bevy is dropped as soon as a new one is released.

The following table shows the latest version of `bevy_prototype_lyon` that supports a certain version of Bevy.

|bevy|bevy_prototype_lyon|license|
|---|---|---|
|0.8|0.6|MIT/Apache 2.0|
|0.7|0.5|MIT/Apache 2.0|
|0.6|0.4|MIT/Apache 2.0|
|0.5|0.3|MIT|
|0.4|0.2|MIT|

***

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
