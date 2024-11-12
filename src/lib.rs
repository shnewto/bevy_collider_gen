#![doc = include_str!("../README.md")]

pub use edges::Edges;

#[cfg(feature = "avian2d")]
pub mod avian2d;
#[cfg(feature = "rapier2d")]
pub mod rapier2d;
mod utils;
