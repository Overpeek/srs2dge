use std::ops::{Deref, DerefMut};

use super::{Widget, WidgetBase};

//

#[derive(Debug, Default)]
pub struct RootWidget {
    base: WidgetBase,
}

//

impl RootWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for RootWidget {
    fn name(&self) -> &str {
        "RootWidget"
    }

    fn base(&self) -> &WidgetBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut WidgetBase {
        &mut self.base
    }
}

impl Deref for RootWidget {
    type Target = WidgetBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for RootWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
