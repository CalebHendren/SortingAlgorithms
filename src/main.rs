mod rng;
mod quicksort;
mod bubblesort;
mod heapsort;
mod bogosort;
mod visualizer;
mod insertionsort;
mod mergesort;
mod shellsort;
mod timsort;

use std::io::{self, Write};

fn main() {
    loop {
        println!("Choose a sorting algorithm:");
        println!("0. Bogosort");
        println!("1. Quicksort");
        println!("2. Bubblesort");
        println!("3. Heapsort");
        println!("4. Insertionsort");
        println!("5. Mergesort");
        println!("6. Shellsort");
        println!("7. Timsort");

        let mut choice = String::new();
        print!("Enter choice (0-7): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => {},
            _ => {
                println!("Only 0: Bogosort, 1: Quicksort, 2: Bubblesort, 3: Heapsort, 4: Insertionsort, 5: Mergesort, 6: Shellsort, and 7: Timsort are implemented.");
                continue;
            }
        }

        print!("Use default settings? (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let (min, max, count, allow_duplicates) = if input.trim().eq_ignore_ascii_case("y") {
            (0, 100, 100, true)
        } else {
            let min = read_input("Enter minimum value (e.g., 0): ") as i32;
            let max = read_input("Enter maximum value (e.g., 100): ") as i32;
            let count = read_input("Enter number of values (e.g., 100): ");
            let allow = read_input_bool("Allow duplicates? (y/n): ");
            (min, max, count, allow)
        };

        let mut array = rng::generate_random_array(min, max, count, allow_duplicates);
        visualizer::draw(&array);

        match choice {
            "0" => bogosort::bogo_sort(&mut array),
            "1" => quicksort::quicksort(&mut array),
            "2" => bubblesort::bubble_sort(&mut array),
            "3" => heapsort::heap_sort(&mut array),
            "4" => insertionsort::insertion_sort(&mut array),
            "5" => mergesort::merge_sort(&mut array),
            "6" => shellsort::shell_sort(&mut array),
            "7" => timsort::timsort(&mut array),
            _ => unreachable!(),
        }

        visualizer::draw(&array);

        println!("Sorting complete. Press Enter to return to menu.");
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
    }
}

fn read_input(prompt: &str) -> usize {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a non-negative integer."),
        }
    }
}

fn read_input_bool(prompt: &str) -> bool {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}