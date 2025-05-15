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
mod countingsort;
mod radixsort;
mod bucketsort;
mod cyclesort;
mod cocktailshakersort;
mod gnomesort;
mod combsort;
mod quantumbogosort;
mod stalinsort;
mod elonsort;
mod intelligentdesignsort;
mod miraclesort;
mod bozosort;
mod slowsort;
mod humansort;
mod powersort;

use std::io::{self, Write};

fn main() {
    loop {
        println!("Choose a sorting algorithm:");
        println!("0. Meme Algorithms");
        println!("1. Quicksort");
        println!("2. Bubblesort");
        println!("3. Heapsort");
        println!("4. Insertionsort");
        println!("5. Mergesort");
        println!("6. Shellsort");
        println!("7. Timsort");
        println!("8. Countingsort");
        println!("9. Radixsort");
        println!("10. Bucketsort");
        println!("11. Cyclesort");
        println!("12. Cocktail Shaker Sort");
        println!("13. Gnomesort");
        println!("14. Combsort");
        println!("15. Power Sort");

        let mut choice = String::new();
        print!("Enter choice (0-15): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "0" {
            let meme_choice = show_meme_menu();
            if let Some(selected) = meme_choice {
                run_sorting(selected, &mut || {
                    let mut array = get_array_settings();
                    visualizer::draw(&array);
                    array
                });
            }
            continue;
        }

        match choice {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" => {},
            _ => {
                println!("Only 0: Meme Algorithms, 1: Quicksort, 2: Bubblesort, 3: Heapsort, 4: Insertionsort, 5: Mergesort, 6: Shellsort, 7: Timsort, 8: Countingsort, 9: Radixsort, 10: Bucketsort, 11: Cyclesort, 12: Cocktail Shaker Sort, 13: Gnomesort, 14: Combsort, 15: Power Sort are implemented.");
                continue;
            }
        }

        let mut array = get_array_settings();
        visualizer::draw(&array);

        match choice {
            "1" => quicksort::quicksort(&mut array),
            "2" => bubblesort::bubble_sort(&mut array),
            "3" => heapsort::heap_sort(&mut array),
            "4" => insertionsort::insertion_sort(&mut array),
            "5" => mergesort::merge_sort(&mut array),
            "6" => shellsort::shell_sort(&mut array),
            "7" => timsort::timsort(&mut array),
            "8" => countingsort::counting_sort(&mut array),
            "9" => radixsort::radix_sort(&mut array),
            "10" => bucketsort::bucket_sort(&mut array),
            "11" => cyclesort::cycle_sort(&mut array),
            "12" => cocktailshakersort::cocktail_shaker_sort(&mut array),
            "13" => gnomesort::gnome_sort(&mut array),
            "14" => combsort::comb_sort(&mut array),
            "15" => powersort::power_sort(&mut array),
            _ => unreachable!(),
        }

        visualizer::draw(&array);

        println!("Sorting complete. Press Enter to return to menu.");
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
    }
}

fn show_meme_menu() -> Option<&'static str> {
    loop {
        println!("\nChoose a meme sorting algorithm:");
        println!("0. Back to Main Menu");
        println!("1. Bogosort");
        println!("2. Quantum Bogosort");
        println!("3. Stalin Sort");
        println!("4. Elon Sort");
        println!("5. Intelligent Design Sort");
        println!("6. Miracle Sort");
        println!("7. Bozo Sort");
        println!("8. Slow Sort");
        println!("9. Human Sort");

        let mut choice = String::new();
        print!("Enter choice (0-9): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "0" => return None,
            "1" => return Some("0"),
            "2" => return Some("15"),
            "3" => return Some("16"),
            "4" => return Some("17"),
            "5" => return Some("18"),
            "6" => return Some("19"),
            "7" => return Some("20"),
            "8" => return Some("21"),
            "9" => return Some("22"),
            _ => {
                println!("Invalid choice. Please select between 0 and 9.");
                continue;
            }
        }
    }
}

fn run_sorting(choice: &str, array_gen: &mut dyn FnMut() -> Vec<i32>) {
    let mut array = array_gen();
    match choice {
        "0" => bogosort::bogo_sort(&mut array),
        "15" => quantumbogosort::quantum_bogo_sort(&mut array),
        "16" => stalinsort::stalinsort(&mut array),
        "17" => elonsort::elon_sort(&mut array),
        "18" => intelligentdesignsort::intelligent_design_sort(&mut array),
        "19" => miraclesort::miracle_sort(&mut array),
        "20" => bozosort::bozo_sort(&mut array),
        "21" => slowsort::slowsort(&mut array),
        "22" => humansort::human_sort(&mut array),
        _ => unreachable!(),
    }
    visualizer::draw(&array);
    println!("Sorting complete. Press Enter to return to menu.");
    let mut tmp = String::new();
    io::stdin().read_line(&mut tmp).unwrap();
}

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