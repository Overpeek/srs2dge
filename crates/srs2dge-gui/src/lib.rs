/* TODO: use prelude::{widget::root::RootWidget, WidgetEvent};
use srs2dge_core::{
    buffer::{IndexBuffer, VertexBuffer},
    main_game_loop::event::Event,
    prelude::PositionedRect,
    target::Target,
    winit::event::WindowEvent,
    Frame,
};
use std::ops::{Deref, DerefMut};

//

pub mod prelude;
pub mod style;
pub mod widget;

//

pub struct Gui {
    root: RootWidget,

    batcher: BatchRenderer,
}

//

impl Gui {
    pub fn new(target: &Target) -> Self {
        Self {
            root: RootWidget::default(),

            batcher: BatchRenderer::new(target),
        }
    }

    pub fn event(&mut self, event: &Event) {
        if let Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } = event
        {
            self.root.react_recurse(
                WidgetEvent::NewSpace(PositionedRect::new(0, 0, size.width, size.height)),
                &mut self.batcher,
            )
        }
    }

    pub fn generate(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
    ) -> (&'_ VertexBuffer, &'_ IndexBuffer, u32) {
        self.root.draw_recurse(&mut self.batcher);
        self.batcher.generate(target, frame)
    }
}

impl Deref for Gui {
    type Target = RootWidget;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl DerefMut for Gui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
    }
}
 */
