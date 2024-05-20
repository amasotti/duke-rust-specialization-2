use rand::{Rng, thread_rng};


/// Create a random sequence of `n` elements, where each element is a random number
/// between `min` and `max`.
pub fn create_random_seq(n: usize, min: u32, max: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut r : Vec<u32> = Vec::with_capacity(n);

    (0..n).for_each(|_| {  r.push(rng.gen_range(min..max)); });

    r
}
