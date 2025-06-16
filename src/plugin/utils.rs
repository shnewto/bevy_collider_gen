use bevy::prelude::*;
use edges::binary_image::BinaryImage;
use image::GenericImageView;

fn crop_image(image: BinaryImage, rect: URect) -> BinaryImage {
    let (width, height) = image.dimensions();
    let (x, y) = (rect.min.x, rect.min.y);
    let crop_width = rect.width().min(width.saturating_sub(x));
    let crop_height = rect.height().min(height.saturating_sub(y));
    BinaryImage::from(image.view(x, y, crop_width, crop_height).to_image())
}

pub fn process_image(
    mut image: BinaryImage,
    atlas_rect: Option<URect>,
    custom_size: Option<Vec2>,
    sprite_rect: Option<Rect>,
    flip_x: bool,
    flip_y: bool,
) -> BinaryImage {
    if let Some(rect) = atlas_rect {
        image = crop_image(image, rect);
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if let Some(size) = custom_size {
        image = image.resize(size.x.max(0.) as u32, size.y.max(0.) as u32);
    }

    if let Some(rect) = sprite_rect {
        image = crop_image(image, rect.as_urect());
    }

    if flip_x {
        image = image.flip_vertical();
    }
    if flip_y {
        image = image.flip_horizontal();
    }

    image
}
