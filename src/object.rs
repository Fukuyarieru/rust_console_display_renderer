use std::collections::btree_set::SymmetricDifference;
use std::default;

use crate::display;
use crate::display::Display;
use crate::display::Vec2;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::DataPoint;

pub enum Object<'a> {
    Free {
        size: (usize, usize),
        center_point: (usize, usize),
        allocated_box: AllocateBox<'a>,
    },
    Shape {
        shape: Shape,
        center_point: (usize, usize),
        allocated_box: AllocateBox<'a>, // Change to mutable reference
        draw_val: char,
    },
    Menu {
        menu: Menu,
        center_point: (usize, usize),
        allocated_box: AllocateBox<'a>, // Change to mutable reference
    },
}
pub enum AllocateBox<'a> {
    AllocateInFunction,
    Allocated { allocated_box: Vec2<&'a DataPoint> },
}
impl<'a> Object<'a> {
    pub fn create_shape(
        shape: Shape,
        center_point: (usize, usize),
        draw_val: char,
        display: &'a Display,
    ) -> Self {
        match shape {
            Shape::Circle { radius } => {
                let left = center_point.0 - radius as usize;
                let right = center_point.0 + radius as usize;
                let top = center_point.1 + radius as usize;
                let bottom = center_point.1 - radius as usize;
                let reference_vec2 = display.allocate(left, right, top, bottom);
                return Object::Shape {
                    shape,
                    center_point,
                    allocated_box: AllocateBox::Allocated {
                        allocated_box: reference_vec2,
                    },
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
enum DisplayBoxAllocation<'a> {
    Empty,
    Allocated { allocated_box: Vec2<&'a DataPoint> },
}
