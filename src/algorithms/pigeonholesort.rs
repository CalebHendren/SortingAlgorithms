use crate::visualizer;

pub fn pigeonhole_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // Find min and max values
    let min_val = *arr.iter().min().unwrap();
    let max_val = *arr.iter().max().unwrap();
    let range = (max_val - min_val + 1) as usize;

    // Create temporary array for pigeonholes
    let mut tmp = vec![0; range];

    // Populate pigeonholes
    for &value in arr.iter() {
        let index = (value - min_val) as usize;
        tmp[index] += 1;
        visualizer::draw(arr);
    }

    // Put elements back into original array
    let mut index = 0;
    for j in 0..range {
        while tmp[j] > 0 {
            tmp[j] -= 1;
            arr[index] = j as i32 + min_val;
            index += 1;
            visualizer::draw(arr);
        }
    }
}