/// Performs bucket sort on a slice of i32 numbers.
///
/// This implementation creates a fixed number of buckets based on the input array's length,
/// distributes the elements by scaling their values between the minimum and maximum values,
/// sorts each bucket individually, and then concatenates the sorted buckets back into the original array.
use crate::visualizer;

pub fn bucket_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n == 0 {
        return;
    }

    // Determine the minimum and maximum values in the array.
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    // If all elements are equal, the array is already sorted.
    if min == max {
        return;
    }

    // Create n empty buckets.
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n];

    // Distribute array values into buckets.
    // The bucket index is determined by scaling the element within the range [min, max].
    for &val in arr.iter() {
        let index = (((val - min) as usize) * (n - 1)) / ((max - min) as usize);
        buckets[index].push(val);
    }

    // Sort individual buckets.
    for bucket in buckets.iter_mut() {
        bucket.sort();
    }

    // Concatenate sorted buckets back into arr.
    let mut idx = 0;
    for bucket in buckets {
        for val in bucket {
            arr[idx] = val;
            idx += 1;
        }
        // Update visualization after merging each bucket.
        visualizer::draw(arr);
    }
}