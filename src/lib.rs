#![doc = include_str!("../README.md")]

mod collider;
mod edge;

#[cfg(feature = "rapier2d")]
pub use collider::rapier2d;

#[cfg(feature = "xpbd_2d")]
pub use collider::xpbd_2d;

pub use edge::image_to_edges;
pub use edge::march_edges;
pub use edge::multi_image_edge_translated;
pub use edge::multi_image_edges_raw;
pub use edge::single_image_edge_raw;
pub use edge::single_image_edge_translated;
pub use edge::translate_vec;
