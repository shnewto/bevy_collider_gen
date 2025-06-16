use bevy::{prelude::*, sprite::Anchor};

use crate::prelude::ColliderType;

#[derive(Component, Clone, Debug, Default)]
pub struct DynamicCollider {
    pub collider_type: ColliderType,
    pub image: Option<Handle<Image>>,
    pub texture_atlas: Option<TextureAtlas>,
    pub custom_size: Option<Vec2>,
    pub rect: Option<Rect>,
    pub anchor: Anchor,
}

impl DynamicCollider {
    #[must_use]
    pub fn sized(custom_size: Vec2) -> Self {
        Self {
            custom_size: Some(custom_size),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn from_image(image: Handle<Image>) -> Self {
        Self {
            image: Some(image),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn from_atlas_image(image: Handle<Image>, atlas: TextureAtlas) -> Self {
        Self {
            image: Some(image),
            texture_atlas: Some(atlas),
            ..Default::default()
        }
    }

    pub(crate) fn merge_with_sprite<'a>(&'a self, sprite: Option<&'a Sprite>) -> MergedVisuals<'a> {
        let handle = self.image.as_ref().or(sprite.map(|s| &s.image));
        let atlas = self
            .texture_atlas
            .as_ref()
            .or(sprite.and_then(|s| s.texture_atlas.as_ref()));
        let size = self.custom_size.or(sprite.and_then(|s| s.custom_size));
        let rect = self.rect.or(sprite.and_then(|s| s.rect));
        (handle, atlas, size, rect)
    }
}

type MergedVisuals<'a> = (
    Option<&'a Handle<Image>>,
    Option<&'a TextureAtlas>,
    Option<Vec2>,
    Option<Rect>,
);

impl From<Handle<Image>> for DynamicCollider {
    fn from(image: Handle<Image>) -> Self {
        Self::from_image(image)
    }
}
