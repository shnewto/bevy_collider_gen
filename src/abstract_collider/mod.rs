use bevy::prelude::*;

pub use builder::Builder as AbstractCollidersBuilder;

use AbstractCollider::{ConvexHull, ConvexPolyline, Heightfield, Polyline};

#[cfg(feature = "avian2d")]
mod avian2d;
#[cfg(feature = "rapier2d")]
mod rapier2d;

mod builder;

/// An enumeration representing different types of abstract colliders.
#[derive(Clone, Debug, PartialEq)]
pub enum AbstractCollider {
    Polyline(Vec<Vec2>),
    ConvexPolyline(Vec<Vec2>),
    ConvexHull(Vec<Vec2>),
    Heightfield(Vec<f32>, Vec2),
}

impl AbstractCollider {
    /// Returns the points of the collider if applicable.
    ///
    /// This method will return the points for `Polyline`, `ConvexPolyline`, and `ConvexHull`.
    /// For `Heightfield`, it returns `None` because it does not have a set of points in the same way.
    #[must_use]
    pub fn points(&self) -> Option<&Vec<Vec2>> {
        match self {
            Polyline(points) | ConvexPolyline(points) | ConvexHull(points) => Some(points),
            Heightfield(_, _) => None,
        }
    }
}
