#[allow(dead_code)]
use text_io::*;
fn main() {
    // std::allocenv::set_var("RUST_BACKTRACE", "1");
    println!("Width and height ratio should 4:1, giving lower values will run everything faster");
    print!("Width (120 recommended): ");
    let width: i32 = read!();
    let width: usize = width as usize;
    print!("Height (30 recommended): ");
    let height: i32 = read!();
    let height: usize = height as usize;

    let mut display = Display::new(width, height); // 104,27
    display.fill_screen_with('A');
    display.print();
    display.pixel(3, 3, 'c');
    display.print();
    for i in 'a'..='z' {
        load_animation(&mut display, ' ');
        load_animation(&mut display, i);
    }
    for i in 'A'..='Z' {
        load_animation(&mut display, ' ');
        load_animation(&mut display, i);
    }
    loop {
        load_animation(&mut display, '`');
        load_animation(&mut display, '#');
    }
}

#[allow(dead_code)]
fn clear_console_screen() {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.screen)
    }
}

fn load_animation(display: &mut Display, thing: char) {
    let mut x = 0;
    let mut y = 0;

    let (cx, cy) = display.get_center();

    while x < display.get_width() {
        display.draw_line(x, y, cx, cy, thing);
        x += 1;
        println!("{}", display);
    }

    while y < display.get_height() {
        display.draw_line(x, y, cx, cy, thing);
        y += 1;
        println!("{}", display);
    }

    while x > 0 {
        display.draw_line(x, y, cx, cy, thing);
        x -= 1;
        println!("{}", display);
    }

    while y > 0 {
        display.draw_line(x, y, cx, cy, thing);
        y -= 1;
        println!("{}", display);
    }
}

pub struct Display {
    width: usize,
    height: usize,
    screen: String,
}

impl Display {
    fn new(width: usize, height: usize) -> Display {
        Display {
            width,
            height,
            screen: make_a_screen(width, height, ' '),
        }
    }

    #[allow(dead_code)]
    fn get_area(&self) -> usize {
        self.width * self.height
    }

    #[allow(dead_code)]
    fn get_resolution(&self) -> String {
        self.width.to_string() + "x" + &self.height.to_string()
    }

    fn fill_screen_with(&mut self, thing: char) {
        self.screen = make_a_screen(self.width, self.height, thing)
    }
    fn print(&self) {
        println!("{}\n", self.screen)
    }
    fn pixel(&mut self, x: usize, y: usize, thing: char) {
        // Ensure x and y are within bounds
        if x >= self.width || y >= self.height {
            return; // Do nothing if out of bounds
        }

        // Calculate the index in the screen string
        let index = y * (self.width + 1) + x; // +1 because of the '\n' character after each row

        // Replace the character at the calculated index
        // Strings in Rust are immutable, so we need to convert to a vector of characters
        let mut screen_chars: Vec<char> = self.screen.chars().collect();
        screen_chars[index] = thing;

        // Convert the vector of characters back into a String
        self.screen = screen_chars.into_iter().collect();
    }

    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, thing: char) {
        // Bresenham's line algorithm
        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.pixel(x as usize, y as usize, thing); // Set pixel

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
        // println!(
        //     "Drawing line from ({}, {}) to ({}, {}) with '{}'",
        //     x1, y1, x2, y2, thing
        // );
    }
    fn get_center(&self) -> (usize, usize) {
        (self.width / 2, self.height / 2)
    }
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}
fn make_a_screen(width: usize, height: usize, thing: char) -> String {
    let new_line = thing.to_string().repeat(width); // Create a line of spaces of 'width' length

    // we return the expression directly
    (0..height)
        .map(|_| new_line.clone()) // Repeat this for each line of height
        .collect::<Vec<_>>() // Collect it into a vector of lines
        .join("\n")
}
