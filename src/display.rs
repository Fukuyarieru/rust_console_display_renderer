use std::cell::{Cell,RefCell};
use crate::functions::calc_distance;
use crate::object;
use crate::object::Object;
use crate::object::ObjType;
use crate::standard::*;
use rand::Rng;

pub const DEFAULT_DATAPOINT_HISTORY_SIZE: usize = 3;

#[derive(Debug)]
#[derive(Default)]
pub struct DataPoint {
    pub val: Cell<char>,
    pub vals_history: RefCell<Vec<char>>,
}

impl DataPoint {
    pub fn new(ch: char, history_size: usize) -> Self {
        if history_size == 0 {
            panic!("History size must be greater than 0!");
        }
        println!("Creating new datapoint with {ch}");
        DataPoint {
            val: Cell::new(ch),
            vals_history: RefCell::new(Vec::with_capacity(history_size)),
        }
    }

    pub fn update(&self, new_ch: char) {

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let current_val=self.val.get();
        let mut history =self.vals_history.borrow_mut();
        println!("Updating DataPoint: Current: {}, New: {}", current_val, new_ch);
        println!("History before update: {:?}", history);
        println!("History Capacity: {}, History Length: {}", history.capacity(), history.len());
        
        // TODO, do this better
        if history.capacity() == 0 {
            history.push(' ');
        }
        
        
        // push() adds to the end of an array
        // pop() removes the end of the array (and returns it)
        // remove(index) removes the index and returns it
        
        if history.capacity() == history.len() {
            history.remove(0);
        }

        history.push(current_val);
        self.val.set(new_ch);
        println!("Datapoint updated to have {}",self.val.get());
        println!("History after update: {:?}", history);
    }

    pub fn reverse(&self) {
        println!("Reversing DataPoint: Current: {}", self.val.get());
        let mut history = self.vals_history.borrow_mut();
        println!("History before reverse: {:?}", history);
        if !history.is_empty() {
            let oldest = history.remove(0);
            self.val.set(oldest);
            history.push(' ');
        }
        println!("History after reverse: {:?}", history);
    }
}

impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val.get())
    }
}

pub struct Display{
    pub screen: Vec2<DataPoint>,
    pub width: usize,
    pub height: usize,
    pub boxer: Vec<Object>,
}

impl Display{
    pub fn new(width: usize, height: usize) -> Self {
        println!("Creating new display with the height of {height} and the width of {width}");
        Display {
            screen: Vec2::<DataPoint>::new(
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
        println!("Placing a pixel on {point} on display");
        // (self.screen.vec[point.x][point.y]).update(new_val);
        self.screen.index_mut_ref(point.x, point.y).update(new_val);
    }

    pub fn draw_line(&mut self, point1: Point, point2: Point, draw_val: char) {
        println!("Drawing a line from {point1} to {point2} on display");
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
        println!("Getting the center point of display");
        Point {
            x: self.width / 2,
            y: self.height / 2,
        }
    }
    pub fn total_area(&self) -> usize {
        println!("Getting total area of display");
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
        println!("Drawing a random line on screen");
        // function returns the random line that was made
        let mut rng = rand::thread_rng();
        let rx1 = rng.gen_range(0..self.screen.max_x);
        let ry1 = rng.gen_range(0..self.screen.max_y);
        let rx2 = rng.gen_range(0..self.screen.max_x);
        let ry2 = rng.gen_range(0..self.screen.max_y);
        self.draw_line(Point { x: rx1, y: ry1 }, Point { x: rx2, y: ry2 }, draw_val);
        (Point { x: rx1, y: ry1 }, Point { x: rx2, y: ry2 })
    }
    pub fn fill_screen(&mut self, new_ch:char) {
        println!("Filling screen with {}", new_ch);
        // for inner_vec in &self.screen.vec {
        //     for datapoint in inner_vec {
        //         datapoint.update(new_ch);
        //     }
        // }
        // TODO, PROBLEM IN THIS FUNCTION, PROBABLY
        self.screen.vec.iter().for_each(|inner_vec| {inner_vec.iter().for_each(|datapoint|datapoint.update(new_ch))});
    }
    pub fn allocate(
        &self,
        left: usize,
        right: usize,
        top: usize,
        bottom: usize,
    ) -> Vec2<Ptr<DataPoint>> {
        println!("Allocating box for object from display");
        // no need to check for bottom and left as we use usizes and they cant be negative

        if right < left  {
            panic!("Invalid allocation region: right < left");
        }
        if top < bottom {
            panic!("Invalid allocation region: top < bottom");
        }

        // CHECK FOR BOUNDARIES, no mut vars when function gets
        let right = if right >= self.width { self.screen.vec.len() - 1 } else { right };
        let top = if top >= self.height { self.screen.vec[0].len() - 1 } else { top };
        
        let width = right - left + 1; // Add 1 to include the last column
        let height = top - bottom + 1; // Add 1 to include the last row

        let mut reference_vec2: Vec2<Ptr<DataPoint>> = Vec2::new(width, height);

        for row in top..=bottom {
            for column in left..=right {
                // TODO: need to recheck this code here, Ptr::new_from_var
                let datapoint_ref = self.screen.index_ref(column, row);
                reference_vec2.index_mut_ref(column,row).set_ptr_to_ptr(datapoint_ref as *const DataPoint as *mut DataPoint);
                // reference_vec2.index(line, column) = Ptr::new_from_ptr(&self.screen.vec[line][column] as *const DataPoint as *mut DataPoint)
                // let raw_pointer: *mut DataPoint = &self.screen.vec[line][row] as *const DataPoint as *mut DataPoint;
                // reference_vec2.vec[line][row] = raw_pointer;
            }
        }
        reference_vec2
    }
    pub fn add_object(&mut self,mut object: Object) {
        println!("Adding object to display");
        object.allocate_from_display(self);
        self.boxer.push(object);
    }
    pub fn initialize_object(&self, obj_ref: &mut Object)  {
        println!("Initializing object from display");
        obj_ref.set_allocated_box(self.allocate(2,10,10,2));
        // obj_ref.allocate_box(self.allocate(2,10,2,10));
    }
}
impl std::fmt::Display for Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display implementation for Vec2<Point>
        write!(f, "{}", self.screen)
    }
}