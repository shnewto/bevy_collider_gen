use bevy_math::{UVec2, Vec2};

/// takes x,y points collects the y values at the top of the image (biggest y)
pub fn heights_and_scale(mut points: Vec<UVec2>, height: u32) -> (Vec<f32>, Vec2) {
    points.sort_by_cached_key(|p| p.x);
    points
        .chunk_by_mut(|p1, p2| p1 == p2)
        .for_each(|ch| ch.sort_by(|p1, p2| p2.y.cmp(&p1.y)));
    points.dedup_by(|p1, p2| p1.x == p2.x);

    let dy = height as f32 / 2.;
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
}
