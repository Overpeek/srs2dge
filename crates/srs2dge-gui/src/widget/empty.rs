use super::{base::WidgetBase, Widget};
use srs2dge_core::{
    batch::{BatchRenderer, Idx},
    color::Color,
    glam::Vec2,
};

//

#[derive(Debug, Default)]
pub struct Empty {
    base: WidgetBase,

    color: Color,

    idx: Option<Idx>,
}

//

impl Empty {
    pub fn new() -> Self {
        Self::default()
    }
}

//

impl Widget for Empty {
    fn name(&self) -> &str {
        "Empty"
    }

    fn base(&self) -> &WidgetBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut WidgetBase {
        &mut self.base
    }

    fn draw(&mut self, batcher: &mut BatchRenderer) {
        let idx = if let Some(idx) = self.idx {
            idx
        } else {
            let idx = batcher.push();
            self.idx = Some(idx);
            idx
        };

        if self.base().update() {
            log::debug!("redraw");
            let quad = batcher.get_mut(idx).unwrap();
            quad.col = self.color;
            quad.size = Vec2::new(self.base.space.x as f32, self.base.space.y as f32);
            quad.pos = Vec2::new(self.base.space.width as f32, self.base.space.height as f32);
        }
    }
}
