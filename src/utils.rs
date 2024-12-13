use edges::{Edges, Vec2};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// takes x,y points collects the y values at the top of the image (biggest y)
pub fn heights_and_scale(mut points: Vec<Vec2>) -> (Vec<f32>, Vec2) {
    points.sort_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());

    #[cfg(not(feature = "parallel"))]
    let chunk = points.chunk_by(|p1, p2| (p1.x - p2.x).abs() <= f32::EPSILON);
    #[cfg(feature = "parallel")]
    let chunk = points.par_chunk_by(|p1, p2| (p1.x - p2.x).abs() <= f32::EPSILON);

    let heights = chunk
        .map(|chunk| chunk.iter().map(|p| p.y).reduce(f32::max).unwrap())
        .collect::<Vec<f32>>();

    let x_scale = heights.len() - 1;
    (heights, Vec2::new(x_scale as f32, 1.0))
}

/// Generate colliders from the image based on the provided collider type and coordinate handling.
pub fn generate_collider<F, R, I>(image: I, collider_fn: F) -> R
where
    F: Fn(Vec<Vec2>) -> R,
    Edges: From<I>,
{
    let edges = Edges::from(image);
    let points = edges.single_image_edge_translated();
    collider_fn(points)
}

/// Generate multiple colliders from the image based on the provided collider type and coordinate handling.
pub fn generate_colliders<F, R, I>(image: I, collider_fn: F) -> Vec<R>
where
    F: Fn(Vec<Vec2>) -> Option<R> + Send + Sync,
    R: Send,
    Edges: From<I>,
{
    let edges = Edges::from(image);
    let points = edges.multi_image_edge_translated();

    #[cfg(not(feature = "parallel"))]
    let iterator = points.into_iter();
    #[cfg(feature = "parallel")]
    let iterator = points.into_par_iter();

    iterator.filter_map(collider_fn).collect()
}
