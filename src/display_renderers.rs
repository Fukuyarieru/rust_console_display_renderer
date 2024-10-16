#![allow(unused_variables)]
use crate::display::Display;
use crate::shapes::*;

// thing about this trait
trait Renderer {
    fn render_shape(shape: Shape) {}
}

// experimental
// also im not sure if these work, untested
pub fn render_quadrilateral(display: &mut Display, quadrilateral: Quadrilateral, thing: char) {
    display.draw_line(quadrilateral.point1, quadrilateral.point2, thing);
    display.draw_line(quadrilateral.point2, quadrilateral.point3, thing);
    display.draw_line(quadrilateral.point3, quadrilateral.point4, thing);
    display.draw_line(quadrilateral.point4, quadrilateral.point1, thing);
}
pub fn render_polygon(display: &mut Display, polygon: Polygon, thing: char) {
    for i in 0..polygon.points.len() - 1 {
        display.draw_line(polygon.points[i], polygon.points[i + 1], thing);
    }
    display.draw_line(
        polygon.points[polygon.points.len() - 1],
        polygon.points[0],
        thing,
    );
}
pub fn render_circle(display: Display, circle: Circle) {
    todo!()
    // display
}
