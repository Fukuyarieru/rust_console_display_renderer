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
    pub fn index(&mut self, mut x: usize, mut y: usize) -> Result<&mut T, &mut T> {
        let mut err = false;
        if x >= self.vec.len() {
            err = true;
            x = self.vec.len() - 1;
        }
        if y >= self.vec[0].len() {
            err = true;
            y = self.vec[0].len() - 1;
        }
        if err {
            Err(&mut self.vec[x][y])
        } else {
            Ok(&mut self.vec[x][y])
        }
    }
}



pub trait NotAPtr{} 
/* a trait to make sure to distinguish 
between the generic implementation of
 fmt of Vec2 to its other implementation
  inside the Object code */


// TODO: play around here to fix these trait rules
impl<T> NotAPtr for T where T: Sized {}
impl<T> NotAPtr for Vec2<T>{}
impl<T> std::fmt::Display for Vec2<T>
// chatgpt
where
    T: std::fmt::Display + NotAPtr,

{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.vec {
            for (i, elem) in row.iter().enumerate() {
                // Separate elements by a space, except for the last element in the row
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", elem)?;
            }
            // Print a new line after each row
            writeln!(f)?;
        }
        Ok(())
    }
}
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
