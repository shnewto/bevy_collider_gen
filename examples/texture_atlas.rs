#![allow(clippy::needless_pass_by_value)]
//! Demonstrates regenerating a `DynamicCollider` when a `TextureAtlas` frame changes.
//!
//! Physics backend is selected by feature flag:
//! - default / `rapier2d`: bevy_rapier2d
//! - `avian2d`: avian2d (wins if both features are enabled)
//!
//! Controls: ← → (previous / next atlas frame)

#[cfg(not(any(feature = "avian2d", feature = "rapier2d")))]
compile_error!("Enable the `avian2d` or `rapier2d` feature to run this example");

#[cfg(feature = "avian2d")]
use avian2d::prelude::*;
use bevy::{asset::LoadState, prelude::*};
use bevy_collider_gen::plugin::{DynamicCollider, DynamicColliderPlugin};
use bevy_collider_gen::prelude::*;
#[cfg(all(feature = "rapier2d", not(feature = "avian2d")))]
use bevy_rapier2d::prelude::*;
use indoc::indoc;

#[derive(Component)]
struct AtlasSprite;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    Loading,
    Running,
}

#[derive(Resource, Default)]
struct GameAsset {
    font_handle: Handle<Font>,
    atlas_handle: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "texture atlas colliders".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: ".".to_string(),
                ..default()
            }),
    );

    #[cfg(feature = "avian2d")]
    {
        app.add_plugins((
            PhysicsPlugins::default(),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
            DynamicColliderPlugin::<Collider>::new(),
        ));
    }

    #[cfg(all(feature = "rapier2d", not(feature = "avian2d")))]
    {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            #[cfg(debug_assertions)]
            RapierDebugRenderPlugin {
                style: DebugRenderStyle {
                    collider_fixed_color: [360., 100., 100., 1.],
                    ..default()
                },
                ..default()
            },
            DynamicColliderPlugin::<Collider>::new(),
        ));
    }

    app.init_state::<AppState>()
        .insert_resource(GameAsset::default())
        .add_systems(Startup, load_assets)
        .add_systems(
            OnExit(AppState::Loading),
            (atlas_spawn, camera_spawn, controls_text_spawn),
        )
        .add_systems(
            Update,
            (
                check_assets.run_if(in_state(AppState::Loading)),
                atlas_control.run_if(in_state(AppState::Running)),
            ),
        )
        .run();
}

fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAsset>) {
    game_assets.font_handle = asset_server.load("assets/font/NotoSansMono-Bold.ttf");
    game_assets.atlas_handle = asset_server.load("assets/sprite/atlas.png");
}

fn check_assets(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAsset>,
    mut state: ResMut<NextState<AppState>>,
) {
    let atlas_loaded = asset_server
        .get_load_state(&game_assets.atlas_handle)
        .is_some_and(|state| matches!(state, LoadState::Loaded));
    let font_loaded = asset_server
        .get_load_state(&game_assets.font_handle)
        .is_some_and(|state| matches!(state, LoadState::Loaded));
    if atlas_loaded && font_loaded {
        state.set(AppState::Running);
    }
}

fn atlas_spawn(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        2,
        2,
        None,
        None,
    ));
    let sprite = Sprite {
        image: game_assets.atlas_handle.clone(),
        texture_atlas: Some(TextureAtlas { layout, index: 0 }),
        custom_size: Some(Vec2::splat(128.)),
        ..default()
    };
    let collider = DynamicCollider {
        collider_type: ColliderType::ConvexPolyline,
        ..default()
    };

    #[cfg(feature = "avian2d")]
    {
        #[cfg(debug_assertions)]
        use bevy::color::palettes::css;

        commands.spawn((
            AtlasSprite,
            collider,
            RigidBody::Static,
            sprite,
            #[cfg(debug_assertions)]
            DebugRender::default().with_collider_color(css::VIOLET.into()),
        ));
    }

    #[cfg(all(feature = "rapier2d", not(feature = "avian2d")))]
    commands.spawn((AtlasSprite, collider, RigidBody::Fixed, sprite));
}

fn atlas_control(
    mut query: Query<&mut Sprite, With<AtlasSprite>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut sprite in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        for key in keys.get_just_pressed() {
            match key {
                KeyCode::ArrowRight => {
                    if atlas.index < 3 {
                        atlas.index += 1;
                    }
                }
                KeyCode::ArrowLeft => {
                    atlas.index = atlas.index.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn controls_text_spawn(mut commands: Commands, game_assets: Res<GameAsset>) {
    #[cfg(feature = "avian2d")]
    let backend = "avian2d";
    #[cfg(all(feature = "rapier2d", not(feature = "avian2d")))]
    let backend = "rapier2d";

    let tips_text = format!(
        indoc! {"
            texture atlas collider ({backend})
            --------------------
            ← → (cycle atlas frame)
        "},
        backend = backend
    );

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(24.),
            top: Val::Px(24.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text(tips_text),
                TextFont {
                    font: game_assets.font_handle.clone(),
                    font_size: 20.,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
