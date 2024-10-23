// this will later replace main.rs
// for now i will work on the main here before i clear everything there
// original references are still heavily needed

// DO NOT USE ANY KIND OF #ALLOW[] BLOCK

// steps to run:
/*
Open Main Menu: Shows -> ViewTutorial,ViewSummeryOfAbilities,DisplaysManager,Reset

ViewTutorial -> My Tutorial for my program, TODO
ViewSummeryOfAbilities -> ??? TODO
DisplayManager -> DisplayManagerMenu -> existing_display1,existing_display2...
Reset -> Initiate Reset of the program, (probably pub fn RESET())


*/
use crate::display::DataPoint;
use crate::display::Display;
use crate::display::Vec2;

use crate::object::Object;

use crate::menu::Menu;
use crate::shape::Shape;

#[allow(non_snake_case)]
pub fn Runner() {
    let mut display: Display;

    display.add(Object::shape(
        Shape::Circle { radius: 5.0 },
        (3, 3),
        '#',
        &display,
    ));
}
