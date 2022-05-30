use super::Widget;
use std::fmt::{self, Debug};

//

#[derive(Default)]
pub struct SubWidgets {
    subwidgets: Vec<Box<dyn Widget>>,
}

//

impl Debug for SubWidgets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.iter().map(|widget| widget.name().to_owned()))
            .finish()
    }
}

impl SubWidgets {
    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.add_box(Box::new(widget));
    }

    pub fn add_box(&mut self, widget: Box<dyn Widget>) {
        self.subwidgets.push(widget);
    }

    pub fn iter(&self) -> impl Iterator<Item = &(dyn Widget + 'static)> + '_ {
        self.subwidgets.iter().map(Box::as_ref)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (dyn Widget + 'static)> + '_ {
        self.subwidgets.iter_mut().map(Box::as_mut)
    }
}
