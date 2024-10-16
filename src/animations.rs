use crate::functions::*;
use crate::Display;
use text_io::*;

pub enum Animation {
    LoadingScreen,
    RandomLines,
    RandomScreens,
}

pub fn animations_menu(options: Vec<String>) -> String {
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

pub fn load_animation(display: &mut Display, thing: char) {
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
pub fn random_lines_animation(mut display: Display) {
    loop {
        display.random_line(random_char(50.0));
        print!("{}", display);
    }
}
pub fn randomize_screen_animation(mut display: Display) {
    loop {
        display.randomize_screen(random_char(50.0), 50.0);
        // display.random_line(random_char(50.0));
        print!("{}", display);
    }
}
pub fn circle_animation(mut display: Display) {
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
