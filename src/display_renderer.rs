use crate::display::Display;
use crate::display_objects_holder::DisplayObjectsHolder;
use crate::menu::Menu;

pub struct DisplayRenderer<'a>{
    display: Display<'a>,
    objects_holder: DisplayObjectsHolder<'a>,
}

impl<'a> DisplayRenderer<'a> {
    pub fn new(display:Display) -> Self {
        let objects_holder=DisplayObjectsHolder::new(&display);
        Self {
            display,
            objects_holder,
        }
    }
    pub fn link_objects_holder(&mut self,DisplayObjectsHolder)
}
