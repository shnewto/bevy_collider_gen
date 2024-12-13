use avian2d::{
    parry::{math::Point, shape::SharedShape},
    prelude::Collider,
};
use edges::{translate, Edges};
use image::GenericImageView;

use crate::{utils::heights_and_scale, ColliderType};

/// Generates a collider from the provided image.
///
/// This function processes the input image and creates a collider of the specified type.
///
/// # Returns
/// Returns an `Option<Collider>`. If the collider is successfully generated, it returns `Some(Collider)`,
/// otherwise, it returns `None` if the generation fails.
///
/// # Example
/// ```
/// let collider = generate_collider(image, ColliderType::Polyline);
/// ```
#[must_use]
pub fn generate_collider(
    image: &bevy_image::Image,
    collider_type: ColliderType,
) -> Option<Collider> {
    let Ok(edges) = Edges::try_from(image) else {
        return None;
    };
    let (width, height) = (edges.0.width(), edges.0.height());
    let points = edges.single_raw()?;
    match collider_type {
        ColliderType::Polyline => Some(Collider::polyline(translate(points, width, height), None)),
        ColliderType::ConvexPolyline => SharedShape::convex_polyline(
            translate(points, width, height)
                .into_iter()
                .map(Point::from)
                .collect(),
        )
        .map(Collider::from),
        ColliderType::ConvexHull => Collider::convex_hull(translate(points, width, height)),
        ColliderType::Heightfield => {
            let (heights, scale) = heights_and_scale(points, height);
            Some(Collider::heightfield(heights, scale))
        }
    }
}

/// Generates multiple colliders from the provided image.
///
/// This function processes the input image and creates a vector of colliders of the specified type.
///
/// # Returns
/// Returns a `Vec<Option<Collider>>`, which is a vector of optional colliders. Each element may be `Some(Collider)`
/// if the collider is successfully generated, or `None` if the generation fails for that particular collider.
///
/// # Example
/// ```
/// let colliders = generate_colliders(image, ColliderType::Polyline);
/// ```
#[must_use]
pub fn generate_colliders(image: &bevy_image::Image, collider_type: ColliderType) -> Vec<Collider> {
    let Ok(edges) = Edges::try_from(image) else {
        return Vec::new();
    };
    let (width, height) = (edges.width(), edges.height());
    let iter = edges.iter();

    match collider_type {
        ColliderType::Polyline => iter
            .map(|points| Collider::polyline(translate(points, width, height), None))
            .collect(),
        ColliderType::ConvexPolyline => iter
            .filter_map(|points| {
                SharedShape::convex_polyline(
                    translate(points, width, height)
                        .into_iter()
                        .map(Point::from)
                        .collect(),
                )
                .map(Collider::from)
            })
            .collect(),
        ColliderType::ConvexHull => iter
            .filter_map(|points| Collider::convex_hull(translate(points, width, height)))
            .collect(),
        ColliderType::Heightfield => iter
            .map(|points| {
                let (heights, scale) = heights_and_scale(points, height);
                Collider::heightfield(heights, scale)
            })
            .collect(),
    }
}
