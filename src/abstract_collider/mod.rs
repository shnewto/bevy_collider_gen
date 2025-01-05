use bevy::prelude::*;

#[cfg(feature = "avian2d")]
mod avian2d;
#[cfg(feature = "rapier2d")]
mod rapier2d;

use AbstractCollider::{ConvexHull, ConvexPolyline, Heightfield, Polyline};

#[derive(Clone, Debug, PartialEq)]
pub enum AbstractCollider {
    Polyline(Vec<Vec2>),
    ConvexPolyline(Vec<Vec2>),
    ConvexHull(Vec<Vec2>),
    Heightfield(Vec<f32>, Vec2),
}

impl AbstractCollider {
    #[must_use]
    pub fn points(&self) -> Option<&Vec<Vec2>> {
        match self {
            Polyline(points) | ConvexPolyline(points) | ConvexHull(points) => Some(points),
            Heightfield(_, _) => None,
        }
    }
}
