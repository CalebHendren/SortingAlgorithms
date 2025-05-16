use std::io;
use crate::visualizer;

// Performs human sort on a mutable slice of i32 numbers.
// This function requires the user to manually type in the sorted order.
// If the entered order is correct (i.e. a non-decreasing permutation of the original array),
// the array is updated accordingly.
// Otherwise, the array is "deleted" by filling it with zeros.
pub fn human_sort(arr: &mut [i32]) {
    // Show the original array to the user.
    println!("Original array: {:?}", arr);
    println!("Manually sort the array by entering the sorted order as space-separated integers:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let user_sorted: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // If the number of entered elements doesn't match, it's an error.
    if user_sorted.len() != arr.len() {
        println!("Incorrect number of elements entered. Deleting array.");
        arr.fill(0);
        visualizer::draw(arr);
        return;
    }

    // To validate, compute the correctly sorted version of the original array.
    // Note: We use a clone of the original array before any user changes.
    let mut correct_sort = arr.to_vec();
    correct_sort.sort();

    // Check if the user's input is in non-decreasing order and matches the expected sorted result.
    if user_sorted == correct_sort {
        // Update the array with the user's correct input.
        arr.copy_from_slice(&user_sorted);
        visualizer::draw(arr);
    } else {
        println!("Sorting mistake detected. Deleting array.");
        arr.fill(0);
        visualizer::draw(arr);
    }
}