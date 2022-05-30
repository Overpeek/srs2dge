use self::base::WidgetBase;
use srs2dge_core::{batch::BatchRenderer, prelude::PositionedRect};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

//

pub mod base;
pub mod empty;
pub mod prelude;
pub mod root;
pub mod sub;

//

pub trait Widget {
    /// Used for debugging
    fn name(&self) -> &str;
    fn base(&self) -> &WidgetBase;
    fn base_mut(&mut self) -> &mut WidgetBase;

    #[allow(unused_variables)]
    fn react(&mut self, event: WidgetEvent, batcher: &mut BatchRenderer) {}

    #[allow(unused_variables)]
    fn draw(&mut self, batcher: &mut BatchRenderer) {}
}

impl dyn Widget {
    pub(crate) fn start_draw(&mut self, batcher: &mut BatchRenderer) {
        self.draw(batcher);
        self.base_mut().draw_recurse(batcher);
    }

    pub(crate) fn start_react(&mut self, event: WidgetEvent, batcher: &mut BatchRenderer) {
        self.react(event, batcher);
        self.base_mut().react_recurse(event, batcher);
    }
}

impl Deref for dyn Widget {
    type Target = WidgetBase;

    fn deref(&self) -> &Self::Target {
        self.base()
    }
}

impl DerefMut for dyn Widget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.base_mut()
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetEvent {
    NewSpace(PositionedRect),
}
