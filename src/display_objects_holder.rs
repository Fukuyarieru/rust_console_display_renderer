use crate::display::Display;
use crate::object::Object;

pub struct DisplayObjectsHolder<'a>{
    ref_display: &'a Display<'a>,
    objects: Vec<Object<'a>>,
}
impl<'a> DisplayObjectsHolder<'a> {
    pub fn new(ref_display: &Display) -> Self {
        Self {
            ref_display,
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}