// use crate::adapters::DisplayAdapter;
use crate::animations::Animation;
use crate::functions::calc_distance;
use crate::object;
use crate::object::Object;
use crate::object::Type;
use crate::standard::*;
use rand::*;

pub const DEFAULT_DATAPOINT_HISTORY_SIZE: usize = 3;

#[derive(Clone)]
pub struct DataPoint {
    pub val: char,
    pub vals_history: Vec<char>,
}

impl DataPoint {
    pub fn create(ch: char, history_size: usize) -> Self {
        DataPoint {
            val: ch,
            vals_history: vec![' '; history_size],
        }
    }
    pub fn update(&mut self, new_ch: char) {
        // TODO can do this better (right now it does 3 scans from what i understand)
        self.vals_history.push(self.val);
        self.val = new_ch;
        self.vals_history.pop();
    }
    pub fn reverse(&mut self) {
        self.val = self.vals_history[0];
        self.vals_history.remove(0);
        self.vals_history.insert(self.vals_history.len(), ' ');
    }
}
impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub struct Display<'a> {
    pub screen: Vec2<DataPoint>,
    pub width: usize,
    pub height: usize,
    pub boxer: Vec<&'a Object<'a>>,
    // more stuff here later, probably (panels, info, titlebar)
}

impl<'a> Display<'a> {
    // Lets limit for now the use of individual pixels inside of the Display struct
    pub fn create(width: usize, height: usize) -> Self {
        Display {
            screen: Vec2::create(
                width,
                height,
                DataPoint::create('#', DEFAULT_DATAPOINT_HISTORY_SIZE),
            ),
            width,
            height,
            boxer: Vec::new(),
        }
    }
    // pixel function should return a Result type to tell if index was out of bounds
    fn pixel(&mut self, point: Point, new_val: char) {
        (self.screen.vec[point.x][point.y]).update(new_val);
    }

    pub fn draw_line(&mut self, point1: Point, point2: Point, draw_val: char) {
        // redeclaration
        let x1 = point1.x;
        let x2 = point2.x;
        let y1 = point1.y;
        let y2 = point2.y;

        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.pixel(
                Point {
                    x: x as usize,
                    y: y as usize,
                },
                draw_val,
            ); // Set pixel

            if x == x2 as isize && y == y2 as isize {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    fn get_center(&self) -> Point {
        Point {
            x: self.width / 2,
            y: self.height / 2,
        }
    }
    fn total_area(&self) -> usize {
        self.width * self.height
    }
    fn randomize_screen(&mut self, draw_val: char, screen_percentage: f32) {
        let screen_area = self.total_area() as f64;
        let mut area_to_change: f64 = screen_area * (screen_percentage as f64) / 100.0;
        while area_to_change > 0.0 {
            let ((x1, y1), (x2, y2)) = self.random_line(draw_val);
            let line_length: f32 = calc_distance(x1, y1, x2, y2); // Assuming calc_distance takes coordinates and returns distance
            area_to_change -= line_length as f64;
        }
    }
    fn random_line(&mut self, draw_val: char) -> ((usize, usize), (usize, usize)) {
        // function returns the random line that was made
        let mut rng = rand::thread_rng();
        let rx1 = rng.gen_range(0..self.screen.max_x);
        let ry1 = rng.gen_range(0..self.screen.max_y);
        let rx2 = rng.gen_range(0..self.screen.max_x);
        let ry2 = rng.gen_range(0..self.screen.max_y);
        self.draw_line(Point { x: rx1, y: ry1 }, Point { x: rx2, y: ry2 }, draw_val);
        ((rx1, ry1), (rx2, ry2))
    }
    // REDO OF ADD
    pub fn add(&'a self, mut object: Object<'a>) -> &'a Object {
        match object.allocated_box {
            None => object.allocated_box = Some(self.allocate(4, 20, 20, 4)),
            Some(_) => (),
        };
        &object
    }

    // TODO
    // #[allow(unused_variables)]
    // pub fn add(&'a self, &mut object: &Object<'a>) -> &Object {
    //     match object.obj_type {
    //         Type::Free { size } => (),
    //         Type::Shape { ref shape } => (),
    //         Type::Menu { ref menu } => (),
    //     }
    //     object.allocated_box = Some(self.allocate(4, 20, 20, 4));
    //     &object
    // }
    // OLD
    // #[allow(unused_variables)]
    // pub fn add(&'a mut self, mut object: Object<'a>) {
    //     let left=
    //     object.allocate(self.allocate(left, right, top, bottom));
    //     // let object = &object;
    //     // match object {
    //     //     Object::Free {
    //     //         size,
    //     //         center_point,
    //     //         mut allocated_box,
    //     //     } => {
    //     //         allocated_box = AllocateBox::Allocated {
    //     //             allocated_box: self.allocate(
    //     //                 center_point.0 - size.0,
    //     //                 center_point.0 + size.1,
    //     //                 center_point.1 + size.1,
    //     //                 center_point.1 - size.1,
    //     //             ),
    //     //         }
    //     //     }
    //     //     Object::Shape {
    //     //         shape,
    //     //         center_point,
    //     //         allocated_box,
    //     //         draw_val,
    //     //     } => todo!(),
    //     //     Object::Menu {
    //     //         menu,
    //     //         center_point,
    //     //         allocated_box,
    //     //     } => todo!(),
    //     // }
    //     self.boxer.push(object);
    // }
    // TODO
    pub fn allocate(
        &self,
        mut left: usize,
        mut right: usize,
        mut top: usize,
        mut bottom: usize,
    ) -> Vec2<&DataPoint> {
        if left < 0 {
            left = 0
        }
        if right >= self.screen.vec.len() {
            right = self.screen.vec.len() - 1
        }
        if top >= self.screen.vec[0].len() {
            top = self.screen.vec[0].len() - 1
        }
        if bottom < 0 {
            bottom = 0
        }

        let default_datapoint = &self.screen.vec[0][0];
        let mut reference_vec2: Vec2<&DataPoint> =
            Vec2::create(right - left, top - bottom, default_datapoint);

        for (line_idx, line) in (top..=bottom).enumerate() {
            for (row_idx, row) in (left..=right).enumerate() {
                reference_vec2.vec[line_idx][row_idx] = &self.screen.vec[line][row];
            }
        }
        reference_vec2
    }
}
// Implement Display for the Display struct
impl<'a> std::fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display implementation for Vec2<Point>
        write!(f, "{}", self.screen)
    }
}
