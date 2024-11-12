use edges::Edges;

use crate::utils::{
    avian2d::{convex_hull_collider, convex_polyline_collider, heightfield_collider, Collider},
    generate_collider, generate_multi_collider,
};

/// Generate a single polyline collider from the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn single_polyline_collider_translated<I>(image: I) -> Collider
where
    Edges: From<I>,
{
    generate_collider(image, |points| Collider::polyline(points, None), true)
}

/// Generate a single polyline collider from the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn single_polyline_collider_raw<I>(image: I) -> Collider
where
    Edges: From<I>,
{
    generate_collider(image, |points| Collider::polyline(points, None), false)
}

/// Generate a single `convex_polyline` collider from the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn single_convex_polyline_collider_translated<I>(image: I) -> Option<Collider>
where
    Edges: From<I>,
{
    generate_collider(image, convex_polyline_collider, true)
}

/// Generate a single `convex_polyline` collider from the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn single_convex_polyline_collider_raw<I>(image: I) -> Option<Collider>
where
    Edges: From<I>,
{
    generate_collider(image, convex_polyline_collider, false)
}

/// Generate a single `convex_hull` collider from the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn single_convex_hull_collider_translated<I>(image: I) -> Option<Collider>
where
    Edges: From<I>,
{
    generate_collider(image, convex_hull_collider, true)
}

/// Generate a single `convex_hull` collider from the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn single_convex_hull_collider_raw<I>(image: I) -> Option<Collider>
where
    Edges: From<I>,
{
    generate_collider(image, convex_hull_collider, false)
}

/// Generate a single heightfield collider from the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn single_heightfield_collider_translated<I>(image: I) -> Collider
where
    Edges: From<I>,
{
    generate_collider(image, heightfield_collider, true)
}

/// Generate a single heightfield collider from the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn single_heightfield_collider_raw<I>(image: I) -> Collider
where
    Edges: From<I>,
{
    generate_collider(image, heightfield_collider, false)
}

/// Generate as many polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn multi_polyline_collider_translated<I>(image: I) -> Vec<Collider>
where
    Edges: From<I>,
{
    generate_multi_collider(image, |points| Collider::polyline(points, None), true)
}

/// Generate as many polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn multi_polyline_collider_raw<I>(image: I) -> Vec<Collider>
where
    Edges: From<I>,
{
    generate_multi_collider(image, |points| Collider::polyline(points, None), false)
}

/// Generate as many `convex_polyline` colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn multi_convex_polyline_collider_translated<I>(image: I) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    generate_multi_collider(image, convex_polyline_collider, true)
}

/// Generate as many `convex_polyline` colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn multi_convex_polyline_collider_raw<I>(image: I) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    generate_multi_collider(image, convex_polyline_collider, false)
}

/// Generate as many heightfield colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn multi_heightfield_collider_translated<I>(image: I) -> Vec<Collider>
where
    Edges: From<I>,
{
    generate_multi_collider(image, heightfield_collider, true)
}

/// Generate as many heightfield colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn multi_heightfield_collider_raw<I>(image: I) -> Vec<Collider>
where
    Edges: From<I>,
{
    generate_multi_collider(image, heightfield_collider, false)
}

/// Generate as many `convex_hull` colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
#[must_use]
pub fn multi_convex_hull_collider_translated<I>(image: I) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    generate_multi_collider(image, convex_hull_collider, true)
}

/// Generate as many `convex_hull` colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
#[must_use]
pub fn multi_convex_hull_collider_raw<I>(image: I) -> Vec<Option<Collider>>
where
    Edges: From<I>,
{
    generate_multi_collider(image, convex_hull_collider, false)
}
