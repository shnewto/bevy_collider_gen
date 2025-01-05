#![doc = include_str!("../README.md")]

pub mod prelude {
    pub extern crate edges;
    pub use crate::{
        abstract_collider::{AbstractCollider, AbstractCollidersBuilder},
        collider_type::ColliderType,
    };
    pub use edges::anchor::Anchor;
}

mod abstract_collider;
mod collider_type;
mod utils;
