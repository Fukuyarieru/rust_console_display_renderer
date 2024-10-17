#![allow(unused_variables)]
use crate::adapters::MenuAdapter;
enum Menu {
    // ALL POSSIBLE MENUS SHOULD BE WRITTEN HERE
    MainMenu,
    DisplaysManager,
    DisplayActionMenu,
    DisplayShapeActionMenu,
    SelectOptionMenu(Vec<String>),
}
// TODO use MenuAdapter for everything here
struct MenuStructure {
    title_bar: String,
    title: String,
    borders_ch: char,
    message: String, // main message, can be nothing (to stay cleared or just none)
    options: Vec<String>, // numbered, can be nothing (use for example as in errors)
    // how high and wide should the blank space inside the menu window be
    window_width_size: usize,
    window_height_size: usize,
}
impl MenuStructure {
    fn create(
        title_bar: String,
        title: String,
        borders_ch: char,
        message: String,
        options: Vec<String>,
        window_width_size: usize,
        window_height_size: usize,
    ) -> String {
        let mut menu: String;
        let mut top_border = String::from(borders_ch).repeat((window_width_size + 1) / 2);
        top_border += " ";
        top_border += &title_bar;
        top_border += " ";
        top_border += String::from(borders_ch).repeat((window_width_size + 1) / 2);
        // top_border += String::from(" ") + &title_bar + String::from(" ") + &top_border;
    }
}

// border_ch============title_bar======================
//
//                       title
//
//                      message
//
//                    -option1
//                    -option2
//                    -no_option3
//
// border_ch===========================================

/*

*/

// create using a copy of this
// ====================================================
// example
//                    Example Template
// example
// ====================================================

// TODO need to create logic to find best text placement in the middle (i think based on longest option text)

// ====================================================
//
//                    Main Menu
//
//             - View Tutorial
//             - View summery for all abilities
//             - Manage Displays
//             - Reset Program
//
// ====================================================

// ====================================================
// example
//                 Displays Manager
//
//              List of Display:
//              - existing_display1
//              - existing_display2
// example
// ====================================================
