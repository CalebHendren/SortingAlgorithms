mod visualizer;
mod algorithms;
pub mod rng;
use std::io::{self, Write};

use algorithms::{
    bogosort, bozosort, bricksort, bubblesort, bucketsort, cocktailshakersort, combsort,
    countingsort, cyclesort, elonsort, gnomesort, heapsort, humansort, insertionsort,
    intelligentdesignsort, mergesort, miraclesort, pigeonholesort, powersort, quantumbogosort,
    quicksort, radixsort, shellsort, slowsort, stalinsort, timsort,
};

// Define a type for sorting functions to make the code cleaner
type SortFn = fn(&mut [i32]);

// Struct to hold information about a sorting algorithm
struct SortingAlgorithm {
    name: &'static str,
    sort_fn: SortFn,
}

// Enum to differentiate between standard and meme algorithms
enum AlgorithmCategory {
    Standard(Vec<SortingAlgorithm>),
    Meme(Vec<SortingAlgorithm>),
}

fn main() {
    // Define standard sorting algorithms
    let standard_algorithms = AlgorithmCategory::Standard(vec![
        SortingAlgorithm { name: "Brick Sort", sort_fn: bricksort::brick_sort },
        SortingAlgorithm { name: "Bubble Sort", sort_fn: bubblesort::bubble_sort },
        SortingAlgorithm { name: "Bucket Sort", sort_fn: bucketsort::bucket_sort },
        SortingAlgorithm { name: "Cocktail Shaker Sort", sort_fn: cocktailshakersort::cocktail_shaker_sort },
        SortingAlgorithm { name: "Comb Sort", sort_fn: combsort::comb_sort },
        SortingAlgorithm { name: "Counting Sort", sort_fn: countingsort::counting_sort },
        SortingAlgorithm { name: "Cycle Sort", sort_fn: cyclesort::cycle_sort },
        SortingAlgorithm { name: "Gnome Sort", sort_fn: gnomesort::gnome_sort },
        SortingAlgorithm { name: "Heap Sort", sort_fn: heapsort::heap_sort },
        SortingAlgorithm { name: "Insertion Sort", sort_fn: insertionsort::insertion_sort },
        SortingAlgorithm { name: "Merge Sort", sort_fn: mergesort::merge_sort },
        SortingAlgorithm { name: "Pigeonhole Sort", sort_fn: pigeonholesort::pigeonhole_sort },
        SortingAlgorithm { name: "Power Sort", sort_fn: powersort::power_sort },
        SortingAlgorithm { name: "Quick Sort", sort_fn: quicksort::quicksort },
        SortingAlgorithm { name: "Radix Sort", sort_fn: radixsort::radix_sort },
        SortingAlgorithm { name: "Shell Sort", sort_fn: shellsort::shell_sort },
        SortingAlgorithm { name: "Tim Sort", sort_fn: timsort::timsort },
    ]);

    // Define meme sorting algorithms
    let meme_algorithms = AlgorithmCategory::Meme(vec![
        SortingAlgorithm { name: "Bogo Sort", sort_fn: bogosort::bogo_sort },
        SortingAlgorithm { name: "Bozo Sort", sort_fn: bozosort::bozo_sort },
        SortingAlgorithm { name: "Elon Sort", sort_fn: elonsort::elon_sort },
        SortingAlgorithm { name: "Human Sort", sort_fn: humansort::human_sort },
        SortingAlgorithm { name: "Intelligent Design Sort", sort_fn: intelligentdesignsort::intelligent_design_sort },
        SortingAlgorithm { name: "Miracle Sort", sort_fn: miraclesort::miracle_sort },
        SortingAlgorithm { name: "Quantum Bogo Sort", sort_fn: quantumbogosort::quantum_bogo_sort },
        SortingAlgorithm { name: "Slow Sort", sort_fn: slowsort::slowsort },
        SortingAlgorithm { name: "Stalin Sort", sort_fn: stalinsort::stalinsort },
    ]);

    loop {
        display_menu(&standard_algorithms);
        let choice = get_user_choice(0, get_algorithm_count(&standard_algorithms));

        let mut array = if choice == 0 {
            match show_meme_menu(&meme_algorithms) {
                Some(meme_choice) => {
                    let mut arr = get_array_settings();
                    visualizer::draw(&arr);
                    run_sorting_algorithm(meme_choice, &mut arr);
                    arr
                }
                None => continue,
            }
        } else {
            let mut arr = get_array_settings();
            visualizer::draw(&arr);
            if let AlgorithmCategory::Standard(ref algos) = standard_algorithms {
                run_sorting_algorithm(algos[choice - 1].sort_fn, &mut arr);
            }
            arr
        };

        visualizer::draw(&array);
        println!("Sorting complete. Press Enter to return to menu.");
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
    }
}

// Display menu for standard algorithms
fn display_menu(category: &AlgorithmCategory) {
    println!("Choose a sorting algorithm:");
    println!("0. Meme Algorithms");
    if let AlgorithmCategory::Standard(ref algos) = category {
        for (i, algo) in algos.iter().enumerate() {
            println!("{}. {}", i + 1, algo.name);
        }
    }
}

// Get the number of algorithms in a category
fn get_algorithm_count(category: &AlgorithmCategory) -> usize {
    match category {
        AlgorithmCategory::Standard(algos) => algos.len(),
        AlgorithmCategory::Meme(algos) => algos.len(),
    }
}

// Get user choice with input validation
fn get_user_choice(min: usize, max: usize) -> usize {
    loop {
        let mut choice = String::new();
        print!("Enter choice (0-{}): ", max);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice.parse::<usize>() {
            Ok(num) if num >= min && num <= max => return num,
            _ => {
                println!("Please enter a number between 0 and {}.", max);
                continue;
            }
        }
    }
}

// Display menu for meme algorithms and handle selection
fn show_meme_menu(category: &AlgorithmCategory) -> Option<SortFn> {
    loop {
        println!("\nChoose a meme sorting algorithm:");
        println!("0. Back to Main Menu");
        if let AlgorithmCategory::Meme(ref algos) = category {
            for (i, algo) in algos.iter().enumerate() {
                println!("{}. {}", i + 1, algo.name);
            }
        }

        let choice = get_user_choice(0, get_algorithm_count(category));
        if choice == 0 {
            return None;
        }
        if let AlgorithmCategory::Meme(ref algos) = category {
            return Some(algos[choice - 1].sort_fn);
        }
    }
}

// Run the selected sorting algorithm
fn run_sorting_algorithm(sort_fn: SortFn, array: &mut Vec<i32>) {
    sort_fn(array);
}

// Get array settings from user input or use defaults
fn get_array_settings() -> Vec<i32> {
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

    rng::generate_random_array(min, max, count, allow_duplicates)
}

// Read numerical input from user with validation
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

// Read boolean input from user (y/n)
fn read_input_bool(prompt: &str) -> bool {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}