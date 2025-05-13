use crate::visualizer;

pub fn bubble_sort(array: &mut [i32]) {
    let n = array.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                visualizer::draw(array);
            }
        }
    }
}
