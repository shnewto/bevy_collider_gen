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

/// Generates a collider from the provided image.
///
/// This function processes the input image and creates a collider of the specified type.
/// The collider can be translated based on the `translate` flag.
///
/// # Parameters
/// - `image`: The input image from which to generate the collider. The type `I` must implement `From<I>` for `Edges`.
/// - `collider_type`: Specifies the type of collider to create (e.g., `Polyline`, `ConvexHull`, etc.).
/// - `translate`: A boolean flag indicating whether to apply translation during collider generation.
///
/// # Returns
/// Returns an `Option<Collider>`. If the collider is successfully generated, it returns `Some(Collider)`,
/// otherwise, it returns `None` if the generation fails.
///
/// # Example
/// ```
/// let collider = generate_collider(image, ColliderType::Polyline, true);
/// ```
#[must_use]
pub fn generate_collider<I>(image: I, collider_type: ColliderType) -> Option<Collider>
where
    Edges: From<I>,
{
    crate::utils::generate_collider(image, |points| to_collider(collider_type, points))
}

/// Generates multiple colliders from the provided image.
///
/// This function processes the input image and creates a vector of colliders of the specified type.
/// Each collider can be translated based on the `translate` flag.
///
/// # Parameters
/// - `image`: The input image from which to generate the colliders. The type `I` must implement `From<I>` for `Edges`.
/// - `collider_type`: Specifies the type of colliders to create (e.g., `Polyline`, `ConvexHull`, etc.).
/// - `translate`: A boolean flag indicating whether to apply translation during collider generation.
///
/// # Returns
/// Returns a `Vec<Option<Collider>>`, which is a vector of optional colliders. Each element may be `Some(Collider)`
/// if the collider is successfully generated, or `None` if the generation fails for that particular collider.
///
/// # Example
/// ```
/// let colliders = generate_colliders(image, ColliderType::Polyline, true);
/// ```
#[must_use]
pub fn generate_colliders<I>(image: I, collider_type: ColliderType) -> Vec<Collider>
where
    Edges: From<I>,
{
    crate::utils::generate_colliders(image, |points| to_collider(collider_type, points))
}
