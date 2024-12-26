#![allow(clippy::needless_pass_by_value)]
use avian2d::prelude::*;
use bevy::{asset::LoadState, color::palettes::css, prelude::*};
use bevy_collider_gen::{
    avian2d::{generate_collider, generate_colliders},
    edges::{anchor::Anchor, Edges},
    ColliderType,
};
use bevy_prototype_lyon::{prelude::*, shapes};
use indoc::indoc;
use std::collections::HashMap;

/// Colliders: Car + Boulder + Terrain
/// Illustrating how to use PNG files with transparency to generate colliders (and geometry)
/// for 2d sprites.
///
/// Controls
/// ← ↑ ↓ → (pan camera)
/// w (zoom in)
/// d (zoom out)

/// Custom PNG: `convex_polyline` collider
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
    let colliders = generate_colliders(
        sprite_image,
        ColliderType::ConvexPolyline,
        Anchor::Center(sprite_image.height(), sprite_image.width()),
    );

    for collider in colliders {
        commands.spawn((
            collider,
            RigidBody::Static,
            Sprite {
                image: sprite_handle.clone(),
                ..default()
            },
        ));
    }
}

#[derive(Component)]
#[require(RigidBody, Transform(|| INITIAL_POSITION))]
pub struct Car;

/// Car: `convex_polyline` collider
/// from assets/sprite/car.png
fn car_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let Some(sprite_handle) = game_assets.image_handles.get("car") else {
        return;
    };
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    let collider = generate_collider(
        sprite_image,
        ColliderType::ConvexPolyline,
        Anchor::Center(sprite_image.height(), sprite_image.width()),
    )
    .unwrap();

    commands.spawn((
        Car,
        collider,
        Sprite {
            image: sprite_handle.clone(),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

/// Terrain: heightfield collider
/// from assets/sprite/terrain.png
fn terrain_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
) {
    let Some(sprite_handle) = game_assets.image_handles.get("terrain") else {
        return;
    };
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    let collider = generate_collider(
        sprite_image,
        ColliderType::Heightfield,
        Anchor::VerticalCenter(sprite_image.height()),
    )
    .unwrap();

    commands.spawn((
        collider,
        RigidBody::Static,
        Sprite {
            image: sprite_handle.clone(),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

/// Boulder: using groups of edge coordinates to create geometry to color fill
/// multiple `convex_polyline` colliders
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

    let edges = Edges::try_from(sprite_image).unwrap();
    let polygons = edges.iter();
    let colliders = generate_colliders(
        sprite_image,
        ColliderType::ConvexPolyline,
        Anchor::AbsoluteCenter,
    );

    for (polygon, collider) in polygons.zip(colliders.into_iter()) {
        let mut pos = polygon.first().unwrap().as_vec2();
        let polygon = Anchor::AbsoluteCenter.translate(polygon);
        pos -= polygon.first().unwrap();
        let shape = shapes::Polygon {
            points: polygon,
            closed: true,
        };

        commands.spawn((
            collider,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_xyz(
                    pos.x - (sprite_image.width() / 2) as f32,
                    pos.y,
                    0.,
                ),
                ..default()
            },
            Fill::color(css::GRAY),
            Stroke::new(css::BLACK, 1.),
            RigidBody::Dynamic,
            DebugRender::default().with_collider_color(css::VIOLET.into()),
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

#[derive(Resource, Default)]
pub struct GameAsset {
    pub font_handle: Handle<Font>,
    pub image_handles: HashMap<&'static str, Handle<Image>>,
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
                    file_path: ".".to_string(),
                    ..default()
                }),
            ShapePlugin,
            PhysicsPlugins::default(),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
        ))
        .init_state::<AppState>()
        .insert_resource(GameAsset::default())
        .add_systems(Startup, load_assets)
        .add_systems(
            OnExit(AppState::Loading),
            (
                camera_spawn,
                car_spawn,
                terrain_spawn,
                boulders_spawn,
                custom_png_spawn,
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
    let all_images_loaded = game_assets.image_handles.values().all(|handle| {
        asset_server
            .get_load_state(handle)
            .is_some_and(|state| matches!(state, LoadState::Loaded))
    });
    let font_load_state = asset_server.get_load_state(&game_assets.font_handle.clone());
    if all_images_loaded && font_load_state.is_some_and(|state| matches!(state, LoadState::Loaded))
    {
        state.set(AppState::Running);
    }
}

pub fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAsset>) {
    game_assets.font_handle = asset_server.load("assets/font/NotoSansMono-Bold.ttf");
    game_assets.image_handles = HashMap::from([
        ("car", asset_server.load("assets/sprite/car.png")),
        ("terrain", asset_server.load("assets/sprite/terrain.png")),
        ("boulders", asset_server.load("assets/sprite/boulders.png")),
    ]);
    if let Some(png_path) = std::env::args().nth(1) {
        info!("Loading {}", png_path);
        game_assets
            .image_handles
            .insert("custom_png", asset_server.load(&png_path));
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
                    font: game_assets.font_handle.clone(),
                    font_size: 20.,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextLayout {
                    justify: JustifyText::Left,
                    ..Default::default()
                },
            ));
        });
}

pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_movement(
    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut projection, mut transform) in &mut query {
        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowUp => transform.translation.y += 10.,
                KeyCode::ArrowDown => transform.translation.y -= 10.,
                KeyCode::ArrowLeft => transform.translation.x -= 10.,
                KeyCode::ArrowRight => transform.translation.x += 10.,
                KeyCode::KeyW => projection.scale -= 0.01,
                KeyCode::KeyS => projection.scale += 0.01,
                _ => {}
            }
        }
    }
}

pub fn car_movement(
    mut query: Query<(&mut Transform, &mut LinearVelocity), With<Car>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut linear_velocity) in &mut query {
        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyA => linear_velocity.x -= 30.,
                KeyCode::KeyD => linear_velocity.x += 30.,
                KeyCode::Digit1 => {
                    *linear_velocity = LinearVelocity::ZERO;
                    *transform = INITIAL_POSITION;
                }
                _ => {}
            }
        }
    }
}
const INITIAL_POSITION: Transform = Transform::from_xyz(-200., 2., 0.);
