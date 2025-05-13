use crate::visualizer;

pub fn insertion_sort(array: &mut [i32]) {
    let n = array.len();
    for i in 1..n {
        let key = array[i];
        let mut j = i;
        // Move elements of array[0..i] that are greater than key one position ahead
        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1];
            visualizer::draw(array);
            j -= 1;
        }
        array[j] = key;
        visualizer::draw(array);
    }
}