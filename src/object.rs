use std::default;
use std::intrinsics::discriminant_value;

use clearscreen::ClearScreen;

use crate::display;
use crate::display::Display;
use crate::display::Vec2;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::DataPoint;

pub enum Object<'a> {
    Shape {
        shape: Shape,
        center_point: (usize, usize),
        allocated_box: Vec2<&'a DataPoint>, // Change to mutable reference
        draw_val: char,
    },
    Menu {
        menu: Menu,
        center_point: (usize, usize),
        allocated_box: Vec2<&'a mut DataPoint>, // Change to mutable reference
    },
}
impl<'a> Object<'a> {
    pub fn shape(
        shape: Shape,
        center_point: (usize, usize),
        draw_val: char,
        display: &'a Display,
    ) -> Self {
        let mut left;
        let mut right;
        let mut top;
        let mut bottom;
        match shape {
            Shape::Circle { radius } => {
                left = (center_point.0 as f64 - radius) as usize;
                right = (center_point.0 as f64 + radius) as usize;
                top = (center_point.1 as f64 + radius) as usize;
                bottom = (center_point.1 as f64 - radius) as usize;
                if left < 0 {
                    left = 0
                }
                if right > display.screen.vec[0].len() {
                    right = display.screen.vec.len() - 1
                }
                if top > display.screen.vec.len() {
                    top = display.screen.vec.len() - 1
                }
                if bottom < 0 {
                    bottom = 0
                }

                let default_datapoint = &display.screen.vec[0][0];
                let mut reference_vec2: Vec2<&'a DataPoint> =
                    Vec2::create(right - left, top - bottom, &default_datapoint);

                for line in top..=bottom {
                    for row in left..=right {
                        reference_vec2.vec[top - line][right - row] =
                            &display.screen.vec[line][row];
                    }
                }
                return Object::Shape {
                    shape,
                    center_point,
                    allocated_box: reference_vec2,
                    draw_val,
                };
            }
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
