#![doc = include_str!("../README.md")]

#[cfg(all(not(feature = "rapier2d"), not(feature = "xpbd_2d")))]
compile_error!("At least one of the features `rapier2d` or `xpbd_2d` must be enabled.");

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
