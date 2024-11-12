use crate::display::Display;
use crate::display_objects_holder::DisplayObjectsHolder;

pub struct DisplayRenderer<'a>{
    display: Display<'a>,
    objects_holder: DisplayObjectsHolder<'a>,
}

impl<'a> DisplayRenderer<'a> {
    pub fn new(display:Display) -> Self {
        Self {
            display,
            objects_holder: DisplayObjectsHolder::new(&display),
        }
    }
}
