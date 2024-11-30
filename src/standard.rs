#[derive(Clone)]
pub struct Vec2<T> {
    pub vec: Vec<Vec<T>>,
    pub max_x: usize,
    pub max_y: usize,
}
impl<T> Vec2<T> {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            vec: {
                let mut vec2= Vec::<Vec<T>>::with_capacity(x_size);
                for _ in 0..x_size {
                    vec2.push(Vec::with_capacity(y_size));
                };
                vec2
            },
            max_x: x_size,
            max_y: y_size,
        }
    }
    pub fn index(&mut self, x: usize, y: usize) -> &mut T {
        if x>self.max_x{
            let x=self.max_x;
        };
        if y>self.max_y{
            let y=self.max_y;
        };
        &mut self.vec[y][x]
    }
}

impl<T> std::fmt::Display for Vec2<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for inner_vec in &self.vec {
            for var in inner_vec {
                write!(f, "{}", var)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// making this here, I learned, there is std::ptr::*, check it out later, TODO
pub struct Ptr<T> {
    ptr: *mut T,
}
impl<T> Ptr<T> {
    pub fn new_from_val(var: T) -> Self {
        Self { ptr : &var as *const T as *mut T }
    }
    pub fn new_from_ptr(ptr: *mut T) -> Self {
        Self { ptr }
    }
    pub fn set_ptr_to_var(&mut self, var: T) {
        self.ptr = &var as *const T as *mut T;
    }
    pub fn set_ptr_to_ptr(&mut self,ptr: *mut T) {
        self.ptr = ptr;
    }
    pub fn get_ptr(&self) -> *mut T {
        self.ptr
    }
    pub fn get_var(&self) -> T {
        unsafe {self.ptr.read()}
    }
    pub fn get_ref(&self) -> &T {
        // AAAAAAAA
            unsafe {self.ptr.as_ref().unwrap()}
    }
    pub fn make_ptr_from_var(var: T) -> *mut T {
        &var as *const T as  *mut T
    }
}
impl<T> std::fmt::Display for Ptr<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_var())
    }
}



pub trait NotAPtr{}
/* a trait to make sure to distinguish 
between the generic implementation of
 fmt of Vec2 to its other implementation
  inside the Object code */


// TODO: play around here to fix these trait rules

// impl<T> NotAPtr for T{}
// impl<T> std::fmt::Display for Vec2<T>
// // chatgpt
// where
//     T: std::fmt::Display,
//
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for row in &self.vec {
//             for (i, elem) in row.iter().enumerate() {
//                 // Separate elements by a space, except for the last element in the row
//                 if i > 0 {
//                     write!(f, " ")?;
//                 }
//                 write!(f, "{}", elem)?;
//             }
//             // Print a new line after each row
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }
#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
