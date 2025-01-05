use bevy_rapier2d::prelude::Collider;

use super::AbstractCollider;
use super::AbstractCollider::{ConvexHull, ConvexPolyline, Heightfield, Polyline};

impl AbstractCollider {
    #[must_use]
    pub fn to_rapier(self) -> Option<Collider> {
        self.into()
    }
}

impl From<AbstractCollider> for Option<Collider> {
    fn from(value: AbstractCollider) -> Self {
        match value {
            Polyline(vertices) => Some(Collider::polyline(vertices, None)),
            ConvexPolyline(points) => Collider::convex_polyline(points),
            ConvexHull(points) => Collider::convex_hull(&points),
            Heightfield(heights, scale) => Some(Collider::heightfield(heights, scale)),
        }
    }
}
