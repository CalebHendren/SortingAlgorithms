use std::{thread::sleep, time::Duration};
use std::io::{stdout, Write};

const FRAME_DELAY: Duration = Duration::from_millis(30);

/// Clear terminal screen
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

/// Draw the current array as bars
///
/// Every element starts as red. Once an element reaches its correct sorted position, it turns green.
pub fn draw(array: &[i32]) {
    let max_value = *array.iter().max().unwrap_or(&1);
    let height = 20;
    
    clear_screen();
    
    // Check if the entire array is sorted.
    let array_is_sorted = array.windows(2).all(|w| w[0] <= w[1]);
    
    // If the array is not sorted, prepare a sorted copy to compare positions.
    let sorted_copy = if !array_is_sorted {
        let mut copy = array.to_vec();
        copy.sort();
        Some(copy)
    } else {
        None
    };
    
    for level in (1..=height).rev() {
        for (i, &value) in array.iter().enumerate() {
            let bar_height = value * height / max_value;
            if bar_height >= level {
                if array_is_sorted {
                    // Array completely sorted: draw green.
                    print!("\x1B[32m█\x1B[0m ");
                } else {
                    // During sorting, if this element is in its sorted position, show green,
                    // otherwise show red.
                    if let Some(ref sorted) = sorted_copy {
                        if value == sorted[i] {
                            print!("\x1B[32m█\x1B[0m ");
                        } else {
                            print!("\x1B[31m█\x1B[0m ");
                        }
                    }
                }
            } else {
                print!("  ");
            }
        }
        println!();
    }
    
    stdout().flush().unwrap();
    sleep(FRAME_DELAY);
}