use avian2d::{
    parry::{math::Point, shape::SharedShape},
    prelude::Collider,
};
use edges::{anchor::Anchor, Edges};

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
    anchor: Anchor,
) -> Option<Collider> {
    let edges = Edges::try_from(image).ok()?;
    let polygon = edges.single_raw()?;
    if matches!(collider_type, ColliderType::Heightfield) {
        let (heights, scale) = heights_and_scale(polygon, anchor);
        Some(Collider::heightfield(heights, scale))
    } else {
        let polygon = anchor.translate(polygon);
        match collider_type {
            ColliderType::Polyline => Some(Collider::polyline(polygon, None)),
            ColliderType::ConvexPolyline => {
                SharedShape::convex_polyline(polygon.into_iter().map(Point::from).collect())
                    .map(Collider::from)
            }
            ColliderType::ConvexHull => Collider::convex_hull(polygon),
            ColliderType::Heightfield => unreachable!(),
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
pub fn generate_colliders(
    image: &bevy_image::Image,
    collider_type: ColliderType,
    anchor: Anchor,
) -> Vec<Collider> {
    if let Ok(edges) = Edges::try_from(image) {
        let iter = edges.iter();

        if matches!(collider_type, ColliderType::Heightfield) {
            iter.map(|polygon| {
                let (heights, scale) = heights_and_scale(polygon, anchor);
                Collider::heightfield(heights, scale)
            })
            .collect()
        } else {
            let polygons = anchor.translate_polygons(iter).into_iter();
            match collider_type {
                ColliderType::Polyline => polygons
                    .map(|polygon| Collider::polyline(polygon, None))
                    .collect(),
                ColliderType::ConvexPolyline => polygons
                    .filter_map(|polygon| {
                        SharedShape::convex_polyline(polygon.into_iter().map(Point::from).collect())
                            .map(Collider::from)
                    })
                    .collect(),
                ColliderType::ConvexHull => polygons.filter_map(Collider::convex_hull).collect(),
                ColliderType::Heightfield => unreachable!(),
            }
        }
    } else {
        Vec::new()
    }
}
