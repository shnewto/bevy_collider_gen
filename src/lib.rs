#![doc = include_str!("../README.md")]

mod collider;

#[cfg(feature = "rapier2d")]
pub use collider::rapier2d;

#[cfg(feature = "xpbd_2d")]
pub use collider::xpbd_2d;

pub use::edges::Edges;
