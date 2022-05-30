use super::{sub::SubWidgets, Widget, WidgetEvent};
use srs2dge_core::{batch::BatchRenderer, prelude::PositionedRect};
use std::sync::atomic::{AtomicBool, Ordering};

//

#[derive(Debug, Default)]
pub struct WidgetBase {
    pub subwidgets: SubWidgets,
    pub space: PositionedRect,

    pub updated: AtomicBool,
}

pub trait AsWidgetBase {
    fn react(&mut self, event: WidgetEvent, batcher: &mut BatchRenderer);
    fn draw(&mut self, batcher: &mut BatchRenderer);
    fn update(&self) -> bool;
}

//

impl WidgetBase {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn update(&self) -> bool {
        self.updated.swap(false, Ordering::SeqCst)
    }

    #[inline]
    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.add_box(Box::new(widget));
    }

    #[inline]
    pub fn add_box(&mut self, widget: Box<dyn Widget>) {
        self.subwidgets.add_box(widget);
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &(dyn Widget + 'static)> + '_ {
        self.subwidgets.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (dyn Widget + 'static)> + '_ {
        self.subwidgets.iter_mut()
    }

    pub(crate) fn react_recurse(&mut self, event: WidgetEvent, batcher: &mut BatchRenderer) {
        match event {
            WidgetEvent::NewSpace(space) => {
                self.space = space;
                self.updated.store(true, Ordering::SeqCst);
            }
        };

        for subwidget in self.subwidgets.iter_mut() {
            subwidget.start_react(event, batcher);
        }
    }

    pub(crate) fn draw_recurse(&mut self, batcher: &mut BatchRenderer) {
        for subwidget in self.subwidgets.iter_mut() {
            subwidget.start_draw(batcher);
        }
    }
}

impl<W: Widget> AsWidgetBase for W {
    fn react(&mut self, event: WidgetEvent, batcher: &mut BatchRenderer) {
        self.base_mut().react_recurse(event, batcher);
    }

    fn draw(&mut self, batcher: &mut BatchRenderer) {
        self.base_mut().draw_recurse(batcher);
    }

    fn update(&self) -> bool {
        self.base().update()
    }
}
