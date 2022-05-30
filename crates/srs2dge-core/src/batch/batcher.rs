use super::{
    mesh::Mesh,
    prelude::{DefaultVertex, IndexBuffer, QuadMesh, Target, Vertex, VertexBuffer},
};
use crate::Frame;
use std::collections::BTreeMap;
use wgpu::Buffer;

//

#[derive(Debug)]
pub struct BatchRenderer<M = QuadMesh, V = DefaultVertex>
where
    M: Mesh<V>,
    V: Vertex + Copy,
{
    vbo: VertexBuffer<V>,
    vbo_next: Offset,
    vbo_free: BTreeMap<Len, Offset>,
    vbo_req: Len,
    ibo: IndexBuffer<u32>,
    ibo_next: Offset,
    ibo_free: BTreeMap<Len, Offset>,
    ibo_req: Len,

    modified: Vec<(Option<M>, Idx)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Idx {
    vbo: BufferAlloc,
    ibo: BufferAlloc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BufferAlloc {
    /// offset in elements, not bytes
    offset: Offset,
    /// len in elements, not bytes
    len: Len,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Offset(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Len(pub u64);

//

impl<M, V> BatchRenderer<M, V>
where
    M: Mesh<V>,
    V: Vertex + Copy,
{
    pub fn new(target: &Target) -> Self {
        Self {
            vbo: VertexBuffer::new(target, 0),
            vbo_next: Offset(0),
            vbo_free: Default::default(),
            vbo_req: Len(0),
            ibo: IndexBuffer::new(target, 0),
            ibo_next: Offset(0),
            ibo_free: Default::default(),
            ibo_req: Len(0),

            modified: Default::default(),
        }
    }

    pub fn push_with(&mut self, mesh: M) -> Idx {
        let idx = self
            .try_push_old(&mesh)
            .unwrap_or_else(|| self.push_new(&mesh));

        self.modify(mesh, idx);

        idx
    }

    pub fn push(&mut self) -> Idx
    where
        M: Default,
    {
        self.push_with(Default::default())
    }

    pub fn modify(&mut self, mesh: M, idx: Idx) {
        self.modified.push((Some(mesh), idx));
    }

    pub fn drop(&mut self, idx: Idx) {
        self.modified.push((None, idx));
        self.vbo_free.insert(idx.vbo.len, idx.vbo.offset);
        self.ibo_free.insert(idx.ibo.len, idx.ibo.offset);

        // TODO: combine free spots + remove trailing free spots
    }

    pub fn generate(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
    ) -> (&'_ VertexBuffer<V>, &'_ IndexBuffer<u32>, u32) {
        self.ensure_vbo_capacity(target, frame);
        self.ensure_ibo_capacity(target, frame);

        for (mesh, idx) in self.modified.drain(..) {
            if let Some(mesh) = mesh {
                // upload newly generated vbo and ibo
                let vertices = mesh.vertices();
                let indices = mesh.indices(idx.vbo.offset.0 as _);

                #[cfg(debug_assertions)]
                let (vertices, indices) = {
                    let vertices = vertices.collect::<Vec<_>>();
                    let indices = indices.collect::<Vec<_>>();

                    assert_eq!(
                        vertices.len() as u64,
                        idx.vbo.len.0,
                        "Vertex count doesn't match the allocated vertex count"
                    );
                    assert_eq!(
                        indices.len() as u64,
                        idx.ibo.len.0,
                        "Index count doesn't match the allocated index count"
                    );

                    (vertices, indices)
                };

                self.vbo
                    .upload_iter(target, frame, idx.vbo.offset.0, idx.vbo.len.0, vertices);
                self.ibo
                    .upload_iter(target, frame, idx.ibo.offset.0, idx.ibo.len.0, indices);
            } else {
                // fill with primitive restart indices
                self.ibo.upload_iter(
                    target,
                    frame,
                    idx.ibo.offset.0,
                    idx.ibo.len.0,
                    std::iter::repeat(!0),
                );
            }
        }

        (&self.vbo, &self.ibo, self.ibo_req.0 as _)
    }

    fn ensure_ibo_capacity(&mut self, target: &mut Target, frame: &mut Frame) {
        if self.ibo_req.0 as usize > self.ibo.capacity() {
            let new_ibo = IndexBuffer::new(target, 2 * self.ibo_req.0 as usize);
            let size = self.ibo.capacity();
            Self::copy_old(frame, self.ibo.inner(), new_ibo.inner(), size);
            self.ibo = new_ibo;
        }
    }

    fn ensure_vbo_capacity(&mut self, target: &mut Target, frame: &mut Frame) {
        if self.vbo_req.0 as usize > self.vbo.capacity() {
            let new_vbo = VertexBuffer::new(target, 2 * self.vbo_req.0 as usize);
            let size = self.vbo.capacity();
            Self::copy_old(frame, self.vbo.inner(), new_vbo.inner(), size);
            self.vbo = new_vbo;
        }
    }

    fn copy_old(frame: &mut Frame, old: &Buffer, new: &Buffer, size: usize) {
        frame
            .encoder()
            .copy_buffer_to_buffer(old, 0, new, 0, size as u64)
    }

    fn pre_alloc_from(required: Len, map: &mut BTreeMap<Len, Offset>) -> Option<(Len, Offset)> {
        // find a spot where this mesh can fit into
        map.iter()
            .find(|(len_avail, _)| **len_avail >= required)
            .map(|(len, off)| (*len, *off))
    }

    fn post_alloc_from(
        required: Len,
        (len, offset): (Len, Offset),
        map: &mut BTreeMap<Len, Offset>,
    ) -> BufferAlloc {
        // <remove>/<decrease the capacity of> this spot
        map.remove(&len);
        if len != required {
            assert!(len > required);
            let left_len = Len(len.0 - required.0);
            let left_offset = Offset(offset.0 + required.0);
            map.insert(left_len, left_offset);
        }
        let len = required;
        BufferAlloc { len, offset }
    }

    fn try_push_old(&mut self, mesh: &M) -> Option<Idx> {
        let vbo_len = Len(mesh.vbo_alloc() as _);
        let ibo_len = Len(mesh.ibo_alloc() as _);
        let vbo_a = Self::pre_alloc_from(vbo_len, &mut self.vbo_free)?;
        let ibo_a = Self::pre_alloc_from(ibo_len, &mut self.ibo_free)?;

        let vbo = Self::post_alloc_from(vbo_len, vbo_a, &mut self.vbo_free);
        let ibo = Self::post_alloc_from(ibo_len, ibo_a, &mut self.ibo_free);

        Some(Idx { vbo, ibo })
    }

    fn push_new(&mut self, mesh: &M) -> Idx {
        let offset = self.vbo_next;
        let len = Len(mesh.vbo_alloc() as _);
        self.vbo_next.0 += len.0;
        self.vbo_req.0 += len.0;
        let vbo = BufferAlloc { offset, len };

        let offset = self.ibo_next;
        let len = Len(mesh.ibo_alloc() as _);
        self.ibo_next.0 += len.0;
        self.ibo_req.0 += len.0;
        let ibo = BufferAlloc { offset, len };

        Idx { vbo, ibo }
    }
}
