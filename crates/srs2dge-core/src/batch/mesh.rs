use wgpu::PrimitiveTopology;

//

pub trait Mesh<V> {
    const PRIM: PrimitiveTopology;

    type VertexIter: Iterator<Item = V>;
    type IndexIter: Iterator<Item = u32>;

    fn vertices(&self) -> Self::VertexIter;
    fn indices(&self, offset: u32) -> Self::IndexIter;

    /// vbo_alloc must match the length of
    /// `Self::VertexIter` from `vertices`
    ///
    /// this is checked only in debug builds
    fn vbo_alloc(&self) -> u32;

    /// ibo_alloc must match the length of
    /// `Self::IndexIter` from `indices`
    ///
    /// this is checked only in debug builds
    fn ibo_alloc(&self) -> u32;
}
