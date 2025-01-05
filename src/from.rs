use bevy::prelude::*;
use edges::binary_image::{self, BinaryImage, BinaryView};
use image::DynamicImage;

use crate::AbstractCollidersBuilder;

impl<'a> From<&'a DynamicImage> for AbstractCollidersBuilder<BinaryView<'a, DynamicImage>> {
    fn from(image: &'a DynamicImage) -> Self {
        Self::new(BinaryView::Ref(image))
    }
}

impl From<DynamicImage> for AbstractCollidersBuilder<BinaryImage> {
    fn from(image: DynamicImage) -> Self {
        Self::new(BinaryImage::from(image))
    }
}

impl TryFrom<&Image> for AbstractCollidersBuilder<BinaryImage> {
    type Error = binary_image::bevy::IntoBinaryImageError;
    fn try_from(image: &Image) -> Result<Self, Self::Error> {
        BinaryImage::try_from(image).map(Self::new)
    }
}

impl TryFrom<Image> for AbstractCollidersBuilder<BinaryImage> {
    type Error = binary_image::bevy::IntoBinaryImageError;
    fn try_from(image: Image) -> Result<Self, Self::Error> {
        BinaryImage::try_from(image).map(Self::new)
    }
}
