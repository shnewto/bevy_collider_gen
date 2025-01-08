use avian2d::{
    parry::{math::Point, shape::SharedShape},
    prelude::Collider,
};

use super::AbstractCollider;
use super::AbstractCollider::{ConvexHull, ConvexPolyline, Heightfield, Polyline};

impl AbstractCollider {
    #[must_use]
    pub fn to_avian(self) -> Option<Collider> {
        self.into()
    }
}

impl From<AbstractCollider> for Option<Collider> {
    fn from(value: AbstractCollider) -> Self {
        match value {
            Polyline(vertices) => Some(Collider::polyline(vertices, None)),
            ConvexPolyline(points) => {
                SharedShape::convex_polyline(points.into_iter().map(Point::from).collect())
                    .map(Collider::from)
            }
            ConvexHull(points) => Collider::convex_hull(points),
            Heightfield(heights, scale) => Some(Collider::heightfield(heights, scale)),
        }
    }
}
