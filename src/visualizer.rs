use std::{thread::sleep, time::Duration};
use std::io::{stdout, Write};

const FRAME_DELAY: Duration = Duration::from_millis(30);

/// Clear terminal screen
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

/// Draw the current array as bars
pub fn draw(array: &[i32]) {
    let max_value = *array.iter().max().unwrap_or(&1);
    let height = 20;

    clear_screen();

    for level in (1..=height).rev() {
        for &value in array {
            let bar_height = value * height / max_value;
            if bar_height >= level {
                print!("█ ");
            } else {
                print!("  ");
            }
        }
        println!();
    }

    stdout().flush().unwrap();
    sleep(FRAME_DELAY);
}