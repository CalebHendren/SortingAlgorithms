use crate::visualizer;

pub fn heap_sort(array: &mut [i32]) {
    let n = array.len();
    // Build max heap
    for start in (0..n/2).rev() {
        heapify(array, n, start);
    }

    // Extract elements from heap one by one
    for end in (1..n).rev() {
        // Swap max (at 0) with the last element
        array.swap(0, end);
        visualizer::draw(array);
        // Heapify reduced heap
        heapify(array, end, 0);
    }
}

fn heapify(array: &mut [i32], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size && array[left] > array[largest] {
        largest = left;
    }
    if right < heap_size && array[right] > array[largest] {
        largest = right;
    }

    if largest != root {
        array.swap(root, largest);
        visualizer::draw(array);
        heapify(array, heap_size, largest);
    }
}