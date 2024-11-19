use edges::{Edges, Vec2};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// takes x,y points collects the y values at the top of the image (biggest y)
pub fn heights_and_scale(mut points: Vec<Vec2>) -> (Vec<f32>, Vec2) {
    points.sort_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());
    let heights = {
        #[cfg(not(feature = "parallel"))]
        let chunk = points.chunk_by(|p1, p2| (p1.x - p2.x).abs() <= f32::EPSILON);

        #[cfg(feature = "parallel")]
        let chunk = points.par_chunk_by(|p1, p2| (p1.x - p2.x).abs() <= f32::EPSILON);
        chunk
    }
    .map(|chunk| chunk.iter().map(|p| p.y).reduce(f32::max).unwrap())
    .collect::<Vec<f32>>();

    let x_scale = heights.len() - 1;
    (heights, Vec2::new(x_scale as f32, 1.0))
}

/// Generate colliders from the image based on the provided collider type and coordinate handling.
pub fn generate_collider<F, R, I>(image: I, collider_fn: F, translate: bool) -> R
where
    F: Fn(Vec<Vec2>) -> R,
    Edges: From<I>,
{
    let edges = Edges::from(image);
    let points = edges.image_edges(translate);
    collider_fn(
        {
            #[cfg(not(feature = "parallel"))]
            let iterator = points.into_iter();

            #[cfg(feature = "parallel")]
            let iterator = points.into_par_iter();
            iterator
        }
        .flatten()
        .collect(),
    )
}

/// Generate multiple colliders from the image based on the provided collider type and coordinate handling.
pub fn generate_colliders<F, R, I>(image: I, collider_fn: F, translate: bool) -> Vec<R>
where
    F: Fn(Vec<Vec2>) -> R + Send + Sync,
    R: Send,
    Edges: From<I>,
{
    let edges = Edges::from(image);
    let points = edges.image_edges(translate);
    {
        #[cfg(not(feature = "parallel"))]
        let iterator = points.into_iter();

        #[cfg(feature = "parallel")]
        let iterator = points.into_par_iter();
        iterator
    }
    .map(collider_fn)
    .collect()
}
