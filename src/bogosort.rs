use crate::visualizer;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn bogo_sort(array: &mut [i32]) {
    let mut rng = thread_rng();
    while !is_sorted(array) {
        array.shuffle(&mut rng);
        visualizer::draw(array);
    }
}

fn is_sorted(array: &[i32]) -> bool {
    array.windows(2).all(|w| w[0] <= w[1])
}