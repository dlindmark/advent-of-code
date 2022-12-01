use std::io;

use std::fs::File;
use std::io::{BufRead};
use std::path::Path;

fn main() {
    let mut max_elf: i32 = 0;
    let mut current_elf: i32 = 0;
    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;


    // File hosts must exist in current path before this procedure output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                // Check if new line
                if number.trim().is_empty() {
                    if current_calories > max_calories {
                        max_calories = current_calories;
                        max_elf = current_elf;
                    }
                    current_calories = 0;
                    current_elf += 1;
                }
                else{
                    let snack = number.trim().parse::<i32>().unwrap();
                    current_calories += snack;
                }
            }
        }
    }
    println!("The strongest elf is elf number {}. That elf was carrying {} calories", max_elf + 1, max_calories);
}

// The output is wrapped in a  Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}