use edges::Edges;
use bevy::prelude::{Image, Vec2};
use bevy_rapier2d::prelude::{Collider, Real};

/// Generate a single bevy_rapier2d polyline collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_polyline_collider_translated(image: &Image) -> Collider {
    let e = Edges::from(image);
    Collider::polyline(e.single_image_edge_translated(), None)
}

/// Generate a single bevy_rapier2d polyline collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_polyline_collider_raw(image: &Image) -> Collider {
    let e = Edges::from(image);
    Collider::polyline(e.single_image_edge_raw(), None)
}

/// Generate a single bevy_rapier2d convex_polyline collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_convex_polyline_collider_translated(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    Collider::convex_polyline(e.single_image_edge_translated())
}

/// Generate a single bevy_rapier2d convex_polyline collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_convex_polyline_collider_raw(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    Collider::convex_polyline(e.single_image_edge_raw())
}

/// Generate a single bevy_rapier2d convex_hull collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_convex_hull_collider_translated(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    let points = e.single_image_edge_translated();
    Collider::convex_hull(&points)
}

/// Generate a single bevy_rapier2d convex_hull collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_convex_hull_collider_raw(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    let points = e.single_image_edge_translated();
    Collider::convex_hull(&points)
}

/// Generate a single bevy_rapier2d heightfield collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_heightfield_collider_translated(image: &Image) -> Collider {
    let e = Edges::from(image);
    heightfield_collider_from_points(&e.single_image_edge_translated())
}

/// Generate a single bevy_rapier2d heightfield collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_heightfield_collider_raw(image: &Image) -> Collider {
    let e = Edges::from(image);
    heightfield_collider_from_points(&e.single_image_edge_raw())
}

/// Generate as many bevy_rapier2d polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_polyline_collider_translated(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(|v| Collider::polyline(v, None))
        .collect()
}

/// Generate as many bevy_rapier2d polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_polyline_collider_raw(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(|v| Collider::polyline(v, None))
        .collect()
}

/// Generate as many bevy_rapier2d convex_polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_convex_polyline_collider_translated(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(Collider::convex_polyline)
        .collect()
}

/// Generate as many bevy_rapier2d convex_polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_convex_polyline_collider_raw(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(Collider::convex_polyline)
        .collect()
}

/// Generate as many bevy_rapier2d heightfield colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_heightfield_collider_translated(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(|v| heightfield_collider_from_points(&v))
        .collect()
}

/// Generate as many bevy_rapier2d heightfield colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_heightfield_collider_raw(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(|v| heightfield_collider_from_points(&v))
        .collect()
}

/// Generate as many bevy_rapier2d convex_hull colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_convex_hull_collider_translated(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(|v| Collider::convex_hull(&v))
        .collect()
}

/// Generate as many bevy_rapier2d convex_hull colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_convex_hull_collider_raw(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(|v| Collider::convex_hull(&v))
        .collect()
}

/// parses x,y points into y values at the top of the image (smallest y) and creates a
/// bevy_rapier2d heightfield collider
fn heightfield_collider_from_points(v: &[Vec2]) -> Collider {
    let hf = heights_from_points(v);
    let x_scale = hf.len() as f32 - 1.0;
    Collider::heightfield(hf, Vec2::new(x_scale, 1.0))
}

/// takes x,y points collects the y values at the top of the image (smallest y)
fn heights_from_points(points: &[Vec2]) -> Vec<Real> {
    let mut heights: Vec<Vec2> = vec![];

    for p in points {
        let elem = heights.iter().enumerate().find(|(_, e)| e.x == p.x);
        if let Some((i, e)) = elem {
            if e.y < p.y {
                heights.remove(i);
                heights.insert(i, *p);
            }
        } else {
            heights.push(*p);
        }
    }

    heights.iter().map(|v| v.y).collect::<Vec<Real>>()
}
