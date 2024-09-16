use crate::shapes::*;
use crate::Display;

pub fn render_quadrilateral(display: &mut Display, shape: Quadrilateral) {
    display.draw_line(shape.points.iter());
}
pub fn render_circle(display: Display, circle: Circle) {
    display
}
