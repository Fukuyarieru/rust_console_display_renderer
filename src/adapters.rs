// idk how this will need to look like, for now everything here is just a placeholder

// ADAPTERS should link the logic between all of its respective modules
// Program logic should run inside the adapters and not the structures, as they have broader access and create a comprehensive sensible centre

use crate::animations;
use crate::display;
use crate::shapes;

// these are "supportive" structs, they will not create their own object but instead will cast actions between two different modules

pub struct DisplayAdapter {
    display: display::Display,
    display_action: display::DisplayAction,
}
pub struct MenuAdapter {}
pub struct ShapeAdapter {}
pub struct AnimationAdapter {
    display: display::Display,
    animation: animations::Animation,
}
