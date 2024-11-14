use bevy_rapier2d::prelude::Collider;
use edges::{Edges, Vec2};

use crate::{
    utils::{generate_collider, generate_multi_collider, heights_and_scale},
    ColliderType,
};

fn to_collider(collider_type: ColliderType, points: Vec<Vec2>) -> Option<Collider> {
    match collider_type {
        ColliderType::Polyline => Some(Collider::polyline(points, None)),
        ColliderType::ConvexPolyline => Collider::convex_polyline(points),
        ColliderType::ConvexHull => Collider::convex_hull(&points),
        ColliderType::Heightfield => {
            let (heights, scale) = heights_and_scale(points);
            Some(Collider::heightfield(heights, scale))
        }
    }
}

#[must_use]
pub fn single_collider<I>(
    image: I,
    collider_type: ColliderType,
    translate: bool,
) -> Option<Collider>
where
    Edges: From<I>,
{
    crate::utils::generate_collider(
        image,
        |points| to_collider(collider_type, points),
        translate,
    )
}

#[must_use]
pub fn multi_collider<I>(
    image: I,
    collider_type: ColliderType,
    translate: bool,
) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    crate::utils::generate_colliders(
        image,
        |points| to_collider(collider_type, points),
        translate,
    )
}
