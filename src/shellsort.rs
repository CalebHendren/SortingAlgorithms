use crate::visualizer;

pub fn shell_sort(array: &mut [i32]) {
    let n = array.len();
    // Start with a big gap, then reduce the gap
    let mut gap = n / 2;
    while gap > 0 {
        // Do a gapped insertion sort for this gap size.
        for i in gap..n {
            let temp = array[i];
            let mut j = i;
            // Shift earlier gap-sorted elements up until the correct location for array[i] is found
            while j >= gap && array[j - gap] > temp {
                array[j] = array[j - gap];
                j -= gap;
                visualizer::draw(array);
            }
            // Put temp (the original array[i]) in its correct location
            array[j] = temp;
            visualizer::draw(array);
        }
        // Reduce the gap for the next element
        gap /= 2;
    }
}