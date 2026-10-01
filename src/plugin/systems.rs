#[cfg(feature = "preprocess")]
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use edges::BinaryImage;

use super::{utils::process_image, DynamicCollider};
use crate::prelude::{AbstractCollider, AbstractCollidersBuilder};
#[cfg(feature = "preprocess")]
use crate::preprocess::{
    combo::{CachedCollider, ColliderAtlas},
    LoadedCollider,
};

type Filter<TargetCollider> = Or<(
    Without<TargetCollider>,
    Added<Sprite>,
    Changed<Sprite>,
    Changed<DynamicCollider>,
)>;

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
/// Updates the physics colliders using change detection.
pub fn update_colliders<TargetCollider>(
    mut commands: Commands,
    query: Query<(Entity, &DynamicCollider, Option<&Sprite>), Filter<TargetCollider>>,
    images: Res<Assets<Image>>,
    #[cfg(feature = "preprocess")] mut saved_query: Query<&LoadedCollider>,
    #[cfg(feature = "preprocess")] mut cached_colliders: ResMut<Assets<CachedCollider>>,
    changed_images: Query<&Sprite, AssetChanged<Sprite>>,
    changed_atlases: Query<&Sprite, Changed<Sprite>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    #[cfg(feature = "preprocess")] mut changes: Local<HashMap<Entity, Sprite>>,
) where
    AbstractCollider: Into<Option<TargetCollider>>,
    TargetCollider: Component,
{
    for (entity, dynamic_collider, sprite) in query.iter() {
        let (handle, atlas, size, rect) = dynamic_collider.merge_with_sprite(sprite);
        let handle = handle.cloned();
        let atlas = atlas.cloned();
        let (flip_x, flip_y) = sprite
            .map(|sprite| (sprite.flip_x, sprite.flip_y))
            .unwrap_or_default();

        // Change detection for a [`Sprite`]'s source [`Image`] & [`TextureAtlas`] index
        let image_changed = changed_images.get(entity).is_ok();
        let atlas_changed = changed_atlases.get(entity).is_ok() || sprite.is_none();
        if !(image_changed || atlas_changed) {
            #[cfg(debug_assertions)]
            bevy::log::info!(
                "Ignoring entity {:?} for collider updates (no change detected).",
                entity
            );
            continue;
        }

        if let Some(handle) = handle {
            #[cfg(feature = "preprocess")]
            let cached_collider = find_collider(
                &handle,
                &saved_query,
                &cached_colliders,
                dynamic_collider,
                atlas.as_ref(),
            );

            // Use the saved collider as long as the underlying [`Image`] asset hasn't been modified
            #[cfg(feature = "preprocess")]
            let exists = changes.get(&entity).is_some();
            #[cfg(feature = "preprocess")]
            if !exists {
                if let Some(sprite) = sprite {
                    changes.insert(
                        entity,
                        Sprite {
                            image: sprite.image.clone(),
                            texture_atlas: sprite.texture_atlas.clone(),
                            ..default()
                        },
                    );
                }
            }

            #[cfg(feature = "preprocess")]
            let sprite_unchanged = changes.get(&entity).is_some_and(|old_sprite| {
                sprite.is_some_and(|new_sprite| {
                    old_sprite.flip_x == new_sprite.flip_x
                        && old_sprite.flip_y == new_sprite.flip_y
                        && old_sprite.image == new_sprite.image
                        && old_sprite.custom_size == new_sprite.custom_size
                        && old_sprite.texture_atlas.as_ref().map(|atlas| &atlas.layout)
                            == new_sprite.texture_atlas.as_ref().map(|atlas| &atlas.layout)
                        && old_sprite.rect == new_sprite.rect
                })
            });

            #[cfg(feature = "preprocess")]
            let sprite_changed = if sprite_unchanged {
                false
            } else if let Some(sprite) = sprite {
                changes.insert(entity, sprite.clone());
                true
            } else {
                false
            };

            #[cfg(feature = "preprocess")]
            let spawned = if let Some(collider) = cached_collider.0 {
                if !image_changed && !sprite_changed {
                    let Ok(mut target) = commands.get_entity(entity) else {
                        continue;
                    };
                    target.insert(collider);
                    true
                } else {
                    false
                }
            } else {
                false
            };
            #[cfg(not(feature = "preprocess"))]
            let spawned = false;

            // Generate a new physics collider according to the type included in the [`DynamicCollider`]
            if !spawned {
                if let Some(image) = images.get(handle.id()) {
                    if let Ok(binary_image) = BinaryImage::try_from(image) {
                        // Generate an atlas
                        if let Some(atlas) = &atlas {
                            if let Some(layout) = layouts.get(&atlas.layout) {
                                // if let Ok(image) = processed_image {
                                let mut colliders = vec![];
                                for urect in &layout.textures {
                                    // Get a cropped view into the original image and calculate the collider generation from it
                                    let sub_image = process_image(
                                        binary_image.clone(),
                                        Some(*urect),
                                        size,
                                        rect,
                                        flip_x,
                                        flip_y,
                                    );
                                    let collider = AbstractCollidersBuilder::new(sub_image)
                                        // TODO: Saved atlas collider type overwritten when Image changes
                                        .with_type(dynamic_collider.collider_type)
                                        .single();
                                    colliders.push(collider);
                                }
                                #[cfg(feature = "preprocess")]
                                // Update the [`CachedCollider`] so change detection uses the correct version
                                if !saved_query.iter_mut().any(
                                    |LoadedCollider(image_handle, collider_handle)| {
                                        if handle == *image_handle {
                                            if let Some(mut collider) =
                                                cached_colliders.get_mut(collider_handle)
                                            {
                                                *collider = CachedCollider::Atlas(ColliderAtlas(
                                                    colliders.clone(),
                                                    layout.clone(),
                                                ));
                                                true
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    },
                                ) {
                                    // If no [`LoadedCollider`] exists, create a new one
                                    commands.spawn(LoadedCollider(
                                        handle,
                                        cached_colliders.add(CachedCollider::Atlas(ColliderAtlas(
                                            colliders.clone(),
                                            layout.clone(),
                                        ))),
                                    ));
                                }

                                // Find the new collider at the current atlas position and insert it
                                if let Some(collider) = colliders
                                    .get(atlas.index)
                                    .cloned()
                                    .flatten()
                                    .and_then(Into::<Option<TargetCollider>>::into)
                                {
                                    let Ok(mut target) = commands.get_entity(entity) else {
                                        continue;
                                    };
                                    #[cfg(all(debug_assertions, feature = "preprocess"))]
                                    bevy::log::info!(
                                        "Generating new atlas collider for entity {:?}.",
                                        entity
                                    );
                                    target.insert(collider);
                                } else {
                                    error!(
                                            "Failed to generate collider from image for entity {:?}: atlas failed to yield a physics collider",
                                            entity
                                        );
                                }
                            } else {
                                error!(
                                    "Failed to generate collider from image for entity {:?}: no layout for atlas",
                                    entity
                                );
                            }
                        } else if let Some(index) = dynamic_collider.multiple_index {
                            // Generate multiple colliders
                            // TODO: Multiple-style colliders generate `n` collider sets instead of 1
                            let processed_image =
                                process_image(binary_image, None, size, rect, flip_x, flip_y);
                            let builder = AbstractCollidersBuilder::new(processed_image)
                                .with_type(dynamic_collider.collider_type)
                                .absolute();
                            let image_width = builder.image().width();
                            let image_height = builder.image().height();
                            let polygons = edges::EdgesIter::new(builder.image());
                            let colliders = polygons
                                .zip(builder.multiple())
                                .filter_map(|(polygon, collider)| {
                                    let points = collider.points()?.clone();
                                    let pos = polygon.first()?.as_vec2()
                                        - points.first()?
                                        - Vec2::new(
                                            (image_width / 2) as f32,
                                            (image_height / 2) as f32,
                                        );
                                    Some((collider, pos))
                                })
                                .collect::<Vec<(AbstractCollider, Vec2)>>();

                            #[cfg(feature = "preprocess")]
                            // Update the [`CachedCollider`] so change detection uses the correct version
                            if !saved_query.iter_mut().any(
                                |LoadedCollider(image_handle, collider_handle)| {
                                    if handle == *image_handle {
                                        if let Some(mut collider) =
                                            cached_colliders.get_mut(collider_handle)
                                        {
                                            *collider = CachedCollider::Multiple(colliders.clone());
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                },
                            ) {
                                // If no [`LoadedCollider`] exists, create a new one
                                commands.spawn(LoadedCollider(
                                    handle,
                                    cached_colliders
                                        .add(CachedCollider::Multiple(colliders.clone())),
                                ));
                            }

                            // Insert the new collider version for the current entity
                            if let Some(collider) = colliders.get(index) {
                                if let Some(target_collider) = collider.0.clone().into() {
                                    let Ok(mut target) = commands.get_entity(entity) else {
                                        continue;
                                    };
                                    #[cfg(all(debug_assertions, feature = "preprocess"))]
                                    bevy::log::info!(
                                        "Generating new multiple-style collider for entity {:?}.",
                                        entity
                                    );
                                    target.insert(target_collider);
                                } else {
                                    error!(
                                        "Failed to generate collider from image for entity {:?}: failed to convert to physics collider",
                                        entity
                                    );
                                }
                            } else {
                                error!(
                                    "Failed to generate collider from image for entity {:?}: requested multiple index out of bounds",
                                    entity
                                );
                            }
                        } else {
                            // Generate a single collider
                            let processed_image =
                                process_image(binary_image, None, size, rect, flip_x, flip_y);
                            if let Some(abstract_collider) =
                                AbstractCollidersBuilder::new(processed_image)
                                    .with_type(dynamic_collider.collider_type)
                                    .single()
                            {
                                #[cfg(feature = "preprocess")]
                                // Update the [`CachedCollider`] so change detection uses the correct version
                                if !saved_query.iter_mut().any(
                                    |LoadedCollider(image_handle, collider_handle)| {
                                        if handle == *image_handle {
                                            if let Some(mut collider) =
                                                cached_colliders.get_mut(collider_handle)
                                            {
                                                *collider = CachedCollider::Single(
                                                    abstract_collider.clone(),
                                                );
                                                true
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    },
                                ) {
                                    // If no [`LoadedCollider`] exists, create a new one
                                    commands.spawn(LoadedCollider(
                                        handle,
                                        cached_colliders
                                            .add(CachedCollider::Single(abstract_collider.clone())),
                                    ));
                                }

                                // Insert the new collider
                                if let Some(collider) = abstract_collider.into() {
                                    let Ok(mut target) = commands.get_entity(entity) else {
                                        continue;
                                    };
                                    #[cfg(all(debug_assertions, feature = "preprocess"))]
                                    bevy::log::info!(
                                        "Generating new collider for entity {:?}.",
                                        entity
                                    );
                                    target.insert(collider);
                                } else {
                                    error!(
                                            "Failed to generate collider from image for entity {:?}: failed to convert to physics collider",
                                            entity
                                        );
                                }
                            } else {
                                error!(
                                    "Failed to generate collider from image for entity {:?}",
                                    entity
                                );
                            }
                        }
                    } else {
                        error!(
                            "Failed to convert image to BinaryImage for entity {:?}",
                            entity
                        );
                    }
                } else {
                    error!(
                        "Failed to retrieve image from handle for entity {:?}",
                        entity
                    );
                }
            }
        } else {
            error!("Failed to retrieve image handle for entity {:?}", entity);
        }
    }
}

#[cfg(feature = "preprocess")]
fn find_collider<TargetCollider>(
    handle: &Handle<Image>,
    saved_query: &Query<&LoadedCollider>,
    cached_colliders: &Assets<CachedCollider>,
    dynamic_collider: &DynamicCollider,
    atlas: Option<&TextureAtlas>,
) -> (Option<TargetCollider>, Option<Vec2>)
where
    AbstractCollider: Into<Option<TargetCollider>>,
    TargetCollider: Component,
{
    // Find a [`CachedCollider`] for the [`Image`]
    if let Some(collider) =
        saved_query
            .iter()
            .find_map(|LoadedCollider(image_handle, collider_handle)| {
                if *handle == *image_handle {
                    cached_colliders.get(collider_handle)
                } else {
                    None
                }
            })
    {
        // Use the [`CachedCollider`] instead of generating a new one
        let collider: (Option<TargetCollider>, Option<Vec2>) = match collider {
            CachedCollider::Single(abstract_collider) => (abstract_collider.clone().into(), None),
            CachedCollider::Multiple(colliders) => {
                let collider_and_pos = dynamic_collider
                    .multiple_index
                    .and_then(|index| colliders.get(index))
                    .map(|(collider, offset)| (collider.clone().into(), Some(*offset)));
                collider_and_pos.unwrap_or_default()
            }
            CachedCollider::Atlas(collider_atlas) => {
                let collider = atlas
                    .as_ref()
                    .map(|atlas| atlas.index)
                    .and_then(|atlas_position| collider_atlas.0.get(atlas_position).cloned())
                    .flatten()
                    .and_then(std::convert::Into::into);
                (collider, None)
            }
        };
        collider
    } else {
        (None, None)
    }
}
