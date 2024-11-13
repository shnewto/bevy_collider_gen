use bevy_rapier2d::prelude::Collider;
use edges::Edges;

use crate::{
    utils::{generate_collider, generate_multi_collider, heights_and_scale},
    ColliderType,
};

/// Generate a single collider from the image.
#[must_use]
pub fn single_collider<I>(
    image: I,
    collider_type: ColliderType,
    translate: bool,
) -> Option<Collider>
where
    Edges: From<I>,
{
    let collider_fn = match collider_type {
        ColliderType::Polyline => |vertices| Some(Collider::polyline(vertices, None)),
        ColliderType::ConvexPolyline => |points| Collider::convex_polyline(points),
        ColliderType::ConvexHull => |points: Vec<_>| Collider::convex_hull(&points),
        ColliderType::Heightfield => |points| {
            let (heights, scale) = heights_and_scale(points);
            Some(Collider::heightfield(heights, scale))
        },
    };
    generate_collider(image, collider_fn, translate)
}

/// Generate as many colliders as it can find in the image.
#[must_use]
pub fn multi_collider<I>(
    image: I,
    collider_type: ColliderType,
    translate: bool,
) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    let collider_fn = match collider_type {
        ColliderType::Polyline => |vertices| Some(Collider::polyline(vertices, None)),
        ColliderType::ConvexPolyline => |points| Collider::convex_polyline(points),
        ColliderType::ConvexHull => |points: Vec<_>| Collider::convex_hull(&points),
        ColliderType::Heightfield => |points| {
            let (heights, scale) = heights_and_scale(points);
            Some(Collider::heightfield(heights, scale))
        },
    };
    generate_multi_collider(image, collider_fn, translate)
}
