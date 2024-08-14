#![doc = include_str!("../README.md")]

mod collider;

#[cfg(feature = "rapier2d")]
pub use collider::rapier2d;

#[cfg(feature = "avian2d")]
pub use collider::avian2d;

pub use ::edges::Edges;
