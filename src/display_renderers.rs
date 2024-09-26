#![allow(unused_variables)]

use std::ops::Index;

use crate::shapes::*;
use crate::shapes::*;
use crate::Display;

trait Renderers {
    fn render(shape: Shape) {}
}
pub fn render_quadrilateral(display: &mut Display, shape: Quadrilateral, thing: char) {
    for i in 0..3 {
        display.draw_line(
            shape.points.index(i).0,
            shape.points.index(i).1,
            shape.points.index(i + 1).0,
            shape.points.index(i + 1).1,
            thing,
        );
        // println!("RENDERING");
    }
}
pub fn render_polygon(display: &mut Display, shape: Polygon, thing: char) {
    for i in 0..shape.points.len() {
        display.draw_line(
            shape.points.index(i).0,
            shape.points.index(i).1,
            shape.points.index(i + 1).0,
            shape.points.index(i + 1).1,
            thing,
        );
    }
}
pub fn render_circle(display: Display, circle: Circle) {
    todo!()
    // display
}
