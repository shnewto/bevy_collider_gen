#![doc = include_str!("../README.md")]

pub extern crate edges;

#[cfg(feature = "avian2d")]
pub mod avian2d;
#[cfg(feature = "rapier2d")]
pub mod rapier2d;
mod utils;

/// An enumeration representing the different types of colliders that can be created.
///
/// This enum is used to specify the type of collider when generating colliders from images or other sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColliderType {
    Polyline,
    ConvexPolyline,
    ConvexHull,
    Heightfield,
}
