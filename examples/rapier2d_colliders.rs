#![allow(clippy::needless_pass_by_value)]
use bevy::{asset::LoadState, color::palettes::css, prelude::*};
#[cfg(feature = "preprocess")]
use bevy_collider_gen::preprocess::{
    combo::ImageWithCollider, LoadedCollider, PreloadColliderPlugin,
};
use bevy_collider_gen::{
    plugin::{DynamicCollider, DynamicColliderPlugin},
    prelude::*,
};
use bevy_prototype_lyon::{prelude::*, shapes};
use bevy_rapier2d::prelude::*;
use edges::EdgesIter;
use indoc::indoc;
use std::collections::HashMap;

// Colliders: Car + Boulder + Terrain
// Illustrating how to use PNG files with transparency to generate colliders (and geometry)
// for 2d sprites.
//
// Controls
// ← ↑ ↓ → (pan camera)
// w (zoom in)
// s (zoom out)
// a d (move car)
// 1 (reset car)

/// Custom PNG: `convex_polyline` colliders
/// from png path specified as cli argument
fn custom_png_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let Some(sprite_handle) = game_assets.image_handles.get("custom_png") else {
        return;
    };
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    let colliders = AbstractCollidersBuilder::try_from(sprite_image)
        .unwrap()
        .convex_polyline()
        .multiple()
        .into_iter()
        .filter_map(AbstractCollider::to_rapier);

    commands.spawn(Sprite {
        image: sprite_handle.clone(),
        ..default()
    });
    for collider in colliders {
        commands.spawn((collider, RigidBody::Fixed));
    }
}

/// for the movement system
#[derive(Component)]
#[require(Velocity, RigidBody, Transform = INITIAL_POSITION)]
pub struct Car;

/// Car: `bevy_rapier2d` `convex_polyline` collider
/// from assets/sprite/car.png
fn car_spawn(mut commands: Commands, game_assets: Res<GameAsset>) {
    let Some(sprite_handle) = game_assets.image_handles.get("car") else {
        return;
    };

    commands.spawn((
        Car,
        DynamicCollider {
            collider_type: ColliderType::ConvexPolyline,
            ..default()
        },
        Sprite {
            image: sprite_handle.clone(),
            ..default()
        },
    ));
}

/// Terrain: `bevy_rapier2d` heightfield collider
/// from assets/sprite/terrain.png
fn terrain_spawn(mut commands: Commands, game_assets: Res<GameAsset>) {
    let Some(sprite_handle) = game_assets.image_handles.get("terrain") else {
        return;
    };

    commands.spawn((
        DynamicCollider {
            collider_type: ColliderType::Heightfield,
            ..default()
        },
        RigidBody::Fixed,
        Sprite {
            image: sprite_handle.clone(),
            ..default()
        },
    ));
}

/// Boulder: using groups of edge coordinates to create geometry to color fill
/// multiple `bevy_rapier2d` `convex_polyline` colliders
/// from assets/sprite/boulders.png
fn boulders_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let Some(sprite_handle) = game_assets.image_handles.get("boulders") else {
        return;
    };
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    let builder = AbstractCollidersBuilder::try_from(sprite_image)
        .unwrap()
        .absolute()
        .convex_polyline();
    let polygons = EdgesIter::new(builder.image());

    #[allow(unused_variables)]
    for (index, (polygon, collider)) in polygons.zip(builder.multiple()).enumerate() {
        let points = collider.points().unwrap().clone();
        let pos = polygon.first().unwrap().as_vec2()
            - points.first().unwrap()
            - Vec2::new(
                (sprite_image.width() / 2) as f32,
                (sprite_image.height() / 2) as f32,
            );
        #[cfg(not(feature = "preprocess"))]
        let collider = collider.to_rapier().unwrap();

        commands.spawn((
            #[cfg(not(feature = "preprocess"))]
            collider,
            #[cfg(feature = "preprocess")]
            DynamicCollider {
                collider_type: ColliderType::ConvexPolyline,
                image: Some(sprite_handle.clone()),
                anchor: bevy::sprite::Anchor(pos),
                multiple_index: Some(index),
                ..default()
            },
            ShapeBuilder::with(&shapes::Polygon {
                points,
                closed: true,
            })
            .fill(css::GRAY)
            .stroke((css::BLACK, 1.))
            .build(),
            Transform::from_xyz(pos.x, pos.y + 60., 0.),
            RigidBody::Dynamic,
        ));
    }
}

#[cfg(feature = "preprocess")]
/// Collects labeled handles to the loaded images.
fn load_images(image_query: Query<(&Name, &LoadedCollider)>, mut game_assets: ResMut<GameAsset>) {
    image_query
        .iter()
        .for_each(|(name, LoadedCollider(image_handle, _))| {
            // Map the image paths to simpler names (optional)
            let name = match name.as_str() {
                "sprite_with_meta/car.png" => "car",
                "sprite_with_meta/terrain.png" => "terrain",
                "sprite_with_meta/boulders.png" => "boulders",
                "sprite_with_meta/atlas.png" => "atlas",
                other => other,
            }
            .to_string();

            game_assets.image_handles.insert(name, image_handle.clone());
        });
}

///
/// After this, things don't differ in a way related to this crate, it's just some of my
/// personal boilerplate
///
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Running,
}

#[derive(Resource, Default)]
pub struct GameAsset {
    pub font_handle: Handle<Font>,
    pub image_handles: HashMap<String, Handle<Image>>,
    #[cfg(feature = "preprocess")]
    /// Contains [`Handle`]s to the processed collider assets.
    ///
    /// These are needed to load the processed colliders,
    /// but can be removed once the [`LoadedCollider`] [`Component`]s are spawned.
    pub collider_handles: HashMap<String, Handle<ImageWithCollider>>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "colliders".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    #[cfg(feature = "preprocess")]
                    mode: AssetMode::Processed,
                    file_path: "./assets".to_string(),
                    ..default()
                }),
            ShapePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            #[cfg(debug_assertions)]
            RapierDebugRenderPlugin {
                style: DebugRenderStyle {
                    collider_fixed_color: [360., 100., 100., 1.],
                    collider_dynamic_color: [360., 100., 100., 1.],
                    ..default()
                },
                ..default()
            },
        ))
        .add_plugins((
            DynamicColliderPlugin::<Collider>::new(),
            #[cfg(feature = "preprocess")]
            // If you don't want to use .meta files, remove `.without_default_asset_processor()`
            PreloadColliderPlugin::default().without_default_asset_processor(),
        ))
        .init_state::<AppState>()
        .insert_resource(GameAsset::default())
        .add_systems(Startup, load_assets)
        .add_systems(
            OnExit(AppState::Loading),
            (
                #[cfg(not(feature = "preprocess"))]
                (car_spawn, terrain_spawn, boulders_spawn, custom_png_spawn),
                #[cfg(feature = "preprocess")]
                (car_spawn, terrain_spawn, boulders_spawn, custom_png_spawn).after(load_images),
                camera_spawn,
                #[cfg(feature = "preprocess")]
                load_images,
                controls_text_spawn,
            ),
        )
        .add_systems(
            Update,
            (
                check_assets.run_if(in_state(AppState::Loading)),
                (camera_movement, car_movement).run_if(in_state(AppState::Running)),
            ),
        )
        .add_systems(Last, check_assets.run_if(in_state(AppState::Loading)))
        .run();
}

pub fn check_assets(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAsset>,
    mut state: ResMut<NextState<AppState>>,
    mut run_next: Local<bool>,
) {
    #[cfg(feature = "preprocess")]
    {
        // Check if all colliders are loaded (which also loads the images as assets)
        let colliders_loaded = game_assets.collider_handles.values().all(|handle| {
            asset_server.is_loaded_with_dependencies(handle)
                || if let Some(load_state) = asset_server.get_load_state(handle) {
                    matches!(load_state, LoadState::Loaded)
                } else {
                    true
                }
        });
        if !colliders_loaded {
            return;
        }
    }

    // Once assets are loaded with dependencies, wait until the next frame so they populate the Query
    if !*run_next {
        *run_next = true;
        return;
    }

    // Check if all images are loaded (will be initially empty if pre-processed)
    let all_images_loaded = game_assets.image_handles.values().all(|handle| {
        asset_server.is_loaded_with_dependencies(handle)
            || if let Some(load_state) = asset_server.get_load_state(handle) {
                matches!(load_state, LoadState::Loaded)
            } else {
                true
            }
    });

    let font_load_state = asset_server.get_load_state(&game_assets.font_handle.clone());
    if all_images_loaded && font_load_state.is_some_and(|state| matches!(state, LoadState::Loaded))
    {
        state.set(AppState::Running);
    }
}

pub fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAsset>) {
    game_assets.font_handle = asset_server.load("font/NotoSansMono-Bold.ttf");
    #[cfg(not(feature = "preprocess"))]
    {
        game_assets.image_handles = HashMap::from([
            ("car".into(), asset_server.load("sprite/car.png")),
            ("terrain".into(), asset_server.load("sprite/terrain.png")),
            ("boulders".into(), asset_server.load("sprite/boulders.png")),
            ("atlas".into(), asset_server.load("sprite/atlas.png")),
        ]);
    }
    #[cfg(feature = "preprocess")]
    {
        game_assets.collider_handles = HashMap::from([
            ("car".into(), asset_server.load("sprite_with_meta/car.png")),
            (
                "terrain".into(),
                asset_server.load("sprite_with_meta/terrain.png"),
            ),
            (
                "boulders".into(),
                asset_server.load("sprite_with_meta/boulders.png"),
            ), // TODO: Multi-collider support
            (
                "atlas".into(),
                asset_server.load("sprite_with_meta/atlas.png"),
            ),
        ]);
    }
    if let Some(png_path) = std::env::args().nth(1) {
        info!("Loading {}", png_path);
        game_assets
            .image_handles
            .insert("custom_png".into(), asset_server.load(&png_path));
    }
}

pub fn controls_text_spawn(mut commands: Commands, game_assets: Res<GameAsset>) {
    let tips_text = indoc! {"
        controls
        --------------------
        ← ↑ ↓ → (pan camera)
        w (zoom in)
        s (zoom out)
        a d (move car)
        1 (reset car transform to initial)
    "};

    commands
        .spawn(Node {
            width: Val::Px(100.),
            height: Val::Px(10.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            left: Val::Px(80.),
            bottom: Val::Px(600.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text(tips_text.to_string()),
                TextFont {
                    font: game_assets.font_handle.clone().into(),
                    font_size: FontSize::Px(20.),
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextLayout {
                    justify: Justify::Left,
                    ..Default::default()
                },
            ));
        });
}

pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut query {
        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowUp => transform.translation.y += 10.,
                KeyCode::ArrowDown => transform.translation.y -= 10.,
                KeyCode::ArrowLeft => transform.translation.x -= 10.,
                KeyCode::ArrowRight => transform.translation.x += 10.,
                KeyCode::KeyW => {
                    if transform.scale.x > 0.01 && transform.scale.y > 0.01 {
                        transform.scale -= 0.01;
                    }
                }
                KeyCode::KeyS if transform.scale.x < f32::MAX && transform.scale.y < f32::MAX => {
                    transform.scale += 0.01;
                }
                _ => {}
            }
        }
    }
}

pub fn car_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Car>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut velocity) in &mut query {
        let linear_velocity = &mut velocity.linear;
        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyA => linear_velocity.x -= 30.,
                KeyCode::KeyD => linear_velocity.x += 30.,
                KeyCode::Digit1 => {
                    *linear_velocity = Vec2::default();
                    *transform = INITIAL_POSITION;
                }
                _ => {}
            }
        }
    }
}
const INITIAL_POSITION: Transform = Transform::from_xyz(-200., 2., 0.);
