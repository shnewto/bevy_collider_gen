use bevy_rapier2d::prelude::Collider;
use edges::{anchor::Anchor, Edges};

use crate::{utils::heights_and_scale, ColliderType};

/// Generates a collider from the provided image.
/// This function processes the input image and creates a collider of the specified type.
///
/// # Example
/// ```
/// let collider = generate_collider(image, ColliderType::Polyline, Anchor::AbsoluteCenter);
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
            ColliderType::ConvexPolyline => Collider::convex_polyline(polygon),
            ColliderType::ConvexHull => Collider::convex_hull(&polygon),
            ColliderType::Heightfield => unreachable!(),
        }
    }
}

/// Generates multiple colliders from the provided image.
/// This function processes the input image and creates a vector of colliders of the specified type.
///
/// # Example
/// ```
/// let colliders = generate_colliders(image, ColliderType::Polyline, Anchor::AbsoluteCenter);
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
                ColliderType::ConvexPolyline => {
                    polygons.filter_map(Collider::convex_polyline).collect()
                }
                ColliderType::ConvexHull => polygons
                    .filter_map(|polygon| Collider::convex_hull(&polygon))
                    .collect(),
                ColliderType::Heightfield => unreachable!(),
            }
        }
    } else {
        Vec::new()
    }
}
