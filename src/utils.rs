use bevy::prelude::*;
use edges::{anchor::Anchor, utils::center_of};

/// Calculates the heights and scale based on the given points and anchor.
///
/// This function processes a vector of 2D integer points, sorts them, removes duplicates based on the x-coordinate,
/// and calculates the heights relative to a specified anchor. The heights are determined by the difference between
/// the anchor's y position and the y-coordinates of the points.
pub fn heights_and_scale(mut points: Vec<UVec2>, anchor: Anchor) -> (Vec<f32>, Vec2) {
    // Sort points by their x-coordinate and remove duplicates based on x.
    points.sort_by_cached_key(|p| p.x);
    points.dedup_by(|p1, p2| p1.x == p2.x);

    if let Some(dy) = anchor
        .size()
        .map(|size| size.y as f32 / 2.)
        .or_else(|| center_of(&points).map(|center| center.y))
    {
        let heights: Vec<f32> = points
            .windows(2)
            .flat_map(|win| {
                let (p1, p2) = (win[0], win[1]);
                let y = dy - p2.y as f32;
                (p1.x..p2.x).map(move |_| y)
            })
            .collect();

        let scale = Vec2::new(heights.len() as f32, 1.);
        (heights, scale)
    } else {
        (Vec::new(), Vec2::ONE)
    }
}
