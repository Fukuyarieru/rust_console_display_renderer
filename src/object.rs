use crate::display::DataPoint;
use crate::menu::Menu;
use crate::shape::Shape;

enum Object {
    Free,
    Shape {},
}
struct DisplayBox {}

// pub struct Object {
//     allocated_box: Vec2<&DataPoint>,
//     object_type: ObjectType,
// }
// pub enum ObjectType {
//     Free,
//     Shape { shape: Shape },
//     Menu { menu: Menu },
// }
// impl Object {
//     pub fn create(object_type: ObjectType, set_point: (usize, usize)) -> Self {}
// }
//======================================
// use std::collections::btree_set::SymmetricDifference;
// use std::default;

// use crate::display;
// use crate::display::Display;
// use crate::display::Vec2;
// use crate::menu::Menu;
// use crate::shape::Shape;
// use crate::DataPoint;
// use crate::DisplayConnector;

// pub enum Object<'a> {
//     Free {
//         center_point: (usize, usize),
//         allocated_box: Vec2<&'a DataPoint>,
//     },
//     Shape {
//         shape: Shape,
//         center_point: (usize, usize),
//         allocated_box: Vec2<&'a DataPoint>, // Change to mutable reference
//         draw_val: char,
//     },
//     Menu {
//         menu: Menu,
//         center_point: (usize, usize),
//         allocated_box: Vec2<&'a DataPoint>, // Change to mutable reference
//     },
// }
// impl<'a> Object<'a> {
//     pub fn create(object_type: Object, center_point: (usize, usize), connection: DisplayConnector) {
//     }
//     pub fn create_shape(
//         shape: Shape,
//         center_point: (usize, usize),
//         draw_val: char,
//         display: &Display,
//     ) -> Self {
//         let mut left;
//         let mut right;
//         let mut top;
//         let mut bottom;
//         match shape {
//             Shape::Circle { radius } => {
//                 left = (center_point.0 as f64 - radius) as usize;
//                 right = (center_point.0 as f64 + radius) as usize;
//                 top = (center_point.1 as f64 + radius) as usize;
//                 bottom = (center_point.1 as f64 - radius) as usize;
//                 let reference_vec2 = display.allocate(left, right, top, bottom);
//                 return Object::Shape {
//                     shape,
//                     center_point,
//                     allocated_box: reference_vec2,
//                     draw_val,
//                 };
//             }
//             Shape::Polygon { points_deltas } => todo!(),
//             Shape::Quadrilateral {
//                 point1_delta,
//                 point2_delta,
//                 point3_delta,
//                 point4_delta,
//             } => todo!(),
//         }
//         // display.
//     }
// }
// enum DisplayBoxAllocation<'a> {
//     Empty,
//     Allocated { allocated_box: Vec2<&'a DataPoint> },
// }
