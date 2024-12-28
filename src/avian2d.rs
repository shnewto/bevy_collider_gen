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
    let edges = Edges::try_from(image).ok()?;
    let (width, height) = (edges.0.width(), edges.0.height());
    let polygon = edges.single_raw()?;
    match collider_type {
        ColliderType::Polyline => Some(Collider::polyline(translate(polygon, width, height), None)),
        ColliderType::ConvexPolyline => SharedShape::convex_polyline(
            translate(polygon, width, height)
                .into_iter()
                .map(Point::from)
                .collect(),
        )
        .map(Collider::from),
        ColliderType::ConvexHull => Collider::convex_hull(translate(polygon, width, height)),
        ColliderType::Heightfield => {
            let (heights, scale) = heights_and_scale(polygon, height);
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
    if let Ok(edges) = Edges::try_from(image) {
        let (width, height) = (edges.width(), edges.height());
        let iter = edges.iter();

        match collider_type {
            ColliderType::Polyline => iter
                .map(|polygon| Collider::polyline(translate(polygon, width, height), None))
                .collect(),
            ColliderType::ConvexPolyline => iter
                .filter_map(|polygon| {
                    SharedShape::convex_polyline(
                        translate(polygon, width, height)
                            .into_iter()
                            .map(Point::from)
                            .collect(),
                    )
                    .map(Collider::from)
                })
                .collect(),
            ColliderType::ConvexHull => iter
                .filter_map(|polygon| Collider::convex_hull(translate(polygon, width, height)))
                .collect(),
            ColliderType::Heightfield => iter
                .map(|polygon| {
                    let (heights, scale) = heights_and_scale(polygon, height);
                    Collider::heightfield(heights, scale)
                })
                .collect(),
        }
    } else {
        Vec::new()
    }
}
