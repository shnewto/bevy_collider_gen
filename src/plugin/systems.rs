use bevy::prelude::*;
use edges::binary_image::BinaryImage;

use super::{utils::process_image, DynamicCollider};
use crate::prelude::{AbstractCollider, AbstractCollidersBuilder};

type Filter<TargetCollider> = Or<(
    Without<TargetCollider>,
    Added<Sprite>,
    Changed<Sprite>,
    Changed<DynamicCollider>,
)>;

pub fn update_colliders<TargetCollider>(
    mut commands: Commands,
    query: Query<(Entity, &DynamicCollider, Option<&Sprite>), Filter<TargetCollider>>,
    images: Res<Assets<Image>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
) where
    AbstractCollider: Into<Option<TargetCollider>>,
    TargetCollider: Component,
{
    for (entity, dynamic_collider, sprite) in &query {
        let Ok(mut target) = commands.get_entity(entity) else {
            continue;
        };
        let (handle, atlas, size, rect) = dynamic_collider.merge_with_sprite(sprite);
        let (flip_x, flip_y) = sprite
            .map(|sprite| (sprite.flip_x, sprite.flip_y))
            .unwrap_or_default();

        if let Some(handle) = handle {
            if let Some(image) = images.get(handle.id()) {
                if let Ok(binary_image) = BinaryImage::try_from(image) {
                    let processed_image = process_image(
                        binary_image,
                        atlas.and_then(|atlas| atlas.texture_rect(&layouts)),
                        size,
                        rect,
                        flip_x,
                        flip_y,
                    );

                    if let Some(collider) = AbstractCollidersBuilder::new(processed_image)
                        .with_type(dynamic_collider.collider_type)
                        .single()
                        .and_then(Into::<Option<TargetCollider>>::into)
                    {
                        target.insert(collider);
                    } else {
                        error!(
                            "Failed to generate collider from image for entity {:?}",
                            entity
                        );
                    }
                } else {
                    error!(
                        "Failed to convert image to BinaryImage for entity {:?}",
                        entity
                    );
                }
            }
        } else {
            error!("Failed to retrieve image handle for entity {:?}", entity);
        }
    }
}
