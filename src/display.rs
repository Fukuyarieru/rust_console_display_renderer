use std::cell::Cell;
// use crate::adapters::DisplayAdapter;
// use crate::animations::Animation;
use crate::functions::calc_distance;
use crate::object;
use crate::object::Object;
use crate::object::ObjType;
use crate::standard::*;
use rand::*;

pub const DEFAULT_DATAPOINT_HISTORY_SIZE: usize = 3;

pub struct DataPoint {
    // CELLLLLLL
    pub val: Cell<char>,
    pub vals_history: Cell<Vec<char>>,
}

impl DataPoint {
    // NOTE TO THIS ENTIRE STRUCT: i should remake this later to be more readable
    pub fn new(ch: char, history_size: usize) -> Self {
        DataPoint {
            val: Cell::new(ch),
            vals_history: Cell::new(vec![' '; history_size]),
        }
    }
    pub fn update(&self, new_ch: char) {
        let mut history = self.vals_history.take(); // Take ownership of the Vec
        history.push(self.val.get());
        self.val.set(new_ch);
        history.pop();
        self.vals_history.set(history); // Set the modified Vec back
        // my "old" solution, problem was that i had to make 'self' respect borrowchecker rules, meaning, "history variable that wasnt actually self" or something
        // self.vals_history.get_mut().push(self.val.get());
        // self.val.set(new_ch);
        // // Datapoint history size is secured to be the same thanks to the vec! macro we used
        // self.vals_history.get_mut().pop();
    }
    pub fn reverse(&self) {
        // again old code
        // let history_size=self.vals_history.get_mut().len();
        // self.val.set(*self.vals_history.get_mut().first().unwrap());
        // self.vals_history.get_mut().remove(0);
        // self.vals_history.get_mut().insert(history_size-1,' ');
        let mut history = self.vals_history.take(); // Take ownership of the Vec
        if !history.is_empty() {
            self.val.set(history[0]);
            history.remove(0);
            history.push(' ');
            self.vals_history.set(history); // Set the modified Vec back
        }
    }
}
impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val.get())
    }
}

pub struct Display<'a>{
    pub screen: Vec2<DataPoint>,
    pub width: usize,
    pub height: usize,
    pub boxer: Vec<Object<'a>>,
}

impl<'a> Display<'a>{
    // Let's limit for now the use of individual pixels inside of the Display struct
    pub fn new(width: usize, height: usize) -> Self {
        Display {
            screen: Vec2::new(
                width, height,
                // DataPoint::create('#', DEFAULT_DATAPOINT_HISTORY_SIZE),
            ),
            width,
            height,
            boxer: Vec::new(),
        }
    }
    // in the case that the index point is outside the screen, no action would happen
    pub fn pixel(&mut self, point: Point, new_val: char) {
        // (self.screen.vec[point.x][point.y]).update(new_val);
        self.screen
            .index(point.x, point.y)
            .unwrap_or_else(|err| err)
            .update(new_val);
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
    pub fn get_center(&self) -> Point {
        Point {
            x: self.width / 2,
            y: self.height / 2,
        }
    }
    pub fn total_area(&self) -> usize {
        self.width * self.height
    }
    // TODO, need to implement newly using Point s
    // fn randomize_screen(&mut self, draw_val: char, screen_percentage: f32) {
    //     let screen_area = self.total_area() as f64;
    //     let mut area_to_change: f64 = screen_area * (screen_percentage as f64) / 100.0;
    //     while area_to_change > 0.0 {
    //         let ((x1, y1), (x2, y2)) = self.random_line(draw_val);
    //         let line_length: f32 = calc_distance(x1, y1, x2, y2); // Assuming calc_distance takes coordinates and returns distance
    //         area_to_change -= line_length as f64;
    //    
    // }
    pub fn random_line(&mut self, draw_val: char) -> (Point, Point) {
        // function returns the random line that was made
        let mut rng = rand::thread_rng();
        let rx1 = rng.gen_range(0..self.screen.max_x);
        let ry1 = rng.gen_range(0..self.screen.max_y);
        let rx2 = rng.gen_range(0..self.screen.max_x);
        let ry2 = rng.gen_range(0..self.screen.max_y);
        self.draw_line(Point { x: rx1, y: ry1 }, Point { x: rx2, y: ry2 }, draw_val);
        (Point { x: rx1, y: ry1 }, Point { x: rx2, y: ry2 })
    }
    pub fn allocate(
        &self,
        left: usize,
        mut right: usize,
        mut top: usize,
        bottom: usize,
    ) -> Vec2<&DataPoint> {
        // no need to check for bottom and left as we use usizes and they cant be negative

        if right >= self.width {
            right = self.screen.vec.len() - 1
        }
        if top >= self.height {
            top = self.screen.vec[0].len() - 1
        }

        // let default_datapoint = &mut self.screen.vec[0][0];
        let mut reference_vec2: Vec2<&DataPoint> = Vec2::new(right - left, top - bottom);
        // {
        //     vec: Vec::new(),
        //     max_x: right - left,
        //     max_y: top - bottom,
        // };
        // Vec2::create(right - left, top - bottom, default_datapoint);

        // for line in top..=bottom {
        //     let mut row_refs: Vec<&mut DataPoint> = Vec::with_capacity(right - left + 1);
        //     for row in left..=right {
        //         // Get a mutable reference to each DataPoint in this row
        //         let data_point = &mut self.screen.vec[line][row] as *mut _; // Cast to a raw pointer
        //         row_refs.push(unsafe { &mut *data_point }); // Safely cast back to &mut
        //     }
        //     reference_vec2.vec.push(row_refs);
        // }
        for line in top..=bottom {
            for row in left..=right {
                reference_vec2.vec[line][row] = &self.screen.vec[line][row];
            }
        }
        // for (line_idx, line) in (top..=bottom).enumerate() {
        //     for (row_idx, row) in (left..=right).enumerate() {
        //         reference_vec2.vec[line_idx][row_idx] = &mut self.screen.vec[line][row];
        //     }
        // }
        reference_vec2
    }
    pub fn add_object(&mut self,object: Object<'a>) {
        self.boxer.push(object);
    }
    pub fn initialize_object<'b>(&'a mut self, obj_ref: &mut Object<'b>) where 'a:'b {
        obj_ref.allocated_box=Some(self.allocate(2,10,2,10));
    }
}
// Implement Display for the Display struct
impl<'a> std::fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display implementation for Vec2<Point>
        write!(f, "{}", self.screen)
    }
}
