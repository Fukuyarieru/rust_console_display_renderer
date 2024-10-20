use crate::menu::Menu;
use crate::shapes::Shape;

enum Object {
    Shape {
        shape: Shape,
        center_point: (usize, usize),
    },
    Menu {
        menu: Menu,
        center_point: (usize, usize), // aka preferred point, think of it for now as like open windows
    },
}
impl Adapter {}
