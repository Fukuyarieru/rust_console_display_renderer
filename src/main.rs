#![allow(dead_code)]

mod shapes;
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use shapes::*;
use std::io::Write;
use text_io::*;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // let circle: Circle;

    println!("Width and height ratio should 4:1, giving lower values will run everything faster");
    print!("Width (120 recommended): ");
    std::io::stdout().flush().unwrap(); // Ensures the prompt appears before reading input
    let width: usize = read!();
    print!("Height (30 recommended): ");
    let height: usize = read!();
    let display = Display::new(width, height); // 104,27
                                               // let mut input: String;
    let options = vec![
        String::from("loading"),
        String::from("random-lines"),
        String::from("random-screen"),
    ];
    // loop {
    //     // Print the available options
    //     println!("Options: {:?}", options);

    //     // Read user input using text_io
    //     let mut input = read!();

    //     // Remove any trailing newline or whitespace
    //     // let input = input.trim();

    //     // Check if the input matches any of the options
    //     if options.iter().any(|option| *option == input) {
    //         println!("Valid option selected: {}", input);
    //         break; // Exit the loop if the input is valid
    //     } else {
    //         println!("Invalid option. Please try again.");
    //     }
    match screen.{
        "random-screen" => randomize_screen_animation(display),
        "random-lines" => random_lines_animation(display),
        "loading" => circle_animation(display),
        _ => println!("No animation was loaded, for some reason? heh???"),
    }
}

fn option_loader() {}
fn random_char(blank_percentage: f32) -> char {
    // Define the character set (can be customized as needed)
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // Create a random number generator
    let mut rng = thread_rng();

    // Get a random index within the range of the characters
    let index = rng.gen_range(0..chars.len());

    // Return the character at the random index
    if (rng.gen_range(0..100) as f32) < blank_percentage {
        ' '
    } else {
        chars.chars().nth(index).unwrap()
    }
}

fn load_animation(display: &mut Display, thing: char) {
    let mut x = 0;
    let mut y = 0;

    let (cx, cy) = display.get_center();

    while x < display.width {
        display.draw_line(x, y, cx, cy, thing);
        x += 1;
        println!("{}", display);
    }
    x -= 1;
    while y < display.height {
        display.draw_line(x, y, cx, cy, thing);
        y += 1;
        println!("{}", display);
    }
    y -= 1;
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
    screen: Vec<u8>,
}
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let screen_string =
            std::str::from_utf8(&self.screen).unwrap_or("[Error displaying screen]");
        write!(f, "{}", screen_string)
    }
}
impl Display {
    fn new(width: usize, height: usize) -> Display {
        let screen = make_a_screen(width, height, ' '); // Fill with spaces initially
        Display {
            width,
            height,
            screen, // Allocate space for the screen        }
        }
    }

    fn get_area(&self) -> usize {
        self.width * self.height
    }

    fn get_resolution(&self) -> String {
        self.width.to_string() + "x" + &self.height.to_string()
    }

    fn fill_screen_with(&mut self, thing: char) {
        self.screen = make_a_screen(self.width, self.height, thing)
    }

    fn pixel(&mut self, x: usize, y: usize, thing: char) {
        if x >= self.width || y >= self.height {
            return;
        }
        // Adjust index calculation to account for the newlines
        let index = y * (self.width + 1) + x;
        self.screen[index] = thing as u8;
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
    }
    #[allow(unused_variables)]
    fn render_circle(circle: Circle) {
        todo!()
    }
    fn animations_menu(options: Vec<String>) -> String {
        let mut input: String;
        loop {
            // Print the available options
            println!("Options: {:?}", options);

            // Read user input using text_io
            input = read!();

            // Remove any trailing newline or whitespace
            // let input = input.trim();

            // Check if the input matches any of the options
            if options.iter().any(|option| *option == input) {
                println!("Valid option selected: {}", input);
                break; // Exit the loop if the input is valid
            } else {
                println!("Invalid option. Please try again.");
            }
        }
        input
    }
    fn randomize_screen_animation(mut display: Display) {
        loop {
            display.randomize_screen(random_char(50.0), 50.0);
            // display.random_line(random_char(50.0));
            print!("{}", display);
        }
    }
    fn random_lines_animation(mut display: Display) {
        loop {
            display.random_line(random_char(50.0));
            print!("{}", display);
        }
    }
    fn circle_animation(mut display: Display) {
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
}
fn make_a_screen(width: usize, height: usize, thing: char) -> Vec<u8> {
    let mut screen = Vec::with_capacity((width + 1) * height); // +1 for '\n' at the end of each line
    for _ in 0..=height {
        screen.extend(vec![thing as u8; width]); // Add the characters for each row
        screen.push(b'\n'); // Add newline after each row
    }
    // screen.pop();
    screen
}
// fn randomize_screen(things: [char], probabilities: [i8]) {}
pub fn calc_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    let dx = (x2 as i64 - x1 as i64).pow(2);
    let dy = (y2 as i64 - y1 as i64).pow(2);
    ((dx + dy) as f32).sqrt()
}
