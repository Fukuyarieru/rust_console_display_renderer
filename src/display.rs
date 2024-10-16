use crate::animations::Animation;
const DEFAULT_HISTORY_SIZE: usize = 3;
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
struct Vec2 {
    vec: Vec<Vec<Point>>,
}
impl Vec2 {
    fn index(&self, x: usize, y: usize) -> &Point {
        &self.vec[x][y]
    }
    fn create(x_size: usize, y_size: usize, ch: char) -> Self {
        Vec2 {
            vec: vec![vec![Point::create(ch, DEFAULT_HISTORY_SIZE); y_size]; x_size],
        }
    }
    fn create_with_history_size(
        x_size: usize,
        y_size: usize,
        ch: char,
        history_size: usize,
    ) -> Self {
        Vec2 {
            vec: vec![vec![Point::create(ch, history_size); y_size]; x_size],
        }
    }
}
struct Display {
    screen: Vec2,
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
            screen: Vec2::create(width, height, '#'),
        }
    }
    fn draw_line(&mut self, point1: Point, point2: Point) {
        todo!()
    }
    fn animation_start(&mut self, animation: Animation) {
        todo!()
    }
    fn copy_screen(&mut self, screen: Vec2) {
        self.screen = screen;
    }
}
