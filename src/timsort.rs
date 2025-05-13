use crate::visualizer;

pub fn timsort(array: &mut [i32]) {
    let n = array.len();
    const MIN_RUN: usize = 32;

    // Step 1: Sort small runs with insertion sort.
    let mut i = 0;
    while i < n {
        let right = if i + MIN_RUN < n { i + MIN_RUN } else { n };
        insertion_sort(array, i, right);
        i += MIN_RUN;
    }

    // Step 2: Merge runs in a bottom-up manner.
    let mut size = MIN_RUN;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = if left + size < n { left + size } else { n };
            let right = if left + size * 2 < n { left + size * 2 } else { n };
            if mid < right {
                merge(array, left, mid, right);
            }
            left += size * 2;
        }
        size *= 2;
    }
}

fn insertion_sort(array: &mut [i32], left: usize, right: usize) {
    for i in (left + 1)..right {
        let key = array[i];
        let mut j = i;
        while j > left && array[j - 1] > key {
            array[j] = array[j - 1];
            j -= 1;
            visualizer::draw(array);
        }
        array[j] = key;
        visualizer::draw(array);
    }
}

fn merge(array: &mut [i32], left: usize, mid: usize, right: usize) {
    let mut left_vec = array[left..mid].to_vec();
    let mut right_vec = array[mid..right].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < left_vec.len() && j < right_vec.len() {
        if left_vec[i] <= right_vec[j] {
            array[k] = left_vec[i];
            i += 1;
        } else {
            array[k] = right_vec[j];
            j += 1;
        }
        k += 1;
        visualizer::draw(array);
    }

    while i < left_vec.len() {
        array[k] = left_vec[i];
        i += 1;
        k += 1;
        visualizer::draw(array);
    }

    while j < right_vec.len() {
        array[k] = right_vec[j];
        j += 1;
        k += 1;
        visualizer::draw(array);
    }
}