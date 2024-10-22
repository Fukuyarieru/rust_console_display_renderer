use crate::display::Display;
use crate::display::Vec2;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::DataPoint;

pub enum Object<'a> {
    Shape {
        shape: Shape,
        center_point: (usize, usize),
        allocated_box: Vec2<&'a DataPoint>,
        draw_val: char,
    },
    Menu {
        menu: Menu,
        center_point: (usize, usize), // aka preferred point, think of it for now as like open windows
        allocated_box: Vec2<&'a DataPoint>,
    },
}
impl Object<'a> {
    pub fn shape(display: &Display, shape: Shape, center_point: (usize, usize)) -> Self {
        let mut vec = &display.screen;
        match shape {
            Shape::Circle { radius } => {for i in (vec[])}
            Shape::Polygon { points_deltas } => todo!(),
            Shape::Quadrilateral {
                point1_delta,
                point2_delta,
                point3_delta,
                point4_delta,
            } => todo!(),
        }
        // display.
    }
}
