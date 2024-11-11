#![allow(clippy::needless_pass_by_value)]
use bevy::{
    asset::LoadState,
    pbr::wireframe::WireframePlugin,
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_collider_gen::{
    rapier2d::{
        multi_convex_polyline_collider_translated, single_convex_polyline_collider_translated,
        single_heightfield_collider_translated,
    },
    Edges,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapePlugin},
    shapes,
};
use bevy_rapier2d::prelude::*;
use indoc::indoc;
use std::collections::HashMap;

/// Colliders (or, with no png path specified, Car + Boulder + Terrain)
/// Illustrating how to use PNG files w transparency to generate colliders (and geometry)
/// for 2d sprites.
///
/// Controls
/// ← ↑ ↓ → (pan camera)
/// w (zoom in)
/// d (zoom out)

/// Custom PNG: `bevy_rapier2d` `convex_polyline` collider
/// from png path specified as cli argument
fn custom_png_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let sprite_handle = game_assets.image_handles.get("custom_png");
    if sprite_handle.is_none() {
        return;
    }
    let sprite_image = image_assets.get(sprite_handle.unwrap()).unwrap();

    let colliders = multi_convex_polyline_collider_translated(sprite_image);
    for collider in colliders {
        commands.spawn((
            collider.unwrap(),
            RigidBody::Fixed,
            SpriteBundle {
                texture: sprite_handle.unwrap().clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ));
    }

    //
    // An approach to generating convex decomposition colliders for your sprites with this crate
    //

    // let edge_coordinate_groups = multi_image_edge_translated(sprite_image);
    // for coords in edge_coordinate_groups {
    //     let indices: Vec<[u32; 2]> = (0..coords.len()).map(|i| [i as u32, i as u32]).collect();
    //     let collider = Collider::convex_decomposition(&coords, &indices);
    //     commands.spawn((
    //         collider,
    //         RigidBody::Fixed,
    //         SpriteBundle {
    //             texture: sprite_handle.unwrap().clone(),
    //             ..default()
    //         },
    //     ));
    // }
}

/// for the movement system
#[derive(Component, Resource)]
pub struct Car {
    pub initial_xyz: Vec3,
}

/// Car: `bevy_rapier2d` `convex_polyline` collider
/// from assets/sprite/car.png
fn car_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let initial_xyz = Vec3::new(-200.0, 2.0, 0.0);
    let sprite_handle = game_assets.image_handles.get("car_handle");
    if sprite_handle.is_none() {
        return;
    }
    let sprite_image = image_assets.get(sprite_handle.unwrap()).unwrap();
    let collider = single_convex_polyline_collider_translated(sprite_image).unwrap();
    commands.spawn((
        collider,
        RigidBody::Dynamic,
        SpriteBundle {
            texture: sprite_handle.unwrap().clone(),
            transform: Transform::from_xyz(initial_xyz.x, initial_xyz.y, initial_xyz.z),
            ..default()
        },
        Car { initial_xyz },
    ));
}

/// Terrain: `bevy_rapier2d` heightfield collider
/// from assets/sprite/terrain.png
fn terrain_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let sprite_handle = game_assets.image_handles.get("terrain_handle");
    if sprite_handle.is_none() {
        return;
    }
    let sprite_image = image_assets.get(sprite_handle.unwrap()).unwrap();
    let collider = single_heightfield_collider_translated(sprite_image);
    commands.spawn((
        collider,
        RigidBody::Fixed,
        SpriteBundle {
            texture: sprite_handle.unwrap().clone(),
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
    let sprite_handle = game_assets.image_handles.get("boulders");
    if sprite_handle.is_none() {
        return;
    }
    let sprite_image = image_assets.get(sprite_handle.unwrap()).unwrap();

    let edges = Edges::from(sprite_image);
    let coord_group = edges.multi_image_edge_translated();
    let colliders = multi_convex_polyline_collider_translated(sprite_image);

    for (coords, collider) in coord_group.iter().zip(colliders.into_iter()) {
        let shape = shapes::Polygon {
            points: coords.clone(),
            closed: true,
        };
        let geometry = GeometryBuilder::build_as(&shape);
        let fill = Fill::color(Srgba::hex("#545454").unwrap());
        let transform = Transform::from_xyz(0., 40., 0.);

        commands.spawn((
            geometry,
            fill,
            transform,
            collider.unwrap(),
            RigidBody::Dynamic,
        ));
    }
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

#[derive(Component, Resource, Default)]
pub struct GameAsset {
    pub font_handle: Handle<Font>,
    pub image_handles: HashMap<String, Handle<Image>>,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "colliders".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: ".".to_string(),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .insert_resource(GameAsset::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(ShapePlugin)
        .add_plugins(WireframePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(RapierDebugRenderPlugin {
            style: DebugRenderStyle {
                collider_fixed_color: [360., 100., 100., 1.],
                collider_dynamic_color: [360., 100., 100., 1.],
                ..default()
            },
            ..default()
        })
        .add_systems(OnEnter(AppState::Loading), load_assets)
        .add_systems(
            OnExit(AppState::Loading),
            (
                camera_spawn,
                custom_png_spawn,
                car_spawn,
                terrain_spawn,
                boulders_spawn,
                controls_text_spawn,
            ),
        )
        .add_systems(
            Update,
            (
                check_assets.run_if(in_state(AppState::Loading)),
                camera_movement.run_if(in_state(AppState::Running)),
                car_movement.run_if(in_state(AppState::Running)),
            ),
        )
        .run();
}

pub fn check_assets(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAsset>,
    mut state: ResMut<NextState<AppState>>,
) {
    for h in game_assets.image_handles.values() {
        if Some(LoadState::Loaded) != asset_server.get_load_state(h) {
            return;
        }
    }

    if Some(LoadState::Loaded)
        != asset_server.get_load_state(&game_assets.font_handle.clone().untyped())
    {
        return;
    }

    state.set(AppState::Running);
}

pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_movement(
    mut query: Query<(&Camera, &mut OrthographicProjection, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (_, mut projection, mut transform) in &mut query {
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x += 10.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x -= 10.0;
        }

        if keys.pressed(KeyCode::ArrowUp) {
            transform.translation.y -= 10.0;
        }

        if keys.pressed(KeyCode::ArrowDown) {
            transform.translation.y += 10.0;
        }

        if keys.pressed(KeyCode::KeyW) {
            projection.scale -= 0.01;
        }

        if keys.pressed(KeyCode::KeyS) {
            projection.scale += 0.01;
        }
    }
}

pub fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAsset>) {
    let custom_png_path = std::env::args().nth(1);
    game_assets.font_handle = asset_server.load("assets/font/NotoSansMono-Bold.ttf");

    if let Some(png_path) = custom_png_path {
        info!("Loading {}", png_path);
        game_assets.image_handles =
            HashMap::from([("custom_png".into(), asset_server.load(&png_path))]);
        return;
    }

    game_assets.image_handles = HashMap::from([
        (
            "car_handle".into(),
            asset_server.load("assets/sprite/car.png"),
        ),
        (
            "terrain_handle".into(),
            asset_server.load("assets/sprite/terrain.png"),
        ),
        (
            "boulders".into(),
            asset_server.load("assets/sprite/boulders.png"),
        ),
    ]);
}

pub fn controls_text_spawn(mut commands: Commands, game_assets: Res<GameAsset>) {
    let mut tips_text: String = indoc! {"
        controls
        --------------------
        ← ↑ ↓ → (pan camera)
        w (zoom in)
        s (zoom out)
    "}
    .into();

    if game_assets.image_handles.contains_key("car_handle") {
        let car_controls: String = indoc! {"
            a d (move car)
            1 (reset car transform to initial)
        "}
        .into();

        tips_text.push_str(&car_controls);
    }

    let node_bundle = NodeBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Px(10.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            left: Val::Px(80.0),
            bottom: Val::Px(600.0),
            ..default()
        },
        ..Default::default()
    };
    let text_bundle = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: tips_text.to_string(),
                style: TextStyle {
                    font: game_assets.font_handle.clone(),
                    font_size: 20.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            }],
            justify: JustifyText::Left,
            ..default()
        },
        ..Default::default()
    };

    commands.spawn(node_bundle).with_children(|parent| {
        parent.spawn(text_bundle);
    });
}

pub fn car_movement(mut query: Query<(&Car, &mut Transform)>, keys: Res<ButtonInput<KeyCode>>) {
    for (car, mut transform) in &mut query {
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += 5.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= 5.0;
        }

        if keys.pressed(KeyCode::Digit1) {
            *transform =
                Transform::from_xyz(car.initial_xyz.x, car.initial_xyz.y, car.initial_xyz.z);
        }
    }
}
