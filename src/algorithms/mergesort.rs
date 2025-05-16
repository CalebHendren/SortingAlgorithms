use crate::visualizer;

pub fn merge_sort(array: &mut [i32]) {
    if array.len() <= 1 {
        return;
    }
    merge_sort_recursive(array);
}

fn merge_sort_recursive(array: &mut [i32]) {
    let n = array.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    let mut left = array[..mid].to_vec();
    let mut right = array[mid..].to_vec();

    merge_sort_recursive(&mut left);
    merge_sort_recursive(&mut right);

    merge(&left, &right, array);
    visualizer::draw(array);
}

fn merge(left: &[i32], right: &[i32], array: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        array[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        array[k] = right[j];
        j += 1;
        k += 1;
    }
}