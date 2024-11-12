use crate::display::Display;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::standard::*;
use crate::DataPoint;

pub struct Object<'a> {
    pub center_point: Point,
    pub obj_type: ObjType,
    pub allocated_box: Option<Vec2<&'a mut DataPoint>>,
}
pub enum ObjType {
    Free { size: (usize, usize) },
    Shape { shape: Shape },
    Menu { menu: Menu },
}

impl<'a> Object<'a>{
    fn new(center_point: Point, obj_type: ObjType) -> Self {
        Self {
            center_point,
            obj_type,
            allocated_box: None,
        }
    }
    fn allocate_box(&mut self,allocated_box: Vec2<&'a mut DataPoint>) {
        self.allocated_box=Some(allocated_box);
    }
    fn get_center_point(&self) -> Point {
        self.center_point
    }
}
// pub enum AllocateBox<'a> {
//     AllocateInFunction,
//     Allocated {
//         allocated_box: Vec2<&'a mut DataPoint>,
//     },
// }
//======================= older stuff down here

// #[derive(Clone)]
// pub enum Object<'a> {
//     Free {
//         size: (usize, usize),
//         center_point: (usize, usize),
//         allocated_box: AllocateBox<'a>,
//     },
//     Shape {
//         shape: Shape,
//         center_point: (usize, usize),
//         allocated_box: AllocateBox<'a>, // Change to mutable reference
//         draw_val: char,
//     },
//     Menu {
//         menu: Menu,
//         center_point: (usize, usize),
//         allocated_box: AllocateBox<'a>, // Change to mutable reference
//     },
// }
// #[derive(Clone)]
// impl<'a> Object<'a> {
//     pub fn allocate(&mut self, given_allocated_box: Vec2<&DataPoint>) {
//         match self {
//             Object::Free {
//                 size,
//                 center_point,
//                 mut allocated_box,
//             } => {
//                 allocated_box = AllocateBox::Allocated {
//                     allocated_box: given_allocated_box,
//                 }
//             }
//             Object::Shape {
//                 shape,
//                 center_point,
//                 allocated_box,
//                 draw_val,
//             } => todo!(),
//             Object::Menu {
//                 menu,
//                 center_point,
//                 allocated_box,
//             } => todo!(),
//         }
//     }
//     pub fn create_shape(
//         shape: Shape,
//         center_point: (usize, usize),
//         draw_val: char,
//         display: &'a Display,
//     ) -> Self {
//         match shape {
//             Shape::Circle { radius } => {
//                 let left = center_point.0 - radius as usize;
//                 let right = center_point.0 + radius as usize;
//                 let top = center_point.1 + radius as usize;
//                 let bottom = center_point.1 - radius as usize;
//                 let reference_vec2 = display.allocate(left, right, top, bottom);
//                 return Object::Shape {
//                     shape,
//                     center_point,
//                     allocated_box: AllocateBox::Allocated {
//                         allocated_box: reference_vec2,
//                     },
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