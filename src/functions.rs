use rand::*;

pub fn random_char(blank_percentage: f32) -> char {
    // Define the character set (can be customized as needed)
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // Create a random number generator
    let mut rng = thread_rng();

    // Get a random index within the range of the characters
    let index = rng.gen_range(0..chars.len());

    // Return the character at the random index
    if (rng.gen_range(0..100) as f32) < blank_percentage {
        ' '
    } else {
        chars.chars().nth(index).unwrap()
    }
}
pub fn calc_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    let dx = (x2 as i64 - x1 as i64).pow(2);
    let dy = (y2 as i64 - y1 as i64).pow(2);
    ((dx + dy) as f32).sqrt()
}

fn test(vec: Vec<Vec<char>>) {
    // function to push all chars inside a vec of vectors into a new vector
    let mut cache: Vec<char> = Vec::new();
    vec.iter()
        .for_each(|inner_vec| inner_vec.iter().for_each(|ch| cache.push(*ch)));
}
