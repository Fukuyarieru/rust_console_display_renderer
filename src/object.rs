use std::path::Display;

use crate::display::{self, Vec2};
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
        todo!()
        // display.
    }
}
