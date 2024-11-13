use avian2d::{
    parry::{math::Point, shape::SharedShape},
    prelude::Collider,
};
use edges::Edges;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{
    utils::{generate_collider, generate_multi_collider, heights_and_scale},
    ColliderType,
};

/// Generate a single collider from the image.
#[must_use]
pub fn single_collider<I>(
    image: I,
    collider_type: ColliderType,
    translated: bool,
) -> Option<Collider>
where
    Edges: From<I>,
{
    let collider_fn = match collider_type {
        ColliderType::Polyline => |vertices| Some(Collider::polyline(vertices, None)),
        ColliderType::ConvexPolyline => |points: Vec<_>| {
            SharedShape::convex_polyline(
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
            .map(Collider::from)
        },
        ColliderType::ConvexHull => |points| Collider::convex_hull(points),
        ColliderType::Heightfield => |points| {
            let (heights, scale) = heights_and_scale(points);
            Some(Collider::heightfield(heights, scale))
        },
    };
    generate_collider(image, collider_fn, translated)
}

/// Generate as many colliders as it can find in the image.
#[must_use]
pub fn multi_collider<I>(
    image: I,
    collider_type: ColliderType,
    translated: bool,
) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    let collider_fn = match collider_type {
        ColliderType::Polyline => |vertices| Some(Collider::polyline(vertices, None)),
        ColliderType::ConvexPolyline => |points: Vec<_>| {
            SharedShape::convex_polyline(
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
            .map(Collider::from)
        },
        ColliderType::ConvexHull => |points| Collider::convex_hull(points),
        ColliderType::Heightfield => |points| {
            let (heights, scale) = heights_and_scale(points);
            Some(Collider::heightfield(heights, scale))
        },
    };
    generate_multi_collider(image, collider_fn, translated)
}
