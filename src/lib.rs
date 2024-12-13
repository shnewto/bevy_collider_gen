#![doc = include_str!("../README.md")]

#[cfg(feature = "avian2d")]
pub mod avian2d;
#[cfg(feature = "rapier2d")]
pub mod rapier2d;

mod utils;

pub extern crate edges;
cfg_if::cfg_if! {
    if #[cfg(feature = "rapier2d")] {
        pub use rapier2d::{generate_collider, generate_colliders};
    } else if #[cfg(feature = "avian2d")] {
        pub use avian2d::{generate_collider, generate_colliders};
    }
}

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
