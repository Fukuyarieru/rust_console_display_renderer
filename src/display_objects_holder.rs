use crate::display::Display;
use crate::object::Object;

pub struct DisplayObjectsHolder<'a>{
    ref_display: &'a Display,   // consider removing ref_display, and let display_renderer handle everything regarding that |||| OR, it was supposed to have a ref from the beginning?
    objects: Vec<Object<'a>>,
}
impl<'a> DisplayObjectsHolder<'a> {
    pub fn new(ref_display: &'a Display) -> Self {
        Self {
            ref_display,
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, object: Object<'a>) {
        self.objects.push(object);
    }
}