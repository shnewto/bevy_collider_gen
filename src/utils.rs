use bevy_render::prelude::Image;
use edges::{Edges, Vec2};
use rayon::prelude::*;

#[cfg(feature = "avian2d")]
pub mod avian2d {
    use super::{heights_and_scale, Vec2};

    use avian2d::parry::{math::Point, shape::SharedShape};
    use rayon::prelude::*;

    pub use avian2d::prelude::Collider;

    /// Generate `convex_polyline` collider from the points,
    pub fn convex_polyline_collider(points: Vec<Vec2>) -> Option<Collider> {
        SharedShape::convex_polyline(points.into_par_iter().map(Point::from).collect())
            .map(Collider::from)
    }

    /// Generate `convex_hull` collider from the points,
    pub fn convex_hull_collider(points: Vec<Vec2>) -> Option<Collider> {
        Collider::convex_hull(points)
    }

    /// takes x,y points collects the y values at the top of the image (smallest y)
    /// and creates a heightfield collider
    pub fn heightfield_collider(v: Vec<Vec2>) -> Collider {
        let (heights, scale) = heights_and_scale(v);
        Collider::heightfield(heights, scale)
    }
}

#[cfg(feature = "rapier2d")]
pub mod rapier2d {
    use super::{heights_and_scale, Vec2};

    pub use bevy_rapier2d::prelude::Collider;

    /// Generate `convex_polyline` collider from the points,
    pub fn convex_polyline_collider(points: Vec<Vec2>) -> Option<Collider> {
        Collider::convex_polyline(points)
    }

    /// Generate `convex_hull` collider from the points,
    #[allow(clippy::needless_pass_by_value)]
    pub fn convex_hull_collider(points: Vec<Vec2>) -> Option<Collider> {
        Collider::convex_hull(&points)
    }

    /// takes x,y points collects the y values at the top of the image (biggest y)
    /// and creates a heightfield collider
    pub fn heightfield_collider(v: Vec<Vec2>) -> Collider {
        let (heights, scale) = heights_and_scale(v);
        Collider::heightfield(heights, scale)
    }
}

/// takes x,y points collects the y values at the top of the image (biggest y)
fn heights_and_scale(mut points: Vec<Vec2>) -> (Vec<f32>, Vec2) {
    points.sort_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());
    let heights = points
        .par_chunk_by(|p1, p2| (p1.x - p2.x).abs() <= f32::EPSILON)
        .map(|chunk| chunk.iter().map(|p| p.y).reduce(f32::max).unwrap())
        .collect::<Vec<f32>>();

    let x_scale = heights.len() - 1;
    (heights, Vec2::new(x_scale as f32, 1.0))
}

/// Generate colliders from the image based on the provided collider type and coordinate handling.
pub fn generate_collider<F, T>(image: &Image, collider_fn: F, translated: bool) -> T
where
    F: Fn(Vec<Vec2>) -> T,
{
    let edges = Edges::from(image);
    collider_fn(
        edges
            .image_edges(translated)
            .into_par_iter()
            .flatten()
            .collect(),
    )
}

/// Generate multiple colliders from the image based on the provided collider type and coordinate handling.
pub fn generate_multi_collider<F, R>(image: &Image, collider_fn: F, translated: bool) -> Vec<R>
where
    F: Fn(Vec<Vec2>) -> R + Send + Sync,
    R: Send,
{
    let edges = Edges::from(image);
    edges
        .image_edges(translated)
        .into_par_iter()
        .map(collider_fn)
        .collect()
}
