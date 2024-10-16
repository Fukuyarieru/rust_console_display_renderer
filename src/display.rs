use core::fmt;
use std::clone;

use crate::animations::Animation;
const DEFAULT_POINT_HISTORY_SIZE: usize = 3;
#[derive(Clone)]
struct Point {
    val: char,
    vals_history: Vec<char>,
}
impl Point {
    fn create(ch: char, history_size: usize) -> Self {
        Point {
            val: ch,
            vals_history: vec![' '; history_size],
        }
    }
    fn update(&mut self, new_ch: char) {
        self.vals_history.push(self.val);
        self.val = new_ch;
        self.vals_history.remove(self.vals_history.len() - 1);
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
#[derive(Clone)]
struct Vec2<T> {
    vec: Vec<Vec<T>>,
}
impl<T> Vec2<T> {
    fn index(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.vec[x][y]
    }
    fn create(x_size: usize, y_size: usize, val: T) -> Self
    where
        // `T` must implement `Clone` to duplicate `val` across
        T: Clone,
    {
        Vec2 {
            vec: vec![vec![val; y_size]; x_size],
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
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
pub struct Display {
    screen: Vec2<Point>,
    // more stuff here later, probably (panels, info, titlebar)
}

// Question: do i do this in this way? do i need it this way?
enum DisplayAction {
    DrawLine(Point, Point),
    HashPixels, // get all pixels content told inside a hashmap - wanted to make panel for later
    ClearScreen, // ehh?
}
impl Display {
    // Lets limit for now the use of individual pixels inside of the Display struct
    fn create(width: usize, height: usize) -> Self {
        Display {
            screen: Vec2::create(
                width,
                height,
                Point::create('#', DEFAULT_POINT_HISTORY_SIZE),
            ),
        }
    }
    fn pixel(&mut self, point: (usize, usize), new_val: char) {
        self.screen.index(point.0, point.1).update(new_val);
    }
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
    fn copy_screen(&mut self, screen: Vec2<Point>) {
        //?????, Vec2<char>? ???
        self.screen = screen;
    }
    fn get_center(&self) -> (usize, usize) {
        todo!()
    }
    fn randomize_screen(&mut self) {
        todo!()
    }
    fn random_line(&mut self) {
        todo!()
    }
}
// Implement Display for the Display struct
impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Display implementation for Vec2<Point>
        write!(f, "{}", self.screen)
    }
}
/* References
fn get_center(&self) -> (usize, usize) {
    (self.width / 2, self.height / 2)
}
fn randomize_screen(&mut self, thing: char, screen_percentage: f32) {
    let screen_area = self.get_area() as f32;
    let mut area_to_change: f32 = screen_area * screen_percentage / 100.0;
    while area_to_change > 0.0 {
        let (x1, y1, x2, y2) = self.random_line(thing);
        let line_length: f32 = calc_distance(x1, y1, x2, y2); // Assuming calc_distance takes coordinates and returns distance
        area_to_change -= line_length;
    }
}
fn random_line(&mut self, thing: char) -> (usize, usize, usize, usize) {
    let mut rng = rand::thread_rng();
    let rx1 = rng.gen_range(0..self.width);
    let ry1 = rng.gen_range(0..self.height);
    let rx2 = rng.gen_range(0..self.width);
    let ry2 = rng.gen_range(0..self.height);
    self.draw_line(rx1, ry1, rx2, ry2, thing);
    (rx1, ry1, rx2, ry2)
} */
