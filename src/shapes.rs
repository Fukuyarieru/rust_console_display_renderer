use core::fmt;

// trait DisplayAdapter {}
enum Shape {
    Quadrilateral,
    Circle,
    Polygon,
}
trait ShapeFunctions {
    fn get_area(self) -> f32;
    fn get_perimeter(self) -> f32;
    fn shift_up_by(self, amount: i32);
    fn shift_down_by(self, amount: i32);
    fn shift_left_by(self, amount: i32);
    fn shit_right_by(self, amount: i32);
    fn rotate_right_by(self, degrees: i32);
    fn rotate_left_by(self, degrees: i32);
}
pub struct Quadrilateral {
    pub points: [(usize, usize); 4],
}
impl Quadrilateral {
    pub fn create(
        point1: (usize, usize),
        point2: (usize, usize),
        point3: (usize, usize),
        point4: (usize, usize),
    ) -> Self {
        Quadrilateral {
            points: [point1, point2, point3, point4],
        }
    }
}
pub struct Circle {
    center: (usize, usize),
    radius: usize,
}
impl Circle {
    fn create(center: (usize, usize), radius: usize) -> Self {
        Circle { center, radius }
    }
}
pub struct Polygon {
    pub points: Vec<(usize, usize)>,
}

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
