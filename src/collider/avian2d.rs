use avian2d::{
    math::{Scalar, Vector, Vector2},
    parry::{
        math::{Point, Real},
        shape::SharedShape,
    },
    prelude::Collider,
};
use bevy::prelude::{Image, Vec2};
use edges::Edges;

/// Generate a single polyline collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_polyline_collider_translated(image: &Image) -> Collider {
    let e = Edges::from(image);
    Collider::polyline(e.single_image_edge_translated(), None)
}

/// Generate a single polyline collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_polyline_collider_raw(image: &Image) -> Collider {
    let e = Edges::from(image);
    Collider::polyline(e.single_image_edge_raw(), None)
}

/// Generate a single convex_polyline collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_convex_polyline_collider_translated(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    let points = e
        .single_image_edge_translated()
        .into_iter()
        .map(Point::from)
        .collect::<Vec<Point<Real>>>();
    SharedShape::convex_polyline(points).map(Collider::from)
}

/// Generate a single convex_polyline collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_convex_polyline_collider_raw(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    let points = e
        .single_image_edge_raw()
        .into_iter()
        .map(Point::from)
        .collect::<Vec<Point<Real>>>();
    SharedShape::convex_polyline(points).map(Collider::from)
}

/// Generate a single convex_hull collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_convex_hull_collider_translated(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    let points = e.single_image_edge_translated();
    Collider::convex_hull(points)
}

/// Generate a single convex_hull collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_convex_hull_collider_raw(image: &Image) -> Option<Collider> {
    let e = Edges::from(image);
    let points = e.single_image_edge_translated();
    Collider::convex_hull(points)
}

/// Generate a single heightfield collider from the image,
/// coordinates translated to either side of (0, 0)
pub fn single_heightfield_collider_translated(image: &Image) -> Collider {
    let e = Edges::from(image);
    heightfield_collider_from_points(&e.single_image_edge_translated())
}

/// Generate a single heightfield collider from the image,
/// coordinates left alone and all in positive x and y
pub fn single_heightfield_collider_raw(image: &Image) -> Collider {
    let e = Edges::from(image);
    heightfield_collider_from_points(&e.single_image_edge_raw())
}

/// Generate as many polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_polyline_collider_translated(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(|v| Collider::polyline(v, None))
        .collect()
}

/// Generate as many polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_polyline_collider_raw(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(|v| Collider::polyline(v, None))
        .collect()
}

/// Generate as many convex_polyline colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_convex_polyline_collider_translated(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(|v| {
            let points = v.into_iter().map(Point::from).collect::<Vec<Point<Real>>>();
            SharedShape::convex_polyline(points).map(Collider::from)
        })
        .collect()
}

/// Generate as many convex_polyline colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_convex_polyline_collider_raw(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(|v| {
            let points = v.into_iter().map(Point::from).collect::<Vec<Point<Real>>>();
            SharedShape::convex_polyline(points).map(Collider::from)
        })
        .collect()
}

/// Generate as many heightfield colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_heightfield_collider_translated(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(|v| heightfield_collider_from_points(&v))
        .collect()
}

/// Generate as many heightfield colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_heightfield_collider_raw(image: &Image) -> Vec<Collider> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(|v| heightfield_collider_from_points(&v))
        .collect()
}

/// Generate as many convex_hull colliders as it can find in the image,
/// coordinates translated to either side of (0, 0)
pub fn multi_convex_hull_collider_translated(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edge_translated()
        .into_iter()
        .map(Collider::convex_hull)
        .collect()
}

/// Generate as many convex_hull colliders as it can find in the image,
/// coordinates left alone and all in positive x and y
pub fn multi_convex_hull_collider_raw(image: &Image) -> Vec<Option<Collider>> {
    let e = Edges::from(image);
    e.multi_image_edges_raw()
        .into_iter()
        .map(Collider::convex_hull)
        .collect()
}

/// parses x,y points into y values at the top of the image (smallest y) and creates a
/// heightfield collider
fn heightfield_collider_from_points(v: &[Vec2]) -> Collider {
    let hf: Vec<Scalar> = heights_from_points(v);
    let x_scale: Real = hf.len() as f32 - 1.0;
    let scale: Vector = Vector2::new(x_scale, 1.0);
    Collider::heightfield(hf, scale)
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
