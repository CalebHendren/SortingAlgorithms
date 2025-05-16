use crate::visualizer;

/// Performs radix sort on a mutable slice of i32 numbers.
///
/// This implementation uses a least significant digit (LSD) radix sort. It handles negative numbers
/// correctly by flipping the sign bit before sorting, then re-flipping it afterward.
pub fn radix_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    // Transform each element by flipping its sign bit.
    // This converts the signed integers into a representation that sorts correctly when treated as unsigned.
    let mut aux: Vec<u32> = arr.iter().map(|&x| (x as u32) ^ 0x8000_0000).collect();
    let mut output = vec![0u32; n];

    // Process 4 bytes (32 bits) in steps of 8 bits.
    for shift in (0..32).step_by(8) {
        let mut count = [0usize; 256];

        // Count the frequency of each byte value at the current shift.
        for &num in aux.iter() {
            let digit = ((num >> shift) & 0xFF) as usize;
            count[digit] += 1;
        }

        // Compute prefix sums to determine positions.
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Build the output array in a stable manner by iterating backwards.
        for &num in aux.iter().rev() {
            let digit = ((num >> shift) & 0xFF) as usize;
            count[digit] -= 1;
            output[count[digit]] = num;
        }
        // Prepare for next iteration.
        aux.copy_from_slice(&output);
        // Visualize the intermediate state by converting back to signed integers.
        let temp: Vec<i32> = aux.iter().map(|&num| (num ^ 0x8000_0000) as i32).collect();
        visualizer::draw(&temp);
    }

    // Transform back by flipping the sign bit again.
    for (i, &num) in aux.iter().enumerate() {
        arr[i] = (num ^ 0x8000_0000) as i32;
        visualizer::draw(arr);
    }
}