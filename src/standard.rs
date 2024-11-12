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
                let mut vec: Vec<Vec<T>> = Vec::with_capacity(x_size);
                vec.fill_with(|| Vec::<T>::with_capacity(y_size));
                // Vec::with_capacity(x_size).fill_with(|| Vec::<T>::with_capacity(y_size));
                vec
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
impl<T> std::fmt::Display for Vec2<T>
// chatgpt
where
    T: std::fmt::Display,
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
