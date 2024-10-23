use std::ops::Index;

// use crate::adapters::DisplayAdapter;
use crate::animations::Animation;
use crate::functions::calc_distance;
use crate::object::{self, Object};
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
        self.val = *self.vals_history.index(0);
        self.vals_history.remove(0);
        self.vals_history.insert(self.vals_history.len(), ' ');
    }
}
impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
#[derive(Clone)]
pub struct Vec2<T> {
    pub vec: Vec<Vec<T>>,
    max_x: usize,
    max_y: usize,
}
impl<T> Vec2<T> {
    fn index(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.vec[x][y]
    }
    pub fn create(x_size: usize, y_size: usize, val: T) -> Self
    where
        // `T` must implement `Clone` to duplicate `val` across
        T: Clone,
    {
        Vec2 {
            vec: vec![vec![val; y_size]; x_size],
            max_x: x_size,
            max_y: y_size,
        }
    }
    // fn create_with_history_size(
    //     x_size: usize,
    //     y_size: usize,
    //     ch: char,
    //     history_size: usize,
    // ) -> Self {
    //     Vec2 {
    //         vec: vec![vec![Point::create(ch, history_size); y_size]; x_size],
    //     }
    // }
    // fn to_string(&self) -> String {
    //     let screen_string = std::str::from_utf8(&self.vec).unwrap_or("[Error displaying screen]");
    // }
}
impl<T> std::fmt::Display for Vec2<T>
// chatgpt
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.vec {
            for (i, elem) in row.iter().enumerate() {
                // Separate elements by a space, except for the last element in the row
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", elem)?;
            }
            // Print a new line after each row
            writeln!(f)?;
        }
        Ok(())
    }
}
pub struct Display<'a> {
    pub screen: Vec2<DataPoint>,
    pub width: usize,
    pub height: usize,
    pub boxer: Vec<Object<'a>>,
    // more stuff here later, probably (panels, info, titlebar)
}

// Question: do i do this in this way? do i need it this way?
pub enum DisplayAction {
    DrawLine((usize, usize), (usize, usize), char),
    HashPixels, // get all pixels content told inside a hashmap - wanted to make panel for later
    ClearScreen, // ehh?
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
    // TODO I should simplify the use of index and entry into Vec2 elements, problems can be seen in draw_line, here, and in the implementation of index()
    fn pixel(&mut self, point: (usize, usize), new_val: char) {
        // heh i got two implementations ??!! :D??
        (self.screen.vec[point.0][point.1]).update(new_val);
        // self.screen.index(point.0, point.1).update(new_val);
    }

    // IT WORKS! :D
    // fn test(&mut self) {
    //     let mut cache: Vec<char> = Vec::new();
    //     self.screen
    //         .vec
    //         .iter()
    //         .for_each(|vec| vec.iter().for_each(|var| cache.push(var.val)));
    // }
    pub fn draw_line(&mut self, point1: (usize, usize), point2: (usize, usize), draw_val: char) {
        // redeclaration
        let x1 = point1.0;
        let x2 = point2.0;
        let y1 = point1.1;
        let y2 = point2.1;

        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.pixel((x as usize, y as usize), draw_val); // Set pixel

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
    // not a good idea to make this, we're executing an animation on a screen, not an animation from a screen
    // fn animation_start(&mut self, animation: Animation) {
    //     todo!()
    // }
    fn copy_screen(&mut self, screen: Vec2<DataPoint>) {
        //?????, Vec2<char>? ???
        self.screen = screen;
    }
    fn get_center(&self) -> (usize, usize) {
        (self.width / 2, self.height / 2)
    }
    fn randomize_screen(&mut self, draw_val: char, screen_percentage: f32) {
        let screen_area = self.total_area as f64;
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
        self.draw_line((rx1, ry1), (rx2, ry2), draw_val);
        ((rx1, ry1), (rx2, ry2))
    }
    pub fn add(&mut self, object: Object<'a>) {
        self.boxer.push(object);
    }
    // TODO
    pub fn render(&mut self) {
        for object in &mut self.boxer {
            // Iterate mutably over `self.boxer`
            match object {
                Object::Shape {
                    shape,
                    center_point,
                    allocated_box,
                    draw_val,
                } => {
                    for vec in &mut allocated_box.vec {
                        // Iterate mutably over `allocated_box.vec`
                        for datapoint in vec {
                            datapoint.update(*draw_val); // Update using mutable `datapoint`
                        }
                    }
                }
                Object::Menu {
                    menu,
                    center_point,
                    allocated_box,
                } => todo!(),
            }
        }
    }
}
// Implement Display for the Display struct
impl<'a> std::fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the Display implementation for Vec2<Point>
        write!(f, "{}", self.screen)
    }
}
