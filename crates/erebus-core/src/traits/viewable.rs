// === Impl ===

/// Trait for structures that can produce zero-copy views of themselves.
/// Implemented by both [`Vector<T>`] and [`VectorData<T>`] for now.
pub trait Viewable<'a, T> {
    /// Associated view type (e.g. `VectorView<'a, T>` or `VectorDataView<'a, T>`).
    type ViewType;

    /// Returns a borrowed, zero-copy view.
    fn view(&'a self) -> Self::ViewType;
}