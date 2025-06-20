#![allow(clippy::needless_pass_by_value)]
use avian2d::prelude::*;
use bevy::{asset::LoadState, color::palettes::css, prelude::*};
use bevy_collider_gen::{
    plugin::{DynamicCollider, DynamicColliderPlugin},
    prelude::*,
};
use bevy_prototype_lyon::{prelude::*, shapes};
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
// d (zoom out)

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
    let colliders = AbstractCollidersBuilder::try_from(sprite_image)
        .unwrap()
        .convex_polyline()
        .multiple()
        .into_iter()
        .filter_map(AbstractCollider::to_avian);

    commands.spawn(Sprite {
        image: sprite_handle.clone(),
        ..default()
    });

    for collider in colliders {
        commands.spawn((collider, RigidBody::Static));
    }
}

#[derive(Component)]
#[require(RigidBody, Transform = INITIAL_POSITION)]
pub struct Car;

/// Car: `convex_polyline` collider
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
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

/// Terrain: heightfield collider
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
        RigidBody::Static,
        Sprite {
            image: sprite_handle.clone(),
            ..default()
        },
        DebugRender::default().with_collider_color(css::VIOLET.into()),
    ));
}

#[derive(Component)]
pub struct Atlas;

fn atlas_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(sprite_handle) = game_assets.image_handles.get("atlas") else {
        return;
    };
    let layout = atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        2,
        2,
        None,
        None,
    ));

    commands.spawn((
        DynamicCollider {
            collider_type: ColliderType::ConvexPolyline,
            ..default()
        },
        RigidBody::Static,
        Sprite {
            image: sprite_handle.clone(),
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Atlas,
    ));
}
pub fn atlas_control(mut query: Query<&mut Sprite, With<Atlas>>, keys: Res<ButtonInput<KeyCode>>) {
    for mut sprite in &mut query {
        for key in keys.get_just_pressed() {
            match key {
                KeyCode::Digit2 => {
                    if let Some(atlas) = sprite.texture_atlas.as_mut() {
                        if atlas.index < 3 {
                            atlas.index += 1;
                        }
                    }
                }
                KeyCode::Digit3 => {
                    if let Some(atlas) = sprite.texture_atlas.as_mut() {
                        atlas.index = atlas.index.saturating_sub(1);
                    }
                }
                _ => {}
            }
        }
    }
}

/// Boulder: using groups of edge coordinates to create geometry to color fill
/// multiple `convex_polyline` colliders
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

    for (polygon, collider) in polygons.zip(builder.multiple().into_iter()) {
        let points = collider.points().unwrap().clone();
        let pos = polygon.first().unwrap().as_vec2()
            - points.first().unwrap()
            - Vec2::new((sprite_image.width() / 2) as f32, -30.);
        let collider = collider.to_avian().unwrap();

        commands.spawn((
            collider,
            ShapeBuilder::with(&shapes::Polygon {
                points,
                closed: true,
            })
            .fill(css::GRAY)
            .stroke((css::BLACK, 1.))
            .build(),
            Transform::from_xyz(pos.x, pos.y, 0.),
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
        .add_plugins(DynamicColliderPlugin::<Collider>::new())
        .init_state::<AppState>()
        .insert_resource(GameAsset::default())
        .add_systems(Startup, load_assets)
        .add_systems(
            OnExit(AppState::Loading),
            (
                atlas_spawn,
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
                (atlas_control, camera_movement, car_movement).run_if(in_state(AppState::Running)),
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
        ("atlas", asset_server.load("assets/sprite/atlas.png")),
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
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextLayout {
                    justify: JustifyText::Left,
                    ..default()
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
                KeyCode::KeyS => {
                    if transform.scale.x < f32::MAX && transform.scale.y < f32::MAX {
                        transform.scale += 0.01;
                    }
                }
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
