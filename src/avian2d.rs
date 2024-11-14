use avian2d::{
    parry::{math::Point, shape::SharedShape},
    prelude::Collider,
};
use edges::{Edges, Vec2};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{utils::heights_and_scale, ColliderType};

fn to_collider(collider_type: ColliderType, points: Vec<Vec2>) -> Option<Collider> {
    match collider_type {
        ColliderType::Polyline => Some(Collider::polyline(points, None)),
        ColliderType::ConvexPolyline => SharedShape::convex_polyline(
            {
                #[cfg(not(feature = "parallel"))]
                let iterator = points.into_iter();
                #[cfg(feature = "parallel")]
                let iterator = points.into_par_iter();
                iterator
            }
            .map(Point::from)
            .collect(),
        )
        .map(Collider::from),
        ColliderType::ConvexHull => Collider::convex_hull(points),
        ColliderType::Heightfield => {
            let (heights, scale) = heights_and_scale(points);
            Some(Collider::heightfield(heights, scale))
        }
    }
}

#[must_use]
pub fn generate_collider<I>(
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
pub fn generate_colliders<I>(
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
