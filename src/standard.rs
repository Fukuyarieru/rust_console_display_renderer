#[derive(Clone,Debug)]
pub struct Vec2<T> {
    pub vec: Vec<Vec<T>>,
    pub max_x: usize,
    pub max_y: usize,
}
impl<T> Vec2<T> {
    pub fn new(x_size: usize, y_size: usize) -> Self where T: Default {
        Self {
            vec: {
                let mut vec2= Vec::<Vec<T>>::with_capacity(x_size);

                vec2.resize_with(x_size, || {
                    let mut inner_vec = Vec::new();
                    inner_vec.resize_with(y_size, T::default);
                    inner_vec
                });

                // vec2.resize_with(x_size, Vec::resize_with(&mut Vec::new(), y_size,  T::default) );

                // vec2.fill_with(||Vec::with_capacity(y_size));
                // vec2.iter_mut().for_each(|inner_vec|inner_vec.fill_with(T::default));

                // for _ in 0..x_size {
                //     vec2.push(Vec::with_capacity(y_size));
                // }
                // vec2.iter_mut().for_each(|inner_vec| {
                //     for _ in 0..y_size {
                //         inner_vec.push(Default::default());
                //     }
                // });
                // vec2
                vec2
            },
            max_x: x_size,
            max_y: y_size,
        }
    }
    pub fn index_mut_ref(&mut self, x: usize, y: usize) -> &mut T {
        let x= if x >= self.max_x { self.max_x - 1 } else { x };
        let y = if y >= self.max_y { self.max_y - 1 } else { y };
        &mut self.vec[x][y]
    }
    pub fn index_ref(&self, x: usize, y: usize) -> &T {
        let x= if x > self.max_x { self.max_x } else { x };
        let y = if y > self.max_y { self.max_y } else { y };
        &self.vec[x][y]
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


// reimplementation of Ptr using Rc<RefCell<T>>

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone,Debug)]
pub struct Ptr<T> {
    ptr: Rc<RefCell<T>>, // Shared, mutable reference to T
}

impl<T> Ptr<T> {
    // Create a new Ptr from an owned value
    pub fn new_from_val(var: T) -> Self {
        Self {
            ptr: Rc::new(RefCell::new(var)),
        }
    }

    // Create a new Ptr from an Rc<RefCell<T>>
    // probably useful
    pub fn new_from_ptr(ptr: Rc<RefCell<T>>) -> Self {
        Self { ptr }
    }

    // Set the value of the Ptr to a new value
    pub fn set_ptr_to_var(&self, var: T) {
        let mut value = self.ptr.borrow_mut();
        *value = var;
    }

    // Clone an existing Rc<RefCell<T>> and set it as the pointer
    pub fn set_ptr_to_ptr(&mut self, ptr: Rc<RefCell<T>>) {
        self.ptr = ptr;
    }

    // Get a cloned Rc<RefCell<T>> for shared ownership
    // forget about implementation using &T for now, it's probably a flawed idea
    pub fn get_ptr(&self) -> Rc<RefCell<T>> {
        Rc::clone(&self.ptr)
    }

    // Get a copy of the inner value (requires T: Clone)
    pub fn get_var(&self) -> T
    where
        T: Clone,
    {
        // self.ptr.borrow().clone()
        self.ptr.borrow().clone()
    }

    // Get an immutable reference to the inner value
    pub fn get_ref(&self) -> std::cell::Ref<T> {
        self.ptr.borrow()
    }

    // Get a mutable reference to the inner value
    pub fn get_mut_ref(&self) -> std::cell::RefMut<T> {
        self.ptr.borrow_mut()
    }

    // Helper to create an Rc<RefCell<T>> from a value
    pub fn make_ptr_from_var(var: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(var))
    }
}


// pub struct Ptr<T> {
//     ptr: *mut T,
// }
// impl<T> Ptr<T> {
//     pub fn new_from_val(var: &T) -> Self {
//         Self { ptr : var as *const T as *mut T }
//     }
//     pub fn new_from_ptr(ptr: *mut T) -> Self {
//         Self { ptr }
//     }
//     pub fn set_ptr_to_var(&mut self, var: &T) {
//         self.ptr = var as *const T as *mut T;
//     }
//     pub fn set_ptr_to_ptr(&mut self,ptr: *mut T) {
//         self.ptr = ptr;
//     }
//     pub fn get_ptr(&self) -> *mut T {
//         self.ptr
//     }
//     pub fn get_var(&self) -> T {
//         unsafe {self.ptr.read()}
//     }
//     pub fn get_ref(&self) -> &T {
//         // AAAAAAAA
//         if self.ptr.is_null() {
//             panic!("Attempting to dereference a null pointer");
//         }
//         
//             unsafe {self.ptr.as_ref().unwrap()}
//         // unsafe{&*self.ptr}
//         /* error: process didn't exit successfully: `target\debug\attempt_at_something_idk.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION) */
//     }
//     pub fn make_ptr_from_var(var: T) -> *mut T {
//         &var as *const T as  *mut T
//     }
// }
impl<T> Default for Ptr<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
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
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
