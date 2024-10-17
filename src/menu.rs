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
