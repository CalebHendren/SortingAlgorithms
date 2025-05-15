## Features
- Visualizes sorting algorithms as dynamic bar charts in the terminal.
- Easy to extend with additional sorting algorithms.

## Project Structure
- **src/visualizer.rs**: Contains functions to render the current state of the array.
- **src/shellsort.rs**: Implements the Shell Sort algorithm and integrates with the visualizer.
- Additional modules can be added for other sorting algorithms as needed.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (Built with Edition 2021)

### Installation
1. Clone the repository:
   ```
   git clone <repository_url>
   ```
2. Navigate to the project directory:
   ```
   cd SortingAlgorithms
   ```
3. Build the project using Cargo:
   ```
   cargo build --release
   ```

### Running the Project
Execute the following command to run the visualizer:
```
cargo run
```
