use self::prelude::{
    base::{BaseOffset, BaseSize, SelfSize},
    IntoMax,
};
use crate::prelude::{Const, WidgetBase};
use srs2dge_core::glam::Vec2;

//

pub mod base;
pub mod constant;
pub mod max;
pub mod min;
pub mod ops;
pub mod prelude;
pub mod ratio;

//

pub trait GuiCalc {
    /// `base` has the size and the offset of the parent widget
    ///
    /// `self_size` is the size calculated for this widget
    /// `Vec2::ZERO` if it is currently being calculated
    fn reduce(self, base: WidgetBase, self_size: Vec2) -> Vec2;
}

//

pub fn inherit_size() -> BaseSize {
    BaseSize
}

pub fn inherit_offset() -> BaseOffset {
    BaseOffset
}

pub fn align(side: Vec2) -> impl GuiCalc {
    BaseOffset + (BaseSize - SelfSize) * Const(side)
}

pub fn border_size(px: f32) -> impl GuiCalc {
    (BaseSize - Const(Vec2::splat(2.0 * px))).max(Const(Vec2::ZERO))
}

pub fn border_offset(px: f32) -> impl GuiCalc {
    BaseOffset + Const(Vec2::splat(px))
}
