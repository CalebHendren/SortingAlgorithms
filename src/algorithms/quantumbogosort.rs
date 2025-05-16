use crate::visualizer;
use rand::seq::SliceRandom;

/// Performs quantum bogosort on a mutable slice of i32 numbers.
///
/// This function runs bogosort exactly one time (i.e. performs a single random permutation)
/// and then prints whether or not the resulting order is sorted. It visualizes the state
/// of the array after the permutation using the visualizer.
pub fn quantum_bogo_sort(arr: &mut [i32]) {
    // Perform a single bogosort attempt: shuffle the array randomly.
    let mut rng = rand::thread_rng();
    arr.shuffle(&mut rng);
    visualizer::draw(arr);

    // Check if the array is sorted.
    let is_sorted = arr.windows(2).all(|w| w[0] <= w[1]);

    if is_sorted {
        println!("This is the universe where it worked.");
    } else {
        println!("This is not the universe where it worked.");
    }
}