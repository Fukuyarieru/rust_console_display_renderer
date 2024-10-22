use crate::display::Display;
use crate::display::Vec2;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::DataPoint;

pub enum Object<'a> {
    Shape {
        shape: Shape,
        center_point: (usize, usize),
        allocated_box: Vec2<&'a mut DataPoint>, // Change to mutable reference
        draw_val: char,
    },
    Menu {
        menu: Menu,
        center_point: (usize, usize),
        allocated_box: Vec2<&'a mut DataPoint>, // Change to mutable reference
    },
}
impl Object<'a> {
    pub fn shape(display: &Display, shape: Shape, center_point: (usize, usize)) -> Self {
        let mut vec = &display.screen;
        match shape {
            Shape::Circle { radius } => {}
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
