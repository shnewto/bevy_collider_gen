/// An enumeration representing the different types of colliders that can be created.
#[derive(Clone, Copy, Debug, Default, Hash)]
pub enum ColliderType {
    #[default]
    Polyline,
    ConvexPolyline,
    ConvexHull,
    Heightfield,
}
