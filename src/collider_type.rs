/// An enumeration representing the different types of colliders that can be created.
///
/// This enum is used to specify the type of collider when generating colliders from images or other sources.
#[derive(Clone, Copy, Debug, Default, Hash)]
pub enum ColliderType {
    #[default]
    Polyline,
    ConvexPolyline,
    ConvexHull,
    Heightfield,
}
