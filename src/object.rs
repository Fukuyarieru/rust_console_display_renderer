use crate::display::Display;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::standard::*;
use crate::DataPoint;
use crate::standard::Ptr;

pub const ERROR_OBJECT_EMPTY_BOX:&str="Object does not have an initialized allocated box";
#[derive(Debug)]
pub struct Object {
    pub center_point: Point,
    pub obj_type: ObjType,
    allocated_box: Option<Vec2<Ptr<DataPoint>>>,
}
#[derive(Debug)]
pub enum ObjType {
    Free { size: (usize, usize) },
    Shape { shape: Shape },
    Menu { menu: Menu },
}
impl std::fmt::Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            ObjType::Free{size: (_width,_height) } => write!(f, "Free"),
            ObjType::Shape{ shape: _shape } => write!(f, "Shape"),
            ObjType::Menu{ menu: _menu } => write!(f, "Menu") ,
        }
    }
}
impl Object{
    pub fn new(center_point: Point, obj_type: ObjType) -> Self {
        println!("Creating new object of type {obj_type} and center of {center_point}");
        Self {
            center_point,
            obj_type,
            allocated_box: None,
        }
    }
    // pub fn allocate_box(&mut self,allocated_box: Vec2<&'a DataPoint>) {
    //     self.allocated_box=Some(allocated_box);
    // }
    pub fn allocate_from_display(&mut self, display: &Display) {
        println!("Allocating to object from display");
        display.initialize_object(self);
    }
    pub fn is_not_initialized(&self) -> bool {
        self.allocated_box.is_none()
    }
    pub fn is_initialized(&self) -> bool {
        self.allocated_box.is_some()
    }
    pub fn fill_box(&mut self, new_ch: char) {
        println!("Filling allocated box inside object to be {new_ch}");
        self.get_allocated_box().vec.iter().for_each(|inner_vec| {
            inner_vec.iter().for_each(|data_point| {
                data_point.get_ref().update(new_ch);
            });
        });
    }
    pub fn get_allocated_box(&self) -> &Vec2<Ptr<DataPoint>> {
        if let Some(allocated_box) = &self.allocated_box {
            allocated_box
        }
        else {
            panic!("{}", ERROR_OBJECT_EMPTY_BOX);
        }
    }
    pub fn set_allocated_box(&mut self, new_box: Vec2<Ptr<DataPoint>>) {
        self.allocated_box = Some(new_box);
    }
}
impl std::fmt::Display for Object{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.get_allocated_box())
    }
}
// impl std::fmt::Display for Vec2<*mut DataPoint> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for inner_vec in &self.vec {
//             for datapoint in inner_vec {
//                 unsafe {
//                     let datapoint_ref = &**datapoint;
//                     write!(f,"{}",datapoint_ref.val.get())?;
//                 }
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }
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
