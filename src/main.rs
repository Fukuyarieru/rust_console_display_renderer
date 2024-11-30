#![allow(dead_code, unused_imports)]

// mod adapters;
mod display;
mod object;
use display::*;
// mod display_renderers;
mod menu;
// use display_renderers::*;
mod functions;
use functions::*;
mod shape;
use rand::*;
use shape::*;
use std::{env, io::Write};
use std::path::Iter;
use text_io::*;
mod standard;

use object::*;
use standard::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let a=std::ptr::from_ref("data");

    let temp_datapoint=DataPoint::new('c',3);
    println!("{}",temp_datapoint);
    temp_datapoint.update('b');
    // println!("{}", temp_datapoint);
    // println!("{temp_datapoint}"); // this works

    let mut obj=Object::new(Point{x:3,y:4},ObjType::Free{size:(3,3)});

    let mut display: Display = Display::new(30, 30);
    display.fill_screen('a');

    println!("{}",display);

    // display.initialize_object(&mut obj);
    // obj.allocated_box.unwrap().vec.iter().for_each(|inner_vec|inner_vec.iter().for_each(|datapoint|unsafe {
    //     // Dereference the raw pointer to access `val`
    //     let data_ref: &DataPoint = &**datapoint;
    //     println!("{}", data_ref.val.get());
    // }));
    // obj.fill_box('c');
    display.initialize_object(&mut obj);
    // println!("{}",obj);

    display.add_object(obj);

    // println!("{}",display);

}
// pub fn main() {
//     // std::env::set_var("RUST_BACKTRACE", "1");
//     // let circle: Circle;

//     println!("Width and height ratio should 4:1, giving lower values will run everything faster");
//     print!("Width (120 recommended): ");
//     std::io::stdout().flush().unwrap(); // Ensures the prompt appears before reading input
//     let width: usize = read!();
//     print!("Height (30 recommended): ");
//     let height: usize = read!();
//     let mut display = Display::new(width, height); // 104,27
//                                                    // let sh = Quadrilateral::create((0, 0), (0, 0), (0, 0), (0, 0));

//     // render_quadrilateral(
//     //     &mut display,
//     //     Quadrilateral::new((1, 3), (23, 7), (17, 15), (5, 3)),
//     //     'c',
//     // );
//     // println!("{}", display);
//     // // let mut input: String;
//     let options = vec![
//         String::from("loading"),
//         String::from("random-lines"),
//         String::from("random-screen"),
//     ];
//     match animations_menu(options).as_str() {
//         "random-screen" => randomize_screen_animation(display),
//         "random-lines" => random_lines_animation(display),
//         "loading" => circle_animation(display),
//         _ => println!("No animation was loaded, for some reason? heh???"),
//     }
// }

// pub struct Display {
//     width: usize,
//     height: usize,
//     screen: Vec<u8>,
// }
// impl std::fmt::Display for Display {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let screen_string =
//             std::str::from_utf8(&self.screen).unwrap_or("[Error displaying screen]");
//         write!(f, "{}", screen_string)
//     }
// }
// impl Display {
//     fn new(width: usize, height: usize) -> Display {
//         let screen = make_a_screen(width, height, ' '); // Fill with spaces initially
//         Display {
//             width,
//             height,
//             screen, // Allocate space for the screen        }
//         }
//     }

//     fn get_area(&self) -> usize {
//         self.width * self.height
//     }

//     fn get_resolution(&self) -> String {
//         self.width.to_string() + "x" + &self.height.to_string()
//     }

//     fn fill_screen_with(&mut self, thing: char) {
//         self.screen = make_a_screen(self.width, self.height, thing)
//     }

//     fn pixel(&mut self, x: usize, y: usize, thing: char) {
//         if x >= self.width || y >= self.height {
//             return;
//         }
//         // Adjust index calculation to account for the newlines
//         let index = y * (self.width + 1) + x;
//         self.screen[index] = thing as u8;
//     }

//     fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, thing: char) {
//         // Bresenham's line algorithm
//         let mut x = x1 as isize;
//         let mut y = y1 as isize;
//         let dx = (x2 as isize - x1 as isize).abs();
//         let dy = (y2 as isize - y1 as isize).abs();
//         let sx = if x1 < x2 { 1 } else { -1 };
//         let sy = if y1 < y2 { 1 } else { -1 };
//         let mut err = dx - dy;

//         loop {
//             self.pixel(x as usize, y as usize, thing); // Set pixel

//             if x == x2 as isize && y == y2 as isize {
//                 break;
//             }

//             let e2 = 2 * err;
//             if e2 > -dy {
//                 err -= dy;
//                 x += sx;
//             }
//             if e2 < dx {
//                 err += dx;
//                 y += sy;
//             }
//         }
//         // println!(
//         //     "Drawing line from ({}, {}) to ({}, {}) with '{}'",
//         //     x1, y1, x2, y2, thing
//         // );
//     }
//     fn get_center(&self) -> (usize, usize) {
//         (self.width / 2, self.height / 2)
//     }
//     fn randomize_screen(&mut self, thing: char, screen_percentage: f32) {
//         let screen_area = self.get_area() as f32;
//         let mut area_to_change: f32 = screen_area * screen_percentage / 100.0;
//         while area_to_change > 0.0 {
//             let (x1, y1, x2, y2) = self.random_line(thing);
//             let line_length: f32 = calc_distance(x1, y1, x2, y2); // Assuming calc_distance takes coordinates and returns distance
//             area_to_change -= line_length;
//         }
//     }
//     fn random_line(&mut self, thing: char) -> (usize, usize, usize, usize) {
//         let mut rng = rand::thread_rng();
//         let rx1 = rng.gen_range(0..self.width);
//         let ry1 = rng.gen_range(0..self.height);
//         let rx2 = rng.gen_range(0..self.width);
//         let ry2 = rng.gen_range(0..self.height);
//         self.draw_line(rx1, ry1, rx2, ry2, thing);
//         (rx1, ry1, rx2, ry2)
//     }
// }

// // fn make_a_screen(width: usize, height: usize, thing: char) -> Vec<u8> {
// //     let mut screen = Vec::with_capacity((width + 1) * height); // +1 for '\n' at the end of each line
// //     for _ in 0..=height {
// //         screen.extend(vec![thing as u8; width]); // Add the characters for each row
// //         screen.push(b'\n'); // Add newline after each row
// //     }
// //     // screen.pop();
// //     screen
// // }
// // fn randomize_screen(things: [char], probabilities: [i8]) {}
