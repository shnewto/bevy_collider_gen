use edges::{BinaryImage, BinaryView};
use image::DynamicImage;

use super::Builder;

impl<'a> From<&'a DynamicImage> for Builder<BinaryView<'a, DynamicImage>> {
    fn from(image: &'a DynamicImage) -> Self {
        Self::new(BinaryView::Ref(image))
    }
}

impl From<DynamicImage> for Builder<BinaryImage> {
    fn from(image: DynamicImage) -> Self {
        Self::new(BinaryImage::from(image))
    }
}

impl TryFrom<&bevy::prelude::Image> for Builder<BinaryImage> {
    type Error = edges::IntoBinaryImageError;
    fn try_from(image: &bevy::prelude::Image) -> Result<Self, Self::Error> {
        BinaryImage::try_from(image).map(Self::new)
    }
}

impl TryFrom<bevy::prelude::Image> for Builder<BinaryImage> {
    type Error = edges::IntoBinaryImageError;
    fn try_from(image: bevy::prelude::Image) -> Result<Self, Self::Error> {
        BinaryImage::try_from(image).map(Self::new)
    }
}
