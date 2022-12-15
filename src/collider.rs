use crate::{
    multi_image_edge_translated, multi_image_edges_raw, single_image_edge_raw,
    single_image_edge_translated,
};
use bevy::prelude::{Image, Vec2};
use bevy_rapier2d::prelude::{Collider, Real};

/// Generate a single bevy_rapier2d polyline collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_polyline_collider_translated(image: &Image) -> Collider {
    Collider::polyline(single_image_edge_translated(image), None)
}

/// Generate a single bevy_rapier2d polyline collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_polyline_collider_raw(image: &Image) -> Collider {
    Collider::polyline(single_image_edge_raw(image), None)
}

/// Generate a single bevy_rapier2d convex_polyline collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_convex_polyline_collider_translated(image: &Image) -> Option<Collider> {
    Collider::convex_polyline(single_image_edge_translated(image))
}

/// Generate a single bevy_rapier2d convex_polyline collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_convex_polyline_collider_raw(image: &Image) -> Option<Collider> {
    Collider::convex_polyline(single_image_edge_raw(image))
}

/// Generate a single bevy_rapier2d convex_hull collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_convex_hull_collider_translated(image: &Image) -> Option<Collider> {
    let points = single_image_edge_translated(image);
    Collider::convex_hull(&points)
}

/// Generate a single bevy_rapier2d convex_hull collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_convex_hull_collider_raw(image: &Image) -> Option<Collider> {
    let points = single_image_edge_translated(image);
    Collider::convex_hull(&points)
}

/// Generate a single bevy_rapier2d heightfield collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_heightfield_collider_translated(image: &Image) -> Collider {
    heightfield_collider_from_points(&single_image_edge_translated(image))
}

/// Generate a single bevy_rapier2d heightfield collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_heightfield_collider_raw(image: &Image) -> Collider {
    heightfield_collider_from_points(&single_image_edge_raw(image))
}

/// Generate as many bevy_rapier2d polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_polyline_collider_translated(image: &Image) -> Vec<Collider> {
    multi_image_edge_translated(image)
        .into_iter()
        .map(|e| Collider::polyline(e, None))
        .collect()
}

/// Generate as many bevy_rapier2d polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_polyline_collider_raw(image: &Image) -> Vec<Collider> {
    multi_image_edges_raw(image)
        .into_iter()
        .map(|e| Collider::polyline(e, None))
        .collect()
}

/// Generate as many bevy_rapier2d convex_polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_convex_polyline_collider_translated(image: &Image) -> Vec<Option<Collider>> {
    multi_image_edge_translated(image)
        .into_iter()
        .map(Collider::convex_polyline)
        .collect()
}

/// Generate as many bevy_rapier2d convex_polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_convex_polyline_collider_raw(image: &Image) -> Vec<Option<Collider>> {
    multi_image_edges_raw(image)
        .into_iter()
        .map(Collider::convex_polyline)
        .collect()
}

/// Generate as many bevy_rapier2d heightfield colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_heightfield_collider_translated(image: &Image) -> Vec<Collider> {
    multi_image_edge_translated(image)
        .into_iter()
        .map(|e| heightfield_collider_from_points(&e))
        .collect()
}

/// Generate as many bevy_rapier2d heightfield colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_heightfield_collider_raw(image: &Image) -> Vec<Collider> {
    multi_image_edges_raw(image)
        .into_iter()
        .map(|e| heightfield_collider_from_points(&e))
        .collect()
}

/// Generate as many bevy_rapier2d convex_hull colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_convex_hull_collider_translated(image: &Image) -> Vec<Option<Collider>> {
    multi_image_edge_translated(image)
        .into_iter()
        .map(|e| Collider::convex_hull(&e))
        .collect()
}

/// Generate as many bevy_rapier2d convex_hull colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_convex_hull_collider_raw(image: &Image) -> Vec<Option<Collider>> {
    multi_image_edges_raw(image)
        .into_iter()
        .map(|e| Collider::convex_hull(&e))
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

    heights.iter().map(|e| e.y).collect::<Vec<Real>>()
}
