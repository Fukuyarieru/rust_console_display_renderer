use crate::display::Display;
use crate::menu::Menu;
use crate::shape::Shape;
use crate::standard::*;
use crate::DataPoint;
use crate::standard::Ptr;

pub const ERROR_OBJECT_EMPTY_BOX:&str="Object does not have an initialized allocated box";
pub struct Object{
    pub center_point: Point,
    pub obj_type: ObjType,
    pub allocated_box: Option<Vec2<Ptr<DataPoint>>>,
}
pub enum ObjType {
    Free { size: (usize, usize) },
    Shape { shape: Shape },
    Menu { menu: Menu },
}
impl std::fmt::Display for ObjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            ObjType::Free{size: (_,_) } => write!(f, "Free"),
            ObjType::Shape{shape} => write!(f, "Shape"),
            ObjType::Menu{menu} => write!(f, "Menu") ,
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
    pub fn get_center_point(&self) -> Point {
        self.center_point
    }
    #[allow(unused_variables)]
    pub fn allocate_from_display(&mut self, display: &Display) {
        todo!()
    }
    pub fn fill_box(&mut self, new_ch: char) {
        println!("Filling allocated box inside object to be {new_ch}");
        if let Some(allocated_box) = &mut self.allocated_box {
            for inner_vec in &allocated_box.vec {
                for datapoint in inner_vec {
                    datapoint.get_ref().update(new_ch);
                    // unsafe {
                    //     // datapoint.as_mut().unwrap().update(new_ch);
                    //     let datapoint_ref=&**datapoint;
                    //     datapoint_ref.update(new_ch);
                    //     // (**datapoint).update(new_ch);
                    //
                    //     // im not sure what's wrong here, and even if its here,
                    //     // although I got a gut feeling It's in here
                    // }
                }
            }
        } else {
            panic!("{}",ERROR_OBJECT_EMPTY_BOX);
        }
    }
}
// pub struct AllocationBox {
//     pub right: usize,
//     pub left: usize,
//     pub top: usize,
//     pub bottom: usize,
//     pub allocated_box: Option<Vec2<*mut DataPoint>>,
// }
impl std::fmt::Display for Object{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        if let Some(allocated_box) = &self.allocated_box {
            write!(f,"{}",allocated_box)
        }
        else {
            panic!("{}",ERROR_OBJECT_EMPTY_BOX);
        }


        // Delegate to the Display implementation for Vec2<Point>
        // write!(f, "{}", self.allocated_box)
        //     if let Some(allocated_box) = &self.allocated_box {
        //         write!(f,"{}",allocated_box)
        //     } else {
        //         // what's the point of this?
        //         let obj_width=self.allocated_box.unwrap_or()
        //         write!(f,Vec2::new(self.allocated_box.))
        //     }
        // }
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
