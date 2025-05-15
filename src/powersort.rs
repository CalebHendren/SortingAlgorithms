use crate::visualizer::draw;

pub fn power_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    // Draw the initial state of the array
    draw(arr);

    let n = arr.len();
    // Perform an iterative version of PowerSort
    for i in 0..n {
        for j in (i + 1)..n {
            if arr[i] > arr[j] {
                arr.swap(i, j);
                // Visualize after each swap
                draw(arr);
            }
        }
    }
}