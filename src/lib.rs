#![doc = include_str!("../README.md")]

pub extern crate edges;

#[cfg(feature = "avian2d")]
pub mod avian2d;
#[cfg(feature = "rapier2d")]
pub mod rapier2d;
mod utils;

#[derive(Clone, Copy, Debug)]
pub enum ColliderType {
    Polyline,
    ConvexPolyline,
    ConvexHull,
    Heightfield,
}
