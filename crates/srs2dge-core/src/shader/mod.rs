use self::builder::ShaderBuilder;
use crate::buffer::{index::Index, vertex::Vertex};
use std::{marker::PhantomData, ops::Deref};
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPipeline, TextureFormat};

//

pub mod builder;
pub mod layout;
pub mod module;
pub mod prelude;

//

pub struct Shader<V, I>
where
    V: Vertex,
    I: Index,
{
    pub(crate) pipeline: RenderPipeline,
    pub(crate) format: TextureFormat,

    _p: PhantomData<(V, I)>,
}

//

pub trait Layout<'a> {
    type Bindings;

    fn bind_group_layout(device: &Device) -> BindGroupLayout;
    fn bind_group(&self, bindings: Self::Bindings) -> BindGroup;
}

pub type BindUnit<'a, V, I> = (BindGroup, &'a Shader<V, I>);

pub trait AsBindUnit<V, I, B>
where
    V: Vertex,
    I: Index,
{
    fn bind_unit(&self, bindings: B) -> BindUnit<V, I>;
}

impl<'a, T, V, I> AsBindUnit<V, I, T::Bindings> for T
where
    T: Deref<Target = Shader<V, I>> + Layout<'a>,
    V: Vertex,
    I: Index,
{
    fn bind_unit(&self, bindings: T::Bindings) -> BindUnit<V, I> {
        (self.bind_group(bindings), self.deref())
    }
}

//

impl<V, I> Shader<V, I>
where
    V: Vertex,
    I: Index,
{
    pub fn builder<'s>() -> ShaderBuilder<'s, V, I> {
        ShaderBuilder::<'s, V, I>::new()
    }
}
