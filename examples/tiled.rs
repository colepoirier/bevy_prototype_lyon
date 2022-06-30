use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(despawn_system)
        .run();
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct LyonShape;

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(0);
    let second_pass_layer = RenderLayers::layer(1);

    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::from_translation(Vec3::new(512.0, 0.0, 0.0)),
        ))
        .insert(LyonShape)
        .insert(first_pass_layer);

    commands
        .spawn_bundle(Camera2dBundle {
            camera_2d: Camera2d::default(),
            camera: Camera {
                // render before the "main pass" camera
                priority: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(512.0, 0.0, 15.0)),
            ..default()
        })
        .insert(first_pass_layer);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(size.width as f32, size.height as f32)),
                ..default()
            },
            texture: image_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(second_pass_layer);

    // The main pass camera.
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)),
            ..default()
        })
        .insert(second_pass_layer);
}

fn despawn_system(mut commands: Commands, q: Query<Entity, With<LyonShape>>) {
    for e in q.iter() {
        // commands.entity(e).despawn();
    }
}
