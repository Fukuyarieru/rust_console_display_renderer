use crate::display::Display;
use crate::display_objects_holder::DisplayObjectsHolder;
use crate::menu::Menu;

pub struct ConsoleDisplayRenderer<'a>{
    display: Display,
    objects_holder: DisplayObjectsHolder<'a>,
}

impl<'a> ConsoleDisplayRenderer<'a> {
    pub fn new(display:Display) -> Self {
        let objects_holder=DisplayObjectsHolder::new(&display);
        Self {
            display,
            objects_holder,
        }
    }
    pub fn set_objects_holder(&mut self, link:DisplayObjectsHolder<'a>) {
        self.objects_holder=link;
    }
}
