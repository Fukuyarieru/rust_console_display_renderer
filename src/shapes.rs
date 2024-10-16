#![allow(unused_variables)]
use crate::adapters::ShapeAdapter;
use crate::Display;

trait DisplayAdapter {
    fn place(display: &Display);
}
trait ShapesFunctions {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
    fn shift_up_by(&mut self, amount: usize);
    fn shift_down_by(&mut self, amount: usize);
    fn shift_left_by(&mut self, amount: usize);
    fn shit_right_by(&mut self, amount: usize);
    fn rotate_right_by(&mut self, degrees: usize);
    fn rotate_left_by(&mut self, degrees: usize);
}
pub enum Shape {
    Quadrilateral(Quadrilateral),
    Circle(Circle),
    Polygon(Polygon),
}
pub struct Quadrilateral {
    pub point1: (usize, usize),
    pub point2: (usize, usize),
    pub point3: (usize, usize),
    pub point4: (usize, usize),
}

impl Quadrilateral {
    pub fn create(
        point1: (usize, usize),
        point2: (usize, usize),
        point3: (usize, usize),
        point4: (usize, usize),
    ) -> Self {
        Quadrilateral {
            point1,
            point2,
            point3,
            point4,
        }
    }
}
impl ShapesFunctions for Quadrilateral {
    fn get_area(&self) -> f64 {
        todo!()
    }
    fn get_perimeter(&self) -> f64 {
        todo!()
    }
    fn rotate_left_by(&mut self, degrees: usize) {
        todo!()
    }
    fn rotate_right_by(&mut self, degrees: usize) {
        todo!()
    }
    fn shift_down_by(&mut self, amount: usize) {
        todo!()
    }
    fn shift_left_by(&mut self, amount: usize) {
        todo!()
    }
    fn shift_up_by(&mut self, amount: usize) {
        todo!()
    }
    fn shit_right_by(&mut self, amount: usize) {
        todo!()
    }
}
pub struct Circle {
    center_point: (usize, usize),
    radius: usize,
}
impl Circle {
    fn create(center_point: (usize, usize), radius: usize) -> Self {
        Circle {
            center_point,
            radius,
        }
    }
}
impl ShapesFunctions for Circle {
    fn get_area(&self) -> f64 {
        todo!()
    }
    fn get_perimeter(&self) -> f64 {
        todo!()
    }
    fn rotate_left_by(&mut self, degrees: usize) {
        todo!()
    }
    fn rotate_right_by(&mut self, degrees: usize) {
        todo!()
    }
    fn shift_down_by(&mut self, amount: usize) {
        todo!()
    }
    fn shift_left_by(&mut self, amount: usize) {
        todo!()
    }
    fn shift_up_by(&mut self, amount: usize) {
        todo!()
    }
    fn shit_right_by(&mut self, amount: usize) {
        todo!()
    }
}
pub struct Polygon {
    pub points: Vec<(usize, usize)>,
}
impl Polygon {
    fn create(points: Vec<(usize, usize)>) -> Self {
        Polygon { points }
    }
}
impl ShapesFunctions for Polygon {
    fn get_area(&self) -> f64 {
        todo!()
    }
    fn get_perimeter(&self) -> f64 {
        todo!()
    }
    fn rotate_left_by(&mut self, degrees: usize) {
        todo!()
    }
    fn rotate_right_by(&mut self, degrees: usize) {
        todo!()
    }
    fn shift_down_by(&mut self, amount: usize) {
        todo!()
    }
    fn shift_left_by(&mut self, amount: usize) {
        todo!()
    }
    fn shift_up_by(&mut self, amount: usize) {
        todo!()
    }
    fn shit_right_by(&mut self, amount: usize) {
        todo!()
    }
}
// TODO

// pub struct Polygon {
//     start: (usize, usize),
//     points: Vec<(usize, usize)>,
// }
// impl Polygon {
//     fn get_degree_sum(self) -> usize {
//         180 * (self.points.len() - 2)
//     }
//     #[allow(unused_variables)]
//     fn new_from_points(points: Vec<(usize, usize)>) -> Polygon {
//         todo!()
//     }
//     #[allow(unused_variables)]
//     fn new_from_linked_coordinates(linked_coordinates: Vec<usize>) -> Polygon {
//         todo!()
//     }
//     #[allow(unused_variables)]
//     fn new_from_line_lengths(lines: Vec<usize>) -> Polygon {
//         todo!()
//     } // could fail or not work well??????? probably.
// }
// impl Shape for Polygon {
//     fn get_center_weight(&self) -> (f64, f64) {
//         let cx = self
//             .points
//             .iter()
//             .map(|(first, _second)| *first as f64)
//             .sum::<f64>()
//             / self.points.len() as f64;
//         let cy = self
//             .points
//             .iter()
//             .map(|(_first, second)| *second as f64)
//             .sum::<f64>()
//             / self.points.len() as f64;
//         (cx, cy)
//     }
//     fn get_area(&self) -> f64 {
//         todo!()
//     }
//     fn get_perimeter(&self) -> f64 {
//         todo!()
//     }
// }
// pub struct Circle {
//     pub center: (usize, usize),
//     pub radius: usize,
// }
// impl Shape for Circle {
//     fn get_area(&self) -> f64 {
//         todo!()
//     }
//     fn get_center_weight(&self) -> (f64, f64) {
//         todo!()
//     }
//     fn get_perimeter(&self) -> f64 {
//         todo!()
//     }
// }
// pub trait Trigonometry {
//     fn rotate(&mut self, degrees: usize);
// }
// pub trait Shape {
//     fn get_area(&self) -> f64;
//     fn get_perimeter(&self) -> f64;
//     fn get_center_weight(&self) -> (f64, f64);
//     // fn rotate(&mut self);
// }
